# bleach

[![Cargo Build & Test](https://github.com/kojix2/bleach/actions/workflows/ci.yml/badge.svg)](https://github.com/kojix2/bleach/actions/workflows/ci.yml)

`bleach` is a command-line utility designed to remove ANSI color codes and other escape sequences from files. It provides an easy way to clean up text files that contain unwanted formatting codes, especially useful for logs or output files generated from scripts and programs.

## Installation

```bash
cargo install --git https://github.com/kojix2/bleach
```

## Usage

```bash
bleach [options] [file]
```

### Options

- `-h, --help`: Display the help menu.
- `-i, --in-place`: Edit the file in-place, saving the result to the same file.
- `-b, --backup`: Create a backup of the original file when using the `-i` option. The backup file is saved with the `.bak` extension.
- `-c, --clean-types [types]`: Specify the types of ANSI sequences to remove: `color`, `movement`, `all`. Defaults to `all` if not specified.

### Examples

- Directly edit a file and remove color codes, creating a backup:

```bash
bleach -i -b file.txt
```

- Remove specific types of ANSI sequences from a file:

```bash
bleach -i -c color,movement file.txt
```

## Contributing

Contributions to `bleach` are welcome! Please feel free to submit pull requests or open issues to suggest improvements or report bugs.

## License

This tool was created making full use of ChatGPT and Copilot.

MIT
