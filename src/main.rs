use anyhow::Result;
use clap::Parser;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Color, Table};
use humansize::{format_size, BINARY};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Disk usage analyzer grouped by file extension
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to analyze
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Minimum file size to include (in bytes)
    #[arg(short = 's', long, default_value = "0")]
    min_size: u64,

    /// Maximum number of extensions to display
    #[arg(short = 'n', long, default_value = "50")]
    top: usize,

    /// Show file count
    #[arg(short = 'c', long)]
    show_count: bool,
}

#[derive(Debug, Clone)]
struct ExtensionStats {
    extension: String,
    total_size: u64,
    file_count: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🔍 Scanning {}...\n", args.path.display());

    // Collect all files
    let files: Vec<_> = WalkDir::new(&args.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    println!("📊 Analyzing {} files...\n", files.len());

    // Parallel analysis
    let stats: HashMap<String, (u64, usize)> = files
        .par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            let metadata = entry.metadata().ok()?;
            let size = metadata.len();

            if size < args.min_size {
                return None;
            }

            let extension = get_extension(path);
            Some((extension, size))
        })
        .fold(
            HashMap::new,
            |mut acc, (ext, size)| {
                let entry = acc.entry(ext).or_insert((0, 0));
                entry.0 += size;
                entry.1 += 1;
                acc
            },
        )
        .reduce(
            HashMap::new,
            |mut acc, map| {
                for (ext, (size, count)) in map {
                    let entry = acc.entry(ext).or_insert((0, 0));
                    entry.0 += size;
                    entry.1 += count;
                }
                acc
            },
        );

    // Convert to sorted vector
    let mut ext_stats: Vec<ExtensionStats> = stats
        .into_iter()
        .map(|(extension, (total_size, file_count))| ExtensionStats {
            extension,
            total_size,
            file_count,
        })
        .collect();

    ext_stats.sort_by(|a, b| b.total_size.cmp(&a.total_size));

    // Calculate totals
    let total_size: u64 = ext_stats.iter().map(|s| s.total_size).sum();
    let total_files: usize = ext_stats.iter().map(|s| s.file_count).sum();

    // Display results
    display_results(&ext_stats, total_size, total_files, &args);

    Ok(())
}

fn get_extension(path: &Path) -> String {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| format!(".{}", s.to_lowercase()))
        .unwrap_or_else(|| "[no extension]".to_string())
}

fn display_results(stats: &[ExtensionStats], total_size: u64, total_files: usize, args: &Args) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    // Headers
    if args.show_count {
        table.set_header(vec![
            Cell::new("Extension").fg(Color::Cyan),
            Cell::new("Size").fg(Color::Cyan),
            Cell::new("Files").fg(Color::Cyan),
            Cell::new("% Total").fg(Color::Cyan),
            Cell::new("Visual").fg(Color::Cyan),
        ]);
    } else {
        table.set_header(vec![
            Cell::new("Extension").fg(Color::Cyan),
            Cell::new("Size").fg(Color::Cyan),
            Cell::new("% Total").fg(Color::Cyan),
            Cell::new("Visual").fg(Color::Cyan),
        ]);
    }

    // Add rows
    let display_count = args.top.min(stats.len());
    for stat in stats.iter().take(display_count) {
        let size_str = format_size(stat.total_size, BINARY);
        let percentage = (stat.total_size as f64 / total_size as f64) * 100.0;
        let bar = create_bar(percentage);

        if args.show_count {
            table.add_row(vec![
                Cell::new(&stat.extension).fg(Color::Green),
                Cell::new(size_str).fg(Color::Yellow),
                Cell::new(stat.file_count.to_string()),
                Cell::new(format!("{:.2}%", percentage)),
                Cell::new(bar),
            ]);
        } else {
            table.add_row(vec![
                Cell::new(&stat.extension).fg(Color::Green),
                Cell::new(size_str).fg(Color::Yellow),
                Cell::new(format!("{:.2}%", percentage)),
                Cell::new(bar),
            ]);
        }
    }

    println!("{}", table);

    // Summary
    println!("\n📈 Summary:");
    println!("   Total size:       {}", format_size(total_size, BINARY));
    println!("   Total files:      {}", total_files);
    println!("   Extensions found: {}", stats.len());
    if stats.len() > display_count {
        println!("   (Showing top {} out of {})", display_count, stats.len());
    }
}

fn create_bar(percentage: f64) -> String {
    let bar_width = 30;
    let filled = ((percentage / 100.0) * bar_width as f64) as usize;
    let empty = bar_width - filled;

    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}
