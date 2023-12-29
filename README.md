# bleach

`bleach` is a command-line utility designed to remove ANSI color codes and other escape sequences from files. It provides an easy way to clean up text files that contain unwanted formatting codes, especially useful for logs or output files generated from scripts and programs.

## Features

- **In-Place Editing:** Modify files directly, saving the results back to the original file.
- **Backup Option:** Create a backup of the original file before making any changes.
- **Customizable Cleaning:** Specify the types of ANSI sequences to remove, such as color codes, cursor movements, etc.

## Installation

(Installation instructions will be here. For example, you might provide a command to clone the repository or download a binary.)

## Usage

```bash
bleach [options] [file]
```

### Options

- `-h, --help`: Display the help menu.
- `-i, --in-place`: Edit the file in-place, saving the result to the same file.
- `-b, --backup`: Create a backup of the original file when using the `-i` option. The backup file is saved with the `.bak` extension.
- `-c, --clean-types [types]`: Specify the types of ANSI sequences to remove, such as `color`, `movement`, `all`.

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
