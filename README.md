# CPV - Copy with Progress Visualization

A modern file copy utility written in Rust that provides visual feedback during file operations. CPV works similarly to the standard `cp` command but adds a progress bar, ETA, and transfer speed information.

## Features

- 📊 Live progress bar showing copy progress
- ⏱️ Estimated time remaining (ETA)
- 📈 Transfer speed monitoring
- 📁 Support for both files and directories
- 🔄 Recursive directory copying
- 🛡️ Preserve file attributes
- 📝 Human-readable file sizes
- 🎯 Standard cp-like behavior

## Installation

### From source

```bash
# Clone the repository
git clone https://github.com/gohm44/cpv.git
cd cpv

# Build and install
cargo install --path .
```

## Usage

CPV follows the standard `cp` command syntax with additional features:

```bash
# Copy a single file
cpv source.txt destination.txt

# Copy a file into a directory
cpv file.txt /existing/directory/

# Copy directory (requires -r flag)
cpv -r source_dir target_dir

# Copy directory into existing directory
cpv -r source_dir /existing/directory/

# Copy with attribute preservation
cpv -p source.txt destination.txt

# Copy with verbose output
cpv -v source.txt destination.txt
```

### Command-line Options

```
OPTIONS:
    -r, --recursive    Copy directories recursively
    -p, --preserve    Preserve file attributes
    -f, --force       Force overwrite existing files
    -v, --verbose     Show verbose output with transfer statistics
    -h, --help        Print help information
```

## Examples

1. Copy a single file with progress:
```bash
cpv large_file.iso backup.iso
```

2. Copy a directory recursively:
```bash
cpv -r my_project project_backup
```

3. Copy with all features enabled:
```bash
cpv -rpvf source_dir destination_dir
```

## Development

### Prerequisites

- Rust 1.70.0 or higher
- Cargo

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'feat: add some amazing feature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by GNU cp
- Built with [indicatif](https://github.com/console-rs/indicatif) for progress bars
- CLI argument parsing with [clap](https://github.com/clap-rs/clap)
