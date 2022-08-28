# QALI (q)
**Q**uick **Ali**asing

![Crates.io](https://img.shields.io/crates/l/qali) 
![Crates.io](https://img.shields.io/crates/v/qali)

Note: we're currently in the stage of initial development.

[![asciicast](https://asciinema.org/a/517408.svg)](https://asciinema.org/a/517408)

## Purpose
QALI can...
- Shorten long & complex commands
- Act as a namespace for your custom scripts (stop worrying about conflicting names with system commands!)
- Make you type less
- Reduce your mental workload

## Installation
Currently, a [Rust](https://www.rust-lang.org) installation is required 

```shell
cargo install qali
```

This installs two separate binaries : `q` and `qali`.

The command `q` is meant for actions with the highest usage frequency, such as setting and alias or executing a command.

The command `qali` is meant as a companion allowing for subcommands and more functionality without influencing the namespace in `q` which is set reserved completly by the users.

## Usage
`q --help`

`qali --help`

## Supported alias types:
- Command: one liners, eg. `qali set hi "echo hi"`
- Shell: Shell scripts, eg. `qali set hi hello.sh` (*EXPERIMENTAL*)
- Python: Python scripts, eg. `qali set hi hello.py` (*EXPERIMENTAL*)
- URI: open URI in default application, eg. `qali set hi https://beta.sayhello.so` (*EXPERIMENTAL*)

## Examples:

### Set an alias
Example: set `gs` as `git status` 
```shell
q -s gs "git status"
```

Set `p` as `python` if `p` doesn't exist
```shell
q p python 
```

Set `hi` to `./hi.py` with `qali`
```shell
qali set hi ./hi.py
```

### Execute an command via alias
Execute aliases interactively
Method 1:`q`

Method 2:`qali select`

Example: execute `git status`
```shell
q gs
```
Execute `python --version` (As of now, use `--` when executing a command to stop flags meant for the program being parsed as flags for qali itself.)
```shell
q -- p --version
```

### List all existing commands
`qali ls`

### Remove a command
Example: remove `gs` as `git status`:

`qali rm gs`

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