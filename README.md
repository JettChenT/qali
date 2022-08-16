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
q gs "git status"
```

### Execute an command
Example: execute `gs`
```shell
q gs
```

## Features to implement
- Better output formatting
- Subcommands
- Change strage method to json or other serde formats
- Allow users to set descriptions for each command
- Linking to shell scripts
- Linking to python scripts(support for python environments)
- TUI to navigate existing commands
- Implement fuzzy finding
- More customizable options eg. print-only