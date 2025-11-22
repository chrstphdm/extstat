# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Interactive TUI mode
- JSON/CSV export
- Progress bar during scan
- Filter by date modified
- Drill-down by extension

## [0.1.0] - 2024-11-22

### Added
- Initial release
- Parallel file scanning with Rayon
- Beautiful table output with colored cells and visual bars
- Group files by extension with cumulative size
- Sort by total size (descending)
- CLI options:
  - `--min-size` / `-s`: Filter files by minimum size
  - `--top` / `-n`: Limit number of extensions displayed
  - `--show-count` / `-c`: Display file count per extension
- Human-readable size formatting (GiB, MiB, etc.)
- Support for files without extension
- High performance: ~500k files/second on modern SSD

### Documentation
- Comprehensive README with examples
- QUICKSTART guide for beginners
- BIOINFORMATICS use cases
- Detailed code comments for Rust learners

[Unreleased]: https://github.com/chrstphdm/extstat/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/chrstphdm/extstat/releases/tag/v0.1.0
