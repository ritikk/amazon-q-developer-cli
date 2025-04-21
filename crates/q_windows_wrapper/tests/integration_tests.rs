use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use tempfile::tempdir;

#[test]
fn test_wrapper_executable_not_found() {
    // Ensure the Q_CLI_PATH is not set
    env::remove_var("Q_CLI_PATH");

    // Temporarily modify PATH to ensure q_cli is not found
    let original_path = env::var_os("PATH").unwrap_or_default();
    env::set_var("PATH", "");

    // Run the wrapper
    let output = Command::new(env!("CARGO_BIN_EXE_q_windows_wrapper"))
        .output()
        .expect("Failed to execute wrapper");

    // Restore PATH
    env::set_var("PATH", original_path);

    // Check that the wrapper returns an error
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Q CLI executable not found"),
        "Expected error message about executable not found, got: {}",
        stderr
    );
}

#[test]
fn test_wrapper_with_custom_path() {
    // Create a temporary directory with a mock q_cli executable
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let mock_executable_path = temp_dir.path().join("q_cli.exe");

    // Create a simple batch script that echoes the arguments
    let mut file = File::create(&mock_executable_path).expect("Failed to create mock executable");
    file.write_all(b"@echo off\necho Arguments: %*\nexit /b 0")
        .expect("Failed to write to mock executable");

    // Set the custom path environment variable
    env::set_var("Q_CLI_PATH", mock_executable_path.to_str().unwrap());

    // Run the wrapper with some arguments
    let output = Command::new(env!("CARGO_BIN_EXE_q_windows_wrapper"))
        .args(["arg1", "arg2", "--flag", "value with spaces"])
        .output()
        .expect("Failed to execute wrapper");

    // Clean up
    env::remove_var("Q_CLI_PATH");

    // Check that the wrapper executed successfully
    assert!(output.status.success(), "Wrapper should exit successfully");

    // Check that the arguments were passed correctly
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Arguments: arg1 arg2 --flag \"value with spaces\""),
        "Arguments were not passed correctly: {}",
        stdout
    );
}

#[test]
fn test_wrapper_preserves_exit_code() {
    // Create a temporary directory with a mock q_cli executable
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let mock_executable_path = temp_dir.path().join("q_cli.exe");

    // Create a simple batch script that exits with the specified code
    let mut file = File::create(&mock_executable_path).expect("Failed to create mock executable");
    file.write_all(b"@echo off\nexit /b %1")
        .expect("Failed to write to mock executable");

    // Set the custom path environment variable
    env::set_var("Q_CLI_PATH", mock_executable_path.to_str().unwrap());

    // Test with exit code 0
    let output = Command::new(env!("CARGO_BIN_EXE_q_windows_wrapper"))
        .arg("0")
        .status()
        .expect("Failed to execute wrapper");
    assert!(output.success(), "Wrapper should exit successfully with code 0");

    // Test with exit code 1
    let output = Command::new(env!("CARGO_BIN_EXE_q_windows_wrapper"))
        .arg("1")
        .status()
        .expect("Failed to execute wrapper");
    assert!(!output.success(), "Wrapper should fail with exit code 1");
    assert_eq!(output.code(), Some(1), "Exit code should be 1");

    // Test with exit code 2
    let output = Command::new(env!("CARGO_BIN_EXE_q_windows_wrapper"))
        .arg("2")
        .status()
        .expect("Failed to execute wrapper");
    assert!(!output.success(), "Wrapper should fail with exit code 2");
    assert_eq!(output.code(), Some(2), "Exit code should be 2");

    // Clean up
    env::remove_var("Q_CLI_PATH");
}

#[test]
fn test_wrapper_handles_spaces_in_paths() {
    // Create a temporary directory with spaces in the name
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let dir_with_spaces = temp_dir.path().join("directory with spaces");
    std::fs::create_dir_all(&dir_with_spaces).expect("Failed to create directory with spaces");

    let mock_executable_path = dir_with_spaces.join("q_cli.exe");

    // Create a simple batch script that echoes the current directory
    let mut file = File::create(&mock_executable_path).expect("Failed to create mock executable");
    file.write_all(b"@echo off\necho Current directory: %CD%\nexit /b 0")
        .expect("Failed to write to mock executable");

    // Set the custom path environment variable
    env::set_var("Q_CLI_PATH", mock_executable_path.to_str().unwrap());

    // Run the wrapper
    let output = Command::new(env!("CARGO_BIN_EXE_q_windows_wrapper"))
        .output()
        .expect("Failed to execute wrapper");

    // Clean up
    env::remove_var("Q_CLI_PATH");

    // Check that the wrapper executed successfully
    assert!(output.status.success(), "Wrapper should exit successfully");

    // Check that the current directory was preserved
    let stdout = String::from_utf8_lossy(&output.stdout);
    let current_dir = env::current_dir().expect("Failed to get current directory");
    assert!(
        stdout.contains(&format!("Current directory: {}", current_dir.display())),
        "Current directory was not preserved: {}",
        stdout
    );
}
