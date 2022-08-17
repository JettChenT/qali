# QALI
**Q**uick **Ali**asing

A work in progress

## Installation
```shell
cargo install qali
```

## Usage
`q --help`

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
### Execute an command
Example: execute `git status`
```shell
q gs
```

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