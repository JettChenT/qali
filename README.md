# QALI (q)
**Q**uick **Ali**asing

Note: we're currently in the stage of initial development.

## Installation
```shell
cargo install qali
```

This installs two separate binaries : `q` and `qali`.
The command `q` is meant for actions with the highest usage frequency, such as setting and alias or executing a command.
The command `qali` is meant as a companion allowing for subcommands and more functionality without influencing the namespace in `q` which is set reserved completly by the users.

## Usage
`q --help`
`qali --help`
## Examples:

### Set an alias
Example: set `gs` as `git status`
```shell
q -s gs "git status"
```

Set `p` as `python`
```shell
q -s p python 
```
### Execute an command via alias
Example: execute `git status`
```shell
q gs
```

### List all existing commands
`qali ls`

### Remove a command
Example: remove `gs` as `git status`
`qali rm gs`

Execute `python --version` (As of now, use `--` when executing a command to stop flags meant for the program being parsed as flags for qali itself.)
```shell
q -- p --version
```

Example execute ``
## Features to implement
- Better output formatting
- Subcommands
- Change storage method to json or other serde formats
- Allow users to set descriptions for each command
- Linking to shell scripts
- Linking to python scripts(support for python environments)
- TUI to navigate existing commands
- Implement fuzzy finding
- More customizable options eg. print-only