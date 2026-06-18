# which

A Rust implementation of the which command for Windows — locates the executable file associated with a given command by searching the PATH environment variable.

Inspired by the GNU which utility.

## Features

- Searches for executables in directories listed in the PATH environment variable.
- Uses the PATHEXT environment variable to try common executable extensions (.exe, .bat, .cmd, etc.) when the command name has no extension.
- Falls back to a built-in default extension list if PATHEXT is not set.
- Supports direct path searching when the command contains \ or /.
- Returns exit code 1 if any command is not found (consistent with GNU which).

## Usage

`ash
# Locate a command
which notepad

# Locate multiple commands at once
which git cargo python

# Check a command with an explicit extension
which node.exe

# Use a direct path
which .\myapp.exe

# Show help
which --help
# or
which -h
`

### Exit Codes

| Code | Meaning                              |
|------|--------------------------------------|
| 0    | All specified commands were found    |
| 1    | One or more commands were not found  |

## Building

`ash
# Build in release mode
cargo build --release

# The binary will be at target/release/which.exe
`

## Requirements

- Rust 2021 edition or later (stable toolchain recommended)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
