//! AWS context detection module for gathering AWS environment information
//!
//! This module provides functionality to detect and display AWS context information
//! such as profile, region, account ID, and access key ID. It's designed to work
//! with the AWS CLI and gracefully handle cases where AWS CLI is not available
//! or credentials are not configured.

use std::process::Stdio;

use eyre::{
    Result,
    WrapErr,
};
use serde::{
    Deserialize,
    Serialize,
};
use tokio::process::Command;
use tracing::{
    debug,
    error,
    warn,
};

/// AWS context information gathered from the current environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsContext {
    /// AWS profile name (e.g., "default", "production")
    pub profile: String,

    /// AWS region (e.g., "us-west-2")
    pub region: String,

    /// AWS account ID (12-digit number)
    pub account_id: Option<String>,
}

impl AwsContext {
    /// Detects AWS context information from the current environment
    ///
    /// # Arguments
    /// * `profile` - Optional AWS profile name, defaults to "default"
    /// * `region` - AWS region to use
    ///
    /// # Returns
    /// Returns an AwsContext with available information. Missing information
    /// is represented as None rather than causing the function to fail.
    pub async fn detect(profile: Option<&str>, region: &str) -> Result<Self> {
        let profile_name = profile.unwrap_or("default").to_string();

        debug!(
            "Starting AWS context detection for profile: {}, region: {}",
            profile_name, region
        );

        let mut context = AwsContext {
            profile: profile_name.clone(),
            region: region.to_string(),
            account_id: None,
        };

        // Attempt to get account ID using AWS STS
        match get_account_id(&profile_name).await {
            Ok(account_id) => {
                debug!("Successfully retrieved AWS account ID: {}", account_id);
                context.account_id = Some(account_id);
            },
            Err(e) => {
                warn!(
                    "Failed to retrieve AWS account ID for profile '{}': {}",
                    profile_name, e
                );
                debug!("Account ID detection error details: {:?}", e);
            },
        }

        debug!(
            "AWS context detection completed. Account ID: {}",
            context.account_id.is_some()
        );

        Ok(context)
    }

    /// Formats the AWS context information for display to the user
    pub fn format_for_display(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("AWS Profile: {}\n", self.profile));
        output.push_str(&format!("AWS Region: {}\n", self.region));

        match &self.account_id {
            Some(account_id) => output.push_str(&format!("AWS Account ID: {}", account_id)),
            None => output.push_str("AWS Account ID: Unable to determine (check AWS CLI configuration)"),
        }

        output
    }

    /// Returns a detailed error message for context detection failures
    pub fn format_error_message(profile: &str, region: &str, error: &eyre::Error) -> String {
        format!(
            "Failed to gather AWS context information:\n\
             Profile: {}\n\
             Region: {}\n\
             Error: {}\n\n\
             This may indicate:\n\
             • AWS CLI is not installed or not in PATH\n\
             • AWS credentials are not configured for this profile\n\
             • Network connectivity issues\n\
             • Insufficient permissions for AWS STS operations\n\n\
             You can still proceed with the operation, but please verify your AWS configuration manually.",
            profile, region, error
        )
    }
}

/// Retrieves the AWS account ID using AWS STS get-caller-identity
async fn get_account_id(profile: &str) -> Result<String> {
    debug!("Attempting to retrieve AWS account ID for profile: {}", profile);

    // Check if AWS CLI is available
    if !is_aws_cli_available().await {
        return Err(eyre::eyre!("AWS CLI is not available in PATH"));
    }

    let mut command = Command::new("aws");
    command
        .arg("sts")
        .arg("get-caller-identity")
        .arg("--query")
        .arg("Account")
        .arg("--output")
        .arg("text");

    // Add profile if it's not the default
    if profile != "default" {
        command.arg("--profile").arg(profile);
    }

    debug!(
        "Executing AWS CLI command: aws sts get-caller-identity --query Account --output text{}",
        if profile != "default" {
            format!(" --profile {}", profile)
        } else {
            String::new()
        }
    );

    let output = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .wrap_err("Failed to spawn AWS CLI command for account ID")?
        .wait_with_output()
        .await
        .wrap_err("Failed to execute AWS CLI command for account ID")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let exit_code = output.status.code().unwrap_or(-1);
        error!("AWS CLI command failed with exit code {}: {}", exit_code, stderr);

        // Provide more specific error messages based on common failure scenarios
        let error_message = if stderr.contains("Unable to locate credentials") {
            format!(
                "AWS credentials not found for profile '{}'. Please configure your AWS credentials.",
                profile
            )
        } else if stderr.contains("The config profile") && stderr.contains("could not be found") {
            format!(
                "AWS profile '{}' not found. Please check your AWS configuration.",
                profile
            )
        } else if stderr.contains("No such file or directory") {
            "AWS CLI is not installed or not in PATH".to_string()
        } else if stderr.contains("ExpiredToken") {
            "AWS credentials have expired. Please refresh your credentials.".to_string()
        } else if stderr.contains("AccessDenied") {
            "Access denied. Please check your AWS permissions.".to_string()
        } else {
            format!("AWS CLI command failed: {}", stderr)
        };

        return Err(eyre::eyre!("{}", error_message));
    }

    let account_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if account_id.is_empty() {
        warn!("AWS CLI returned empty account ID for profile: {}", profile);
        return Err(eyre::eyre!("AWS CLI returned empty account ID"));
    }

    // Validate account ID format (should be 12 digits)
    if !account_id.chars().all(|c| c.is_ascii_digit()) || account_id.len() != 12 {
        warn!("AWS CLI returned invalid account ID format: {}", account_id);
        return Err(eyre::eyre!(
            "AWS CLI returned invalid account ID format: {}",
            account_id
        ));
    }

    debug!("Successfully retrieved AWS account ID: {}", account_id);
    Ok(account_id)
}

/// Checks if AWS CLI is available in the system PATH
async fn is_aws_cli_available() -> bool {
    debug!("Checking if AWS CLI is available");

    let result = Command::new("aws")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match result {
        Ok(mut child) => match child.wait().await {
            Ok(status) => {
                let available = status.success();
                debug!(
                    "AWS CLI availability check: {}",
                    if available { "available" } else { "not available" }
                );
                available
            },
            Err(e) => {
                debug!("AWS CLI availability check failed during wait: {}", e);
                false
            },
        },
        Err(e) => {
            debug!("AWS CLI availability check failed during spawn: {}", e);
            false
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_aws_context_creation() {
        let context = AwsContext {
            profile: "test-profile".to_string(),
            region: "us-west-2".to_string(),
            account_id: Some("123456789012".to_string()),
        };

        assert_eq!(context.profile, "test-profile");
        assert_eq!(context.region, "us-west-2");
        assert_eq!(context.account_id, Some("123456789012".to_string()));
    }

    #[tokio::test]
    async fn test_format_for_display_with_all_info() {
        let context = AwsContext {
            profile: "production".to_string(),
            region: "us-east-1".to_string(),
            account_id: Some("123456789012".to_string()),
        };

        let formatted = context.format_for_display();
        assert!(formatted.contains("AWS Profile: production"));
        assert!(formatted.contains("AWS Region: us-east-1"));
        assert!(formatted.contains("AWS Account ID: 123456789012"));
    }

    #[tokio::test]
    async fn test_format_for_display_with_missing_info() {
        let context = AwsContext {
            profile: "test".to_string(),
            region: "us-west-2".to_string(),
            account_id: None,
        };

        let formatted = context.format_for_display();
        assert!(formatted.contains("AWS Profile: test"));
        assert!(formatted.contains("AWS Region: us-west-2"));
        assert!(formatted.contains("AWS Account ID: Unable to determine"));
    }
}
