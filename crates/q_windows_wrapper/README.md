# Q Windows Wrapper

A Windows wrapper for the Amazon Q CLI that ensures proper execution in the Windows environment.

## Features

- **Error Handling**:
  - Checks if the Q CLI exists before trying to execute it
  - Handles any execution errors gracefully
  - Provides meaningful error messages

- **Path Management**:
  - Uses the correct path to the Q CLI executable
  - Handles spaces in paths correctly
  - Supports both default and custom installation locations

- **Arguments**:
  - Passes through all command line arguments to the Q CLI
  - Preserves argument order and quotes
  - Handles special characters correctly

- **Environment**:
  - Preserves environment variables
  - Handles working directory correctly
  - Maintains exit codes

## Installation Locations

The wrapper searches for the Q CLI executable in the following locations:

1. In the system PATH
2. Default installation locations:
   - `%ProgramFiles%\Amazon\Q\bin\q_cli.exe`
   - `%ProgramFiles(x86)%\Amazon\Q\bin\q_cli.exe`
   - `%LOCALAPPDATA%\Amazon\Q\bin\q_cli.exe`
3. Custom location specified by the `Q_CLI_PATH` environment variable

## Usage

Simply use `q_windows_wrapper` instead of `q_cli` in your commands:

```
q_windows_wrapper [arguments]
```

All arguments will be passed through to the Q CLI executable.

## Custom Installation Location

If the Q CLI is installed in a non-standard location, you can set the `Q_CLI_PATH` environment variable to point to the executable:

```
set Q_CLI_PATH=C:\path\to\q_cli.exe
q_windows_wrapper [arguments]
```

## Error Messages

- **Q CLI executable not found**: The wrapper could not find the Q CLI executable in any of the searched locations. Make sure it is installed and in your PATH, or set the `Q_CLI_PATH` environment variable.
- **Failed to execute Q CLI**: There was an error executing the Q CLI executable. Check the error message for details.
- **Invalid path**: The path to the Q CLI executable is invalid. Check the `Q_CLI_PATH` environment variable if set.

## Exit Codes

The wrapper preserves the exit code from the Q CLI executable. If the wrapper itself encounters an error before executing the Q CLI, it will exit with code 1.