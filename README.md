# extstat

[![CI](https://github.com/chrstphdm/extstat/workflows/CI/badge.svg)](https://github.com/chrstphdm/extstat/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

⚡ Fast disk usage analyzer that groups files by extension. Perfect for finding what's taking up space on your drives.

A parallel, high-performance CLI tool that scans directories and displays disk usage statistics grouped by file extension, with beautiful terminal output.

## Features

✅ **Parallel scanning** - Uses all CPU cores for maximum speed
✅ **Beautiful table output** - Color-coded results with visual bars
✅ **Flexible filtering** - Minimum file size, top N extensions
✅ **File count tracking** - See how many files per extension
✅ **No dependencies** - Single binary, works everywhere

## Installation

### From source (recommended for now)

```bash
# Clone or copy the project
cd extstat

# Build release version (optimized)
cargo build --release

# Binary will be in target/release/extstat
# Copy to your PATH
sudo cp target/release/extstat /usr/local/bin/
```

## Usage

### Basic usage

```bash
# Analyze current directory
extstat

# Analyze specific directory
extstat /path/to/directory

# Show file counts
extstat -c

# Filter small files (e.g., min 1MB)
extstat -s 1048576

# Show only top 20 extensions
extstat -n 20

# Combine options
extstat /data -c -s 1000000 -n 10
```

### Examples

```bash
# Analyze your home directory
extstat ~

# Find what's taking space in /var
extstat /var -n 15

# Show detailed stats for current project
extstat . -c
```

## Command Line Options

```
Options:
  <PATH>              Directory to analyze [default: .]
  -s, --min-size      Minimum file size to include (in bytes) [default: 0]
  -n, --top           Maximum number of extensions to display [default: 50]
  -c, --show-count    Show file count
  -h, --help          Print help
  -V, --version       Print version
```

## Output Explanation

```
╭────────────┬──────────┬─────────┬──────────────────────────────────╮
│ Extension  │ Size     │ % Total │ Visual                           │
├────────────┼──────────┼─────────┼──────────────────────────────────┤
│ .fastq     │ 2.5 GiB  │ 45.23%  │ ██████████████░░░░░░░░░░░░░░░░░░ │
│ .bam       │ 1.2 GiB  │ 21.67%  │ ███████░░░░░░░░░░░░░░░░░░░░░░░░░ │
│ .fasta     │ 567 MiB  │ 10.11%  │ ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ │
╰────────────┴──────────┴─────────┴──────────────────────────────────╯
```

- **Extension**: File extension (or [no extension] for files without one)
- **Size**: Total size for all files with this extension (human-readable)
- **% Total**: Percentage of total scanned space
- **Visual**: Bar chart representation

## Performance

- Parallel scanning using Rayon (uses all CPU cores)
- Typical performance: ~500k files/second on modern SSD
- Memory efficient: doesn't load file contents, only metadata

## Development

### Project Structure

```
extstat/
├── Cargo.toml       # Rust dependencies
├── src/
│   └── main.rs      # Main application code
└── README.md        # This file
```

### Building for development

```bash
# Build debug version (faster compilation)
cargo build

# Run directly
cargo run -- /path/to/scan

# Run with options
cargo run -- . -c -n 10
```

### Understanding the code

**Key Rust concepts used:**

1. **Parallel iteration with Rayon**: 
   ```rust
   files.par_iter()  // Process files in parallel
   ```

2. **Result handling with `?`**:
   ```rust
   let metadata = entry.metadata().ok()?;  // Return None if error
   ```

3. **Pattern matching**:
   ```rust
   path.extension()
       .and_then(|s| s.to_str())  // Chain operations safely
   ```

4. **HashMap aggregation**:
   ```rust
   let entry = acc.entry(ext).or_insert((0, 0));  // Get or create
   entry.0 += size;  // Update tuple
   ```

### Adding features

Want to add more features? Common additions:

1. **JSON export**: Add `serde` and `serde_json` dependencies
2. **Interactive TUI**: Add `ratatui` and `crossterm`
3. **Progress bar**: Add `indicatif` dependency
4. **Date filtering**: Use file metadata `modified()` time

## Troubleshooting

**Permission denied errors**: 
- Use `sudo` for system directories
- Or skip inaccessible files (feature coming soon)

**Slow on network drives**:
- Network I/O is the bottleneck, not the tool
- Consider scanning locally first

**Out of memory**:
- Only happens with millions of different extensions
- Try filtering with `-s` to reduce file count

## Why Rust?

- **Speed**: As fast as C/C++, often faster than Go/Python
- **Safety**: No segfaults, data races prevented at compile time
- **Modern**: Great tooling (cargo), helpful compiler errors
- **Dependencies**: Easy to manage, reproducible builds

## Next Steps (Version 2)

Planned features:
- [ ] Interactive TUI mode (like ncdu)
- [ ] Drill-down: click extension → see directories
- [ ] Export to JSON/CSV
- [ ] Progress bar during scan
- [ ] Filter by date modified
- [ ] Compare two scans (before/after cleanup)

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes in each release.

## License

MIT License - Feel free to use, modify, distribute
