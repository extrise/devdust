# Dev Dust 🧹

**Clean build artifacts from development projects to reclaim disk space.**

Dev Dust is a fast, cross-platform command-line tool written in Rust that recursively scans directories to find development projects and cleans their build artifacts (like `target/`, `node_modules/`, `__pycache__/`, etc.).

> [!WARNING]
> Dev Dust permanently deletes files and directories. Always review what will be deleted before confirming, or use `--dry-run` to preview changes safely.

## Features

- **Fast** - Written in Rust for maximum performance
- **Smart Detection** - Automatically detects 18+ project types
- **Space Saver** - Reclaim gigabytes of disk space
- **Beautiful CLI** - Colored output with clear information
- **Safe** - Confirmation prompts before deletion
- **Flexible** - Many options for customization

## Supported Project Types

<details>
<summary><strong>Click to expand full list of 18+ supported project types</strong></summary>

### Currently Supported

- [x] **Rust** - Cargo projects (`target/`, `.xwin-cache/`)
- [x] **Node.js/JavaScript** - npm, yarn, pnpm (`node_modules/`, `.next/`, `dist/`, `build/`)
- [x] **Python** - pip, venv, pytest (`__pycache__/`, `.venv/`, `.pytest_cache/`)
- [x] **.NET** - C#, F# projects (`bin/`, `obj/`)
- [x] **Unity** - Game Engine projects (`Library/`, `Temp/`, `Obj/`)
- [x] **Unreal Engine** - Game projects (`Binaries/`, `Intermediate/`, `Saved/`)
- [x] **Java Maven** - Maven projects (`target/`)
- [x] **Java/Kotlin Gradle** - Gradle projects (`build/`, `.gradle/`)
- [x] **CMake** - C/C++ projects (`build/`, `cmake-build-*/`)
- [x] **Haskell Stack** - Stack projects (`.stack-work/`)
- [x] **Scala SBT** - SBT projects (`target/`, `project/target/`)
- [x] **PHP Composer** - Composer projects (`vendor/`)
- [x] **Dart/Flutter** - Flutter projects (`build/`, `.dart_tool/`)
- [x] **Elixir** - Mix projects (`_build/`, `.elixir-tools/`)
- [x] **Swift** - Swift Package Manager (`.build/`, `.swiftpm/`)
- [x] **Zig** - Zig projects (`zig-cache/`, `zig-out/`)
- [x] **Godot** - Godot 4.x projects (`.godot/`)
- [x] **Jupyter** - Jupyter notebooks (`.ipynb_checkpoints/`)

### Roadmap

- [ ] **Go** - Go modules (`vendor/`, `bin/`)
- [ ] **Ruby** - Bundler projects (`vendor/bundle/`)
- [ ] **Terraform** - Infrastructure projects (`.terraform/`)
- [ ] **Docker** - Build cache and volumes
- [ ] **Bazel** - Build system (`bazel-*/`)

</details>

## Installation

### Quick Install (Recommended)

> [!TIP]
> Use the provided installation script for the easiest setup experience. It handles building and installing devdust automatically.

```bash
# Clone the repository
git clone https://github.com/extrise/devdust.git
cd devdust

# Run the installation script
chmod +x install.sh
./install.sh
```

The install script supports several options:

```bash
# Install to a custom directory
./install.sh --prefix ~/.local

# Run tests before installing
./install.sh --test

# Skip building (use existing binary)
./install.sh --skip-build

# Show help
./install.sh --help
```

### Manual Installation

#### From Source

```bash
# Clone the repository
git clone https://github.com/extrise/devdust.git
cd devdust

# Build and install
cargo build --release
cargo install --path devdust-cli
```

> [!NOTE]
> Make sure `~/.cargo/bin` is in your PATH to use the `devdust` command globally.

### Platform-Specific Binaries

Pre-built binaries are available for multiple platforms via GitHub Releases:

| Platform | Architecture | Download |
|----------|-------------|----------|
| Linux | x86_64 | `devdust-linux-x86_64` |
| Linux | ARM64 | `devdust-linux-aarch64` |
| macOS | Intel | `devdust-macos-x86_64` |
| macOS | Apple Silicon | `devdust-macos-aarch64` |
| Windows | x86_64 | `devdust-windows-x86_64.exe` |

## Usage

### Basic Usage

```bash
# Scan current directory
devdust

# Scan specific directories
devdust ~/projects ~/work

# Clean all projects without confirmation
devdust --all

# Dry run (show what would be deleted)
devdust --dry-run
```

> [!IMPORTANT]
> Always use `--dry-run` first when scanning important directories to preview what will be deleted before actually cleaning.

### Advanced Options

```bash
# Only show projects older than 30 days
devdust --older 30d

# Follow symbolic links
devdust --follow-symlinks

# Stay on same filesystem (don't cross mount points)
devdust --same-filesystem

# Quiet mode (minimal output)
devdust --quiet

# Combine options for powerful workflows
devdust ~/projects --older 7d --all --quiet
```

### Command-Line Options Reference

| Option | Short | Description |
|--------|-------|-------------|
| `--all` | `-a` | Clean all found projects without confirmation |
| `--follow-symlinks` | `-L` | Follow symbolic links during scanning |
| `--same-filesystem` | `-s` | Stay on the same filesystem (don't cross mount points) |
| `--older <TIME>` | `-o` | Only show projects older than specified time |
| `--quiet` | `-q` | Quiet mode with minimal output |
| `--dry-run` | `-n` | Show what would be deleted without actually deleting |
| `--format <FORMAT>` | `-f` | Output format: `pretty`, `plain`, or `json` |
| `--help` | `-h` | Display help information |
| `--version` | `-V` | Display version information |

### Age Filter Examples

devdust supports flexible time specifications for the `--older` flag:

| Format | Meaning | Example |
|--------|---------|---------|
| `30m` | 30 minutes | `devdust --older 30m` |
| `2h` | 2 hours | `devdust --older 2h` |
| `7d` | 7 days | `devdust --older 7d` |
| `2w` | 2 weeks | `devdust --older 2w` |
| `6M` | 6 months | `devdust --older 6M` |
| `1y` | 1 year | `devdust --older 1y` |

> [!TIP]
> Use the `--older` flag to target stale projects that haven't been modified recently, keeping your active projects untouched.

## Examples

### Interactive Cleaning

```bash
$ devdust ~/projects

╔═══════════════════════════════════════╗
║        Dev Dust v1.0.0                ║
║  Clean Development Project Artifacts  ║
╚═══════════════════════════════════════╝

Scanning: /home/user/projects

Found: 5 projects with 2.3 GB of artifacts

● my-rust-app (Rust)
  Path: /home/user/projects/my-rust-app
  Artifacts: 1.2 GB
  Modified: 2 days ago
  → Artifact directories:
    • target
  ? Clean my-rust-app project? [y/N/a/q]: y
  ✓ Cleaned 1.2 GB

● old-website (Node.js)
  Path: /home/user/projects/old-website
  Artifacts: 450.5 MB
  Modified: 3 months ago
  → Artifact directories:
    • node_modules
  ? Clean old-website project? [y/N/a/q]: y
  ✓ Cleaned 450.5 MB

...

═══════════════════════════════════════════════════
Summary: 3 projects cleaned, 1.8 GB freed!
```

### Automated Cleaning

```bash
# Clean all projects older than 30 days
devdust ~/projects --older 30d --all --quiet

# Result: 12 projects cleaned, 5.4 GB freed
```

### Safe Preview Mode

```bash
# Preview what would be deleted without actually deleting
devdust ~/projects --dry-run

# Output shows potential space savings without making changes
```

### JSON Output for Scripting

```bash
# Get machine-readable output for automation
devdust ~/projects --format json --dry-run > projects.json
```

## Safety Guidelines

> [!CAUTION]
> devdust deletes files permanently. Follow these safety guidelines to avoid data loss:

### Best Practices

1. **Always test first**: Use `--dry-run` to preview changes before cleaning
2. **Start small**: Test on a single project directory before scanning large areas
3. **Check your backups**: Ensure important projects are backed up
4. **Review carefully**: Read the list of artifact directories before confirming deletion
5. **Use age filters**: Target old projects with `--older` to avoid cleaning active work

### What Gets Deleted

devdust **only** deletes recognized build artifact directories. It **never** deletes:

- [x] Source code files (`.rs`, `.js`, `.py`, etc.)
- [x] Configuration files (`Cargo.toml`, `package.json`, etc.)
- [x] Documentation and README files
- [x] Git repositories (`.git/` directories)
- [x] Any files outside artifact directories

### What Gets Cleaned

devdust **will** delete these artifact directories:

- ❌ Build outputs (`target/`, `build/`, `dist/`)
- ❌ Dependencies (`node_modules/`, `vendor/`)
- ❌ Cache directories (`__pycache__/`, `.cache/`)
- ❌ Temporary files (`Temp/`, `.tmp/`)

> [!NOTE]
> All deleted artifacts can be regenerated by rebuilding your projects. devdust never touches source code or configuration files.

### Building from Source

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized for performance)
cargo build --release

# Run tests
cargo test

# Run with logging enabled
RUST_LOG=debug cargo run

# Run clippy for linting
cargo clippy

# Format code
cargo fmt
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_format_size

# Run tests in release mode
cargo test --release
```

### Code Style

This project follows Rust best practices and conventions:

- [x] Comprehensive inline documentation
- [x] Clear error handling with custom error types
- [x] Modular design with separation of concerns
- [x] Extensive unit tests for core functionality
- [x] Type safety and zero-cost abstractions
- [x] Idiomatic Rust patterns

## Contributing

Contributions are welcome! Here's how you can help:

### Ways to Contribute

- 🐛 **Report bugs**: Open an issue with details about the problem
- 💡 **Suggest features**: Share ideas for new functionality
- 📝 **Improve documentation**: Fix typos or add examples
- 🔧 **Submit pull requests**: Fix bugs or implement features
- ⭐ **Star the project**: Show your support on GitHub

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes and add tests
4. Run tests and linting (`cargo test && cargo clippy`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to your branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

> [!TIP]
> Before submitting a PR, make sure all tests pass and the code is properly formatted with `cargo fmt`.

## Troubleshooting

### Common Issues

**Issue**: `devdust: command not found`

**Solution**: Make sure `~/.cargo/bin` is in your PATH:
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Issue**: Permission denied when cleaning

**Solution**: Some artifact directories may require elevated permissions. Run with appropriate permissions or skip those projects.

**Issue**: Symbolic links not followed

**Solution**: Use the `--follow-symlinks` flag to traverse symbolic links during scanning.

## License

MIT License - See [LICENSE](https://raw.githubusercontent.com/extrise/devdust/refs/heads/main/LICENSE) file for details.

## Author

**Ext Rise**
- Email: nayanchandradas@hotmail.com
- GitHub: [extrise](https://github.com/extrise)
- Repository: [github.com/extrise/devdust](https://github.com/extrise/devdust)

## Acknowledgments

devdust was built from scratch with modern Rust practices, inspired by similar tools in the ecosystem. Special thanks to the Rust community for excellent libraries like `clap`, `walkdir`, and `colored`.

---

**Made with ❤️ and Rust** | [Report Issues](https://github.com/extrise/devdust/issues) | [View Releases](https://github.com/extrise/devdust/releases)
