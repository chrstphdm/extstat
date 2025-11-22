# Contributing to extstat

Thanks for your interest in contributing! 🎉

## Quick Start

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/extstat.git`
3. Create a branch: `git checkout -b feature/my-feature`
4. Make your changes
5. Test: `cargo test && cargo clippy`
6. Commit: `git commit -m "feat: add awesome feature"`
7. Push: `git push origin feature/my-feature`
8. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

### Building

```bash
# Debug build (fast compilation)
cargo build

# Release build (optimized)
cargo build --release

# Run directly
cargo run -- /path/to/scan -c
```

### Testing

```bash
# Run tests
cargo test

# Run with test data
./test.sh

# Check code quality
cargo clippy

# Format code
cargo fmt
```

## Code Style

- **Formatting**: Run `cargo fmt` before committing
- **Linting**: Fix all `cargo clippy` warnings
- **Comments**: Document public APIs and complex logic
- **Tests**: Add tests for new features

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation only
- `style:` Formatting, missing semicolons, etc.
- `refactor:` Code restructuring
- `perf:` Performance improvements
- `test:` Adding tests
- `chore:` Maintenance tasks

Examples:
```
feat: add JSON export option
fix: handle permission errors gracefully
docs: update README with new examples
perf: optimize parallel scanning for large directories
```

## Pull Request Process

1. **Update documentation** if you're adding features
2. **Add tests** for new functionality
3. **Update CHANGELOG.md** with your changes
4. **Ensure CI passes** (tests, clippy, fmt)
5. **Request review** from maintainers

## Project Structure

```
extstat/
├── src/
│   └── main.rs          # Main application code
├── tests/               # Integration tests (to be added)
├── Cargo.toml           # Dependencies
├── README.md            # Main documentation
└── CONTRIBUTING.md      # This file
```

## Areas for Contribution

### Good First Issues
- [ ] Add more output formats (CSV, JSON)
- [ ] Improve error messages
- [ ] Add more CLI options
- [ ] Write integration tests

### Medium Difficulty
- [ ] Add progress bar during scan
- [ ] Filter by file modification date
- [ ] Compare two scans (before/after)
- [ ] Add verbose mode for debugging

### Advanced
- [ ] Interactive TUI mode (ncdu-like)
- [ ] Drill-down: extension → directories
- [ ] Cross-platform support (Windows, macOS)
- [ ] Remote filesystem support

## Feature Requests

Open an issue with:
- **Use case**: Why is this needed?
- **Proposed solution**: How should it work?
- **Examples**: CLI usage examples

## Bug Reports

Include:
- **Version**: `extstat --version`
- **OS**: Linux distro, kernel version
- **Command**: Exact command run
- **Expected**: What should happen
- **Actual**: What actually happened
- **Logs**: Any error messages

## Questions?

- Open a [Discussion](https://github.com/chrstphdm/extstat/discussions)
- Check existing [Issues](https://github.com/chrstphdm/extstat/issues)

## Code of Conduct

Be respectful, constructive, and professional. We're all here to build something useful together.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thanks for contributing! 🦀✨
