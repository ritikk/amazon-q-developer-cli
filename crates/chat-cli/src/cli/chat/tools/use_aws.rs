use std::collections::HashMap;
use std::io::Write;
use std::process::Stdio;

use convert_case::{
    Case,
    Casing,
};
use crossterm::{
    queue,
    style,
};
use eyre::{
    Result,
    WrapErr,
};
use serde::Deserialize;
use tracing::{
    debug,
    error,
};

use super::aws_context::AwsContext;
use super::{
    InvokeOutput,
    MAX_TOOL_RESPONSE_SIZE,
    OutputKind,
};
use crate::cli::agent::{
    Agent,
    PermissionEvalResult,
};
use crate::os::Os;

const READONLY_OPS: [&str; 6] = ["get", "describe", "list", "ls", "search", "batch_get"];

/// The environment variable name where we set additional metadata for the AWS CLI user agent.
const USER_AGENT_ENV_VAR: &str = "AWS_EXECUTION_ENV";
const USER_AGENT_APP_NAME: &str = "AmazonQ-For-CLI";
const USER_AGENT_VERSION_KEY: &str = "Version";
const USER_AGENT_VERSION_VALUE: &str = env!("CARGO_PKG_VERSION");

// TODO: we should perhaps composite this struct with an interface that we can use to mock the
// actual cli with. That will allow us to more thoroughly test it.
#[derive(Debug, Clone, Deserialize)]
pub struct UseAws {
    pub service_name: String,
    pub operation_name: String,
    pub parameters: Option<HashMap<String, serde_json::Value>>,
    pub region: String,
    pub profile_name: Option<String>,
    pub label: Option<String>,
}

impl UseAws {
    /// Check if double-check confirmation is enabled for AWS actions
    pub fn is_actions_double_check_enabled(os: &Os) -> bool {
        use crate::database::settings::Setting;
        os.database
            .settings
            .get_bool(Setting::AwsActionsDoubleCheckEnabled)
            .unwrap_or(false)
    }

    /// Check if double-check confirmation is enabled for AWS actions (test version)
    #[cfg(test)]
    pub fn is_actions_double_check_enabled_with_settings(settings: &crate::database::settings::Settings) -> bool {
        use crate::database::settings::Setting;
        settings
            .get_bool(Setting::AwsActionsDoubleCheckEnabled)
            .unwrap_or(false)
    }

    pub fn requires_acceptance(&self) -> bool {
        // Readonly operations never require acceptance, regardless of double-check setting
        if READONLY_OPS.iter().any(|op| self.operation_name.starts_with(op)) {
            return false;
        }

        // Non-readonly operations always require acceptance
        true
    }

    pub async fn invoke(&self, os: &Os, mut updates: impl Write) -> Result<InvokeOutput> {
        debug!(
            "Invoking AWS tool: service={}, operation={}, profile={:?}, region={}",
            self.service_name, self.operation_name, self.profile_name, self.region
        );

        // Check if this is a readonly operation - if so, execute directly without any confirmation
        if READONLY_OPS.iter().any(|op| self.operation_name.starts_with(op)) {
            debug!("Operation '{}' is readonly, executing directly", self.operation_name);
            return self.execute_aws_command().await;
        }

        // For non-readonly operations, check if actions double-check is enabled
        let actions_double_check_enabled = Self::is_actions_double_check_enabled(os);
        debug!("AWS actions double-check enabled: {}", actions_double_check_enabled);

        if !actions_double_check_enabled {
            // Use existing single confirmation flow (handled by the tool framework)
            debug!("AWS actions double-check disabled, using single confirmation flow");
            return self.execute_aws_command().await;
        }

        // AWS actions double-check flow: first confirmation is handled by the tool framework
        // We need to implement the second confirmation with AWS context display
        debug!("Starting AWS actions double-check flow");

        // Gather and display AWS context information
        match self.display_aws_context(os, &mut updates).await {
            Ok(_) => debug!("Successfully displayed AWS context"),
            Err(e) => {
                error!("Failed to display AWS context: {:?}", e);
                queue!(updates, style::Print("⚠️  Error displaying AWS context information.\n"))?;
                queue!(
                    updates,
                    style::Print("This may indicate AWS CLI configuration issues.\n")
                )?;
                queue!(
                    updates,
                    style::Print("Please verify your AWS setup before proceeding.\n\n")
                )?;
                updates.flush()?;
                // Continue with confirmation despite context display error
            },
        }

        // Prompt for second confirmation
        match self.prompt_actions_double_check_confirmation(os, &mut updates).await {
            Ok(confirmed) => {
                if confirmed {
                    debug!("User confirmed AWS actions double-check, executing AWS command");
                    self.execute_aws_command().await
                } else {
                    debug!("User cancelled operation at AWS actions double-check prompt");
                    queue!(updates, style::Print("\n📋 Operation Summary:\n"))?;
                    queue!(updates, style::Print("  Status: Cancelled by user at security check\n"))?;
                    queue!(updates, style::Print("  Reason: User declined second confirmation\n"))?;
                    queue!(updates, style::Print("  Action: No AWS resources were modified\n\n"))?;
                    updates.flush()?;
                    Err(eyre::eyre!(
                        "Operation cancelled by user at AWS actions double-check confirmation"
                    ))
                }
            },
            Err(e) => {
                error!("Error during AWS actions double-check confirmation: {:?}", e);
                queue!(updates, style::Print("\n❌ Error during confirmation process.\n"))?;
                queue!(updates, style::Print("Operation cancelled for safety.\n\n"))?;
                updates.flush()?;
                Err(eyre::eyre!("Operation cancelled due to confirmation error: {}", e))
            },
        }
    }

    pub fn queue_description(&self, output: &mut impl Write) -> Result<()> {
        queue!(
            output,
            style::Print("Running aws cli command:\n\n"),
            style::Print(format!("Service name: {}\n", self.service_name)),
            style::Print(format!("Operation name: {}\n", self.operation_name)),
        )?;
        if let Some(parameters) = &self.parameters {
            queue!(output, style::Print("Parameters: \n".to_string()))?;
            for (name, value) in parameters {
                match value {
                    serde_json::Value::String(s) if s.is_empty() => {
                        queue!(output, style::Print(format!("- {}\n", name)))?;
                    },
                    _ => {
                        queue!(output, style::Print(format!("- {}: {}\n", name, value)))?;
                    },
                }
            }
        }

        if let Some(ref profile_name) = self.profile_name {
            queue!(output, style::Print(format!("Profile name: {}\n", profile_name)))?;
        } else {
            queue!(output, style::Print("Profile name: default\n".to_string()))?;
        }

        queue!(output, style::Print(format!("Region: {}", self.region)))?;

        if let Some(ref label) = self.label {
            queue!(output, style::Print(format!("\nLabel: {}", label)))?;
        }
        Ok(())
    }

    pub async fn validate(&mut self, _os: &Os) -> Result<()> {
        Ok(())
    }

    /// Displays AWS context information to the user before the second confirmation
    async fn display_aws_context(&self, _os: &Os, updates: &mut impl Write) -> Result<()> {
        debug!(
            "Displaying AWS context for profile: {:?}, region: {}",
            self.profile_name, self.region
        );

        queue!(updates, style::Print("\n"))?;
        queue!(updates, style::Print("=== AWS Context Information ===\n"))?;

        // Attempt to gather AWS context
        match AwsContext::detect(self.profile_name.as_deref(), &self.region).await {
            Ok(aws_context) => {
                debug!("Successfully gathered AWS context information");
                let formatted_context = aws_context.format_for_display();
                queue!(updates, style::Print(formatted_context))?;
            },
            Err(e) => {
                error!("Failed to gather AWS context information: {:?}", e);

                // Display user-friendly error message
                queue!(
                    updates,
                    style::Print("⚠️  Warning: Unable to gather complete AWS context information.\n\n")
                )?;

                // Show the detailed error message
                let error_message = AwsContext::format_error_message(
                    self.profile_name.as_deref().unwrap_or("default"),
                    &self.region,
                    &e,
                );
                queue!(updates, style::Print(error_message))?;
                queue!(updates, style::Print("\n\n"))?;

                // Show available information
                queue!(updates, style::Print("Available information from tool parameters:\n"))?;
                queue!(
                    updates,
                    style::Print(format!(
                        "AWS Profile: {}\n",
                        self.profile_name.as_deref().unwrap_or("default")
                    ))
                )?;
                queue!(updates, style::Print(format!("AWS Region: {}\n", self.region)))?;
                queue!(
                    updates,
                    style::Print("AWS Account ID: Unable to determine (see error above)\n")
                )?;
            },
        }

        queue!(updates, style::Print("================================\n"))?;
        updates.flush()?;
        Ok(())
    }

    /// Prompts the user for the second confirmation with AWS context details
    async fn prompt_actions_double_check_confirmation(&self, _os: &Os, updates: &mut impl Write) -> Result<bool> {
        debug!("Prompting user for AWS actions double-check confirmation");

        queue!(updates, style::Print("\n"))?;
        queue!(updates, style::Print("⚠️  ADDITIONAL SECURITY CHECK ⚠️\n"))?;
        queue!(
            updates,
            style::Print("This is a second confirmation to prevent accidental AWS operations.\n")
        )?;
        queue!(
            updates,
            style::Print("Please review the AWS context information above carefully.\n\n")
        )?;

        // Show operation details again for clarity
        queue!(updates, style::Print("Operation to be performed:\n"))?;
        queue!(updates, style::Print(format!("  Service: {}\n", self.service_name)))?;
        queue!(updates, style::Print(format!("  Operation: {}\n", self.operation_name)))?;
        if let Some(ref label) = self.label {
            queue!(updates, style::Print(format!("  Label: {}\n", label)))?;
        }
        queue!(updates, style::Print("\n"))?;

        queue!(
            updates,
            style::Print("Do you want to proceed with this AWS operation? (y/N): ")
        )?;
        updates.flush()?;

        // Read user input with error handling
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_lowercase();
                let confirmed = matches!(input.as_str(), "y" | "yes");

                if confirmed {
                    debug!("User confirmed the AWS actions double-check prompt");
                    queue!(
                        updates,
                        style::Print("✓ Second confirmation granted. Proceeding with operation...\n")
                    )?;
                } else {
                    debug!("User denied the AWS actions double-check prompt");
                    queue!(
                        updates,
                        style::Print("✗ Second confirmation denied. Operation cancelled.\n")
                    )?;
                }

                updates.flush()?;
                Ok(confirmed)
            },
            Err(e) => {
                error!(
                    "Failed to read user input for AWS actions double-check confirmation: {}",
                    e
                );
                queue!(
                    updates,
                    style::Print("\n✗ Error reading user input. Operation cancelled for safety.\n")
                )?;
                updates.flush()?;
                Ok(false)
            },
        }
    }

    /// Executes the AWS CLI command
    async fn execute_aws_command(&self) -> Result<InvokeOutput> {
        let mut command = tokio::process::Command::new("aws");

        // Add service name
        command.arg(&self.service_name);

        // Add operation name
        command.arg(&self.operation_name);

        // Add parameters
        if self.parameters.is_some() {
            let cli_params = self.cli_parameters()?;
            for (param_key, param_value) in cli_params {
                command.arg(param_key).arg(param_value);
            }
        }

        // Add region
        command.arg("--region").arg(&self.region);

        // Add profile if specified
        if let Some(profile) = &self.profile_name {
            command.arg("--profile").arg(profile);
        }

        // Add output format
        command.arg("--output").arg("json");

        // Set user agent environment variable
        command.env(
            USER_AGENT_ENV_VAR,
            format!(
                "{}_{}/{}",
                USER_AGENT_APP_NAME, USER_AGENT_VERSION_KEY, USER_AGENT_VERSION_VALUE
            ),
        );

        // Execute the command
        let output = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .wrap_err("Failed to spawn AWS CLI command")?
            .wait_with_output()
            .await
            .wrap_err("Failed to execute AWS CLI command")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let exit_code = output.status.code().unwrap_or(-1);

        // Truncate output if it's too large
        let truncated_stdout = if stdout.len() > MAX_TOOL_RESPONSE_SIZE {
            format!("{}... [truncated]", &stdout[..MAX_TOOL_RESPONSE_SIZE])
        } else {
            stdout.to_string()
        };

        let truncated_stderr = if stderr.len() > MAX_TOOL_RESPONSE_SIZE {
            format!("{}... [truncated]", &stderr[..MAX_TOOL_RESPONSE_SIZE])
        } else {
            stderr.to_string()
        };

        let result = serde_json::json!({
            "stdout": truncated_stdout,
            "stderr": truncated_stderr,
            "exit_status": exit_code
        });

        Ok(InvokeOutput {
            output: OutputKind::Json(result),
        })
    }

    /// Converts parameters to CLI format
    fn cli_parameters(&self) -> Result<Vec<(String, String)>> {
        let mut params = Vec::new();

        if let Some(parameters) = &self.parameters {
            for (key, value) in parameters {
                let cli_key = format!("--{}", key.to_case(Case::Kebab));
                let cli_value = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Number(n) => n.to_string(),
                    _ => value.to_string(),
                };
                params.push((cli_key, cli_value));
            }
        }

        Ok(params)
    }

    pub fn eval_perm(&self, agent: &Agent) -> PermissionEvalResult {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Settings {
            allowed_services: Vec<String>,
            denied_services: Vec<String>,
        }

        let Self { service_name, .. } = self;
        let is_in_allowlist = agent.allowed_tools.contains("use_aws");
        match agent.tools_settings.get("use_aws") {
            Some(settings) if is_in_allowlist => {
                let settings = match serde_json::from_value::<Settings>(settings.clone()) {
                    Ok(settings) => settings,
                    Err(e) => {
                        error!("Failed to deserialize tool settings for use_aws: {:?}", e);
                        return PermissionEvalResult::Ask;
                    },
                };
                if settings.denied_services.contains(service_name) {
                    return PermissionEvalResult::Deny;
                }
                if settings.allowed_services.contains(service_name) {
                    return PermissionEvalResult::Allow;
                }
                PermissionEvalResult::Ask
            },
            None if is_in_allowlist => PermissionEvalResult::Allow,
            _ => {
                if self.requires_acceptance() {
                    PermissionEvalResult::Ask
                } else {
                    PermissionEvalResult::Allow
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! use_aws {
        ($value:tt) => {
            serde_json::from_value::<UseAws>(serde_json::json!($value)).unwrap()
        };
    }

    #[test]
    fn test_requires_acceptance() {
        let cmd = use_aws! {{
            "service_name": "ecs",
            "operation_name": "list-task-definitions",
            "region": "us-west-2",
            "profile_name": "default",
            "label": ""
        }};
        assert!(!cmd.requires_acceptance());
        let cmd = use_aws! {{
            "service_name": "lambda",
            "operation_name": "list-functions",
            "region": "us-west-2",
            "profile_name": "default",
            "label": ""
        }};
        assert!(!cmd.requires_acceptance());
        let cmd = use_aws! {{
            "service_name": "s3",
            "operation_name": "put-object",
            "region": "us-west-2",
            "profile_name": "default",
            "label": ""
        }};
        assert!(cmd.requires_acceptance());
    }

    #[test]
    fn test_use_aws_deser() {
        let cmd = use_aws! {{
            "service_name": "s3",
            "operation_name": "put-object",
            "parameters": {
                "TableName": "table-name",
                "KeyConditionExpression": "PartitionKey = :pkValue"
            },
            "region": "us-west-2",
            "profile_name": "default",
            "label": ""
        }};
        let params = cmd.cli_parameters().unwrap();
        assert!(
            params.iter().any(|p| p.0 == "--table-name" && p.1 == "table-name"),
            "not found in {:?}",
            params
        );
        assert!(
            params
                .iter()
                .any(|p| p.0 == "--key-condition-expression" && p.1 == "PartitionKey = :pkValue"),
            "not found in {:?}",
            params
        );
    }

    #[tokio::test]
    #[ignore = "not in ci"]
    async fn test_aws_read_only() {
        let os = Os::new().await.unwrap();

        let v = serde_json::json!({
            "service_name": "s3",
            "operation_name": "put-object",
            // technically this wouldn't be a valid request with an empty parameter set but it's
            // okay for this test
            "parameters": {},
            "region": "us-west-2",
            "profile_name": "default",
            "label": ""
        });

        assert!(
            serde_json::from_value::<UseAws>(v)
                .unwrap()
                .invoke(&os, &mut std::io::stdout())
                .await
                .is_err()
        );
    }

    #[test]
    fn test_requires_acceptance_with_actions_double_check_disabled() {
        // Test default behavior (actions double-check disabled)
        // We need to test this indirectly since we can't easily mock the static method

        // Read-only operations should not require acceptance
        let readonly_cmd = use_aws! {{
            "service_name": "ecs",
            "operation_name": "list-task-definitions",
            "region": "us-west-2",
            "profile_name": "default",
            "label": ""
        }};

        let get_cmd = use_aws! {{
            "service_name": "s3",
            "operation_name": "get-object",
            "region": "us-west-2",
            "profile_name": "default",
            "label": ""
        }};

        let describe_cmd = use_aws! {{
            "service_name": "ec2",
            "operation_name": "describe-instances",
            "region": "us-west-2",
            "profile_name": "default",
            "label": ""
        }};

        // Write operations should require acceptance
        let write_cmd = use_aws! {{
            "service_name": "s3",
            "operation_name": "put-object",
            "region": "us-west-2",
            "profile_name": "default",
            "label": ""
        }};

        let delete_cmd = use_aws! {{
            "service_name": "s3",
            "operation_name": "delete-object",
            "region": "us-west-2",
            "profile_name": "default",
            "label": ""
        }};

        // Test the logic without the settings dependency
        // This tests the READONLY_OPS logic
        assert!(
            !READONLY_OPS
                .iter()
                .any(|op| readonly_cmd.operation_name.starts_with(op))
                == false
        );
        assert!(!READONLY_OPS.iter().any(|op| get_cmd.operation_name.starts_with(op)) == false);
        assert!(
            !READONLY_OPS
                .iter()
                .any(|op| describe_cmd.operation_name.starts_with(op))
                == false
        );
        assert!(!READONLY_OPS.iter().any(|op| write_cmd.operation_name.starts_with(op)) == true);
        assert!(!READONLY_OPS.iter().any(|op| delete_cmd.operation_name.starts_with(op)) == true);
    }

    #[tokio::test]
    async fn test_display_aws_context() {
        let use_aws = use_aws! {{
            "service_name": "s3",
            "operation_name": "put-object",
            "region": "us-west-2",
            "profile_name": "test-profile",
            "label": ""
        }};

        let os = Os::new().await.unwrap();
        let mut output = Vec::new();

        // This should not fail even if AWS CLI is not available
        let result = use_aws.display_aws_context(&os, &mut output).await;
        assert!(result.is_ok());

        let output_str = String::from_utf8_lossy(&output);
        assert!(output_str.contains("=== AWS Context Information ==="));
        assert!(output_str.contains("AWS Profile: test-profile"));
        assert!(output_str.contains("AWS Region: us-west-2"));
        // Account ID may show "Unable to determine" if AWS CLI fails
        assert!(output_str.contains("AWS Account ID:"));
    }

    #[tokio::test]
    #[ignore = "not in ci"]
    async fn test_aws_output() {
        let os = Os::new().await.unwrap();

        let v = serde_json::json!({
            "service_name": "s3",
            "operation_name": "ls",
            "parameters": {},
            "region": "us-west-2",
            "profile_name": "default",
            "label": ""
        });
        let out = serde_json::from_value::<UseAws>(v)
            .unwrap()
            .invoke(&os, &mut std::io::stdout())
            .await
            .unwrap();

        if let OutputKind::Json(json) = out.output {
            // depending on where the test is ran we might get different outcome here but it does
            // not mean the tool is not working
            let exit_status = json.get("exit_status").unwrap();
            if exit_status == 0 {
                assert_eq!(json.get("stderr").unwrap(), "");
            } else {
                assert_ne!(json.get("stderr").unwrap(), "");
            }
        } else {
            panic!("Expected JSON output");
        }
    }
}
