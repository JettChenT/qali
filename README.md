# QALI (q)
**Q**uick **Ali**asing

![Crates.io](https://img.shields.io/crates/l/qali) 
![Crates.io](https://img.shields.io/crates/v/qali)


[![asciicast](https://asciinema.org/a/517546.svg)](https://asciinema.org/a/517546)

## Purpose
QALI can...
- Shorten long & complex commands
- Act as a namespace for your custom scripts (stop worrying about conflicting names with system commands!)
- Make you type less
- Reduce your mental workload

## Installation

### Homebrew (for macOS)
```shell
brew tap JettChenT/qali
brew intall qali
```

### From release
download the [latest release](https://github.com/JettChenT/qali/releases/latest) of your platform, unzip, and move `q` and `qali` binaries to your `bin` folder (usually /usr/local/bin)

### Cargo
A [Rust](https://rust-lang.org) installation is required
```shell
cargo install qali
```

### Note for windows users: 

To run the QALI on a windows machine without encoding errors, use [windows terminal](https://github.com/microsoft/terminal).

`q.exe` works...

but `qali.exe` somehow gets detected as a [trojan virus](https://www.microsoft.com/en-us/wdsi/threats/malware-encyclopedia-description?Name=Trojan:Win32/Sabsik.FL.B!ml&ThreatID=2147780203) (I wish I know how to make one (>_<)
 
This can be solved by allowing the "virus" in windows defender.

Luckily, `qali.exe` is not required for simple actions such as setting and executing an alias.

## The Commands
This installs two separate binaries : `q` and `qali`.

The command `q` is meant for actions with the highest usage frequency, such as setting and alias or executing a command.

The command `qali` is meant as a companion allowing for subcommands and more functionality without influencing the namespace in `q` which is set reserved completly by the users.

## Usage
`q --help`

`qali --help`

## Supported alias types:
- Command: one liners, eg. `q -s hi "echo hi"`
- Shell: Shell scripts, eg. `q -s hi hello.sh` (*EXPERIMENTAL*) (Works **iff** your shell installation has the name "sh")
- Python: Python scripts, eg. `q -s hi hello.py` (*EXPERIMENTAL*) (Works **iff** your python command has the name "python")
- URI: open URI in default application, eg. `q -s hi https://beta.sayhello.so` 

## Examples:

### Set an alias
Example: set `gs` as `git status` 
```shell
q -s gs "git status"
```

Example: set `cc` to `cargo check` locally
```shell
q -s cc "cargo check" -m local
```

Set `p` as `python` if `p` doesn't exist
```shell
q p python 
```

Set `hi` to `./hi.py` with `qali`
```shell
qali set hi ./hi.py
```

### Execute aliases interactively (fuzzy select)
Method 1: `q`

Method 2: `qali select`

### Execute aliases directly
Example: execute `git status`
```shell
q gs
```
Execute `python --version` (As of now, use `--` when executing a command to stop flags meant for the program being parsed as flags for qali itself.)
```shell
q -- p --version
```

### List all existing commands
`qali list`

Alternatively, you can set `q -s ls "qali list"` to shorten this.
### Remove a command
Example: remove `gs` as `git status`:

`qali remove gs`

Alternatively, you can set `q -s rm "qali remove"` to shorten this.

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