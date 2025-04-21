use std::env;
use std::path::{
    Path,
    PathBuf,
};
use std::process::{
    Command,
    ExitCode,
    Stdio,
};

use eyre::Result;
use thiserror::Error;
use tracing::{
    debug,
    error,
    info,
};
use tracing_subscriber::fmt::format::FmtSpan;
use which::which;

/// Custom error types for the q_windows_wrapper
#[derive(Error, Debug)]
enum WrapperError {
    #[error(
        "Q CLI executable not found. Please ensure it is installed and in your PATH or set Q_CLI_PATH environment variable."
    )]
    ExecutableNotFound,

    #[error("Failed to execute Q CLI: {0}")]
    ExecutionError(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Failed to initialize logging: {0}")]
    LoggingError(String),
}

/// Possible locations where the Q CLI executable might be installed
fn get_possible_q_cli_locations() -> Vec<PathBuf> {
    let mut locations = Vec::new();

    // Add the PATH environment variable locations
    if let Ok(exe_path) = which("q_cli") {
        locations.push(exe_path);
    }
    if let Ok(exe_path) = which("q_cli.exe") {
        locations.push(exe_path);
    }

    // Add default installation locations
    if let Some(program_files) = env::var_os("ProgramFiles") {
        let program_files_path = Path::new(&program_files);
        locations.push(
            program_files_path
                .join("Amazon")
                .join("Q")
                .join("bin")
                .join("q_cli.exe"),
        );
    }

    if let Some(program_files_x86) = env::var_os("ProgramFiles(x86)") {
        let program_files_x86_path = Path::new(&program_files_x86);
        locations.push(
            program_files_x86_path
                .join("Amazon")
                .join("Q")
                .join("bin")
                .join("q_cli.exe"),
        );
    }

    // Add user-specific installation locations
    if let Some(local_app_data) = env::var_os("LOCALAPPDATA") {
        let local_app_data_path = Path::new(&local_app_data);
        locations.push(
            local_app_data_path
                .join("Amazon")
                .join("Q")
                .join("bin")
                .join("q_cli.exe"),
        );
    }

    // Add custom location from environment variable if set
    if let Some(custom_path) = env::var_os("Q_CLI_PATH") {
        locations.push(Path::new(&custom_path).to_path_buf());
    }

    locations
}

/// Find the Q CLI executable
fn find_q_cli_executable() -> Result<PathBuf, WrapperError> {
    let locations = get_possible_q_cli_locations();

    debug!("Searching for Q CLI executable in the following locations:");
    for location in &locations {
        debug!("  - {}", location.display());
    }

    for location in locations {
        if location.exists() && location.is_file() {
            info!("Found Q CLI executable at: {}", location.display());
            return Ok(location);
        }
    }

    error!("Q CLI executable not found in any of the searched locations");
    Err(WrapperError::ExecutableNotFound)
}

/// Execute the Q CLI with the given arguments
fn execute_q_cli(executable_path: &Path, args: &[String]) -> Result<ExitCode, WrapperError> {
    let executable_path_str = executable_path.to_string_lossy();

    debug!("Executing Q CLI: {} {}", executable_path_str, args.join(" "));

    // Create the command with proper Windows path handling
    let mut command = Command::new(executable_path);

    // Add all arguments, preserving quotes and special characters
    command.args(args);

    // Inherit stdio to ensure proper handling of interactive prompts
    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    // Preserve the current working directory
    let current_dir = env::current_dir()
        .map_err(|e| WrapperError::ExecutionError(format!("Failed to get current directory: {}", e)))?;
    command.current_dir(current_dir);

    // Preserve environment variables
    // (Command inherits environment variables by default)

    // Execute the command
    let status = command
        .status()
        .map_err(|e| WrapperError::ExecutionError(format!("Failed to execute Q CLI: {}", e)))?;

    // Get the exit code
    let exit_code = status.code().unwrap_or(1);
    debug!("Q CLI exited with code: {}", exit_code);

    Ok(ExitCode::from(exit_code as u8))
}

/// Initialize logging
fn init_logging() -> Result<(), WrapperError> {
    // Check if verbose logging is requested
    let log_level = match env::var("Q_WRAPPER_VERBOSE").ok().as_deref() {
        Some("1" | "true" | "yes") => "debug",
        Some("2" | "trace") => "trace",
        _ => "info",
    };

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .with_span_events(FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).map_err(|e| WrapperError::LoggingError(e.to_string()))?;

    Ok(())
}

/// Check if the executable path is valid
fn validate_executable_path(path: &Path) -> Result<(), WrapperError> {
    if !path.exists() {
        return Err(WrapperError::InvalidPath(format!(
            "Path does not exist: {}",
            path.display()
        )));
    }

    if !path.is_file() {
        return Err(WrapperError::InvalidPath(format!(
            "Path is not a file: {}",
            path.display()
        )));
    }

    Ok(())
}

fn main() -> ExitCode {
    // Initialize logging
    if let Err(e) = init_logging() {
        eprintln!("Warning: Failed to initialize logging: {}", e);
    }

    // Get command line arguments (skip the first one, which is the program name)
    let args: Vec<String> = env::args().skip(1).collect();

    debug!("Command line arguments: {:?}", args);

    // Find the Q CLI executable
    let executable_path = match find_q_cli_executable() {
        Ok(path) => {
            // Validate the executable path
            if let Err(e) = validate_executable_path(&path) {
                error!("Error: {}", e);
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
            path
        },
        Err(e) => {
            error!("Error: {}", e);
            eprintln!("Error: {}", e);
            return ExitCode::FAILURE;
        },
    };

    // Execute the Q CLI with the given arguments
    match execute_q_cli(&executable_path, &args) {
        Ok(exit_code) => exit_code,
        Err(e) => {
            error!("Error: {}", e);
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        },
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_get_possible_q_cli_locations() {
        let locations = get_possible_q_cli_locations();
        assert!(!locations.is_empty(), "Should return at least some possible locations");
    }

    #[test]
    fn test_find_q_cli_executable_not_found() {
        // Temporarily modify PATH to ensure q_cli is not found
        let original_path = env::var_os("PATH").unwrap_or_default();
        env::set_var("PATH", "");

        // Also unset any custom path
        env::remove_var("Q_CLI_PATH");

        let result = find_q_cli_executable();

        // Restore PATH
        env::set_var("PATH", original_path);

        assert!(result.is_err(), "Should return an error when executable is not found");
    }

    #[test]
    fn test_find_q_cli_executable_custom_path() {
        // Create a temporary directory with a mock q_cli executable
        let temp_dir = tempdir().unwrap();
        let mock_executable_path = temp_dir.path().join("q_cli.exe");

        // Create an empty file to simulate the executable
        let mut file = File::create(&mock_executable_path).unwrap();
        file.write_all(b"mock executable").unwrap();

        // Set the custom path environment variable
        env::set_var("Q_CLI_PATH", mock_executable_path.to_str().unwrap());

        let result = find_q_cli_executable();

        // Clean up
        env::remove_var("Q_CLI_PATH");

        assert!(result.is_ok(), "Should find the executable at the custom path");
        assert_eq!(result.unwrap(), mock_executable_path, "Should return the correct path");
    }

    #[test]
    fn test_validate_executable_path() {
        // Test with a non-existent path
        let non_existent_path = PathBuf::from("non_existent_file.exe");
        let result = validate_executable_path(&non_existent_path);
        assert!(result.is_err(), "Should return an error for non-existent path");

        // Test with a directory
        let temp_dir = tempdir().unwrap();
        let result = validate_executable_path(temp_dir.path());
        assert!(result.is_err(), "Should return an error for a directory");

        // Test with a valid file
        let valid_file_path = temp_dir.path().join("valid_file.exe");
        let mut file = File::create(&valid_file_path).unwrap();
        file.write_all(b"valid file").unwrap();
        let result = validate_executable_path(&valid_file_path);
        assert!(result.is_ok(), "Should return Ok for a valid file");
    }
}
