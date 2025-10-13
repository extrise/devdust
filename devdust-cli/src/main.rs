//! Dev Dust CLI
//!
//! A command-line tool to scan directories for development projects
//! and clean their build artifacts to reclaim disk space.
//!
//! Author: Ext Rise <nayanchandradas@hotmail.com>
//! Repository: https://github.com/extrise/devdust

use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    process,
};

use clap::{Parser, ValueEnum};
use colored::*;
use devdust_core::{format_elapsed_time, format_size, scan_directory, Project, ScanOptions};

// ============================================================================
// CLI Argument Parsing
// ============================================================================

/// devdust - Clean build artifacts from development projects
#[derive(Parser, Debug)]
#[command(
    name = "devdust",
    version,
    author = "Ext Rise <nayanchandradas@hotmail.com>",
    about = "Scan and clean build artifacts from development projects",
    long_about = "Dev Dust recursively scans directories to find development projects \
                  (Rust, Node.js, Python, .NET, Unity, etc.) and cleans their build \
                  artifacts to reclaim disk space."
)]
struct Args {
    /// Directories to scan (defaults to current directory)
    #[arg(value_name = "PATHS")]
    paths: Vec<PathBuf>,

    /// Clean all found projects without confirmation
    #[arg(short, long)]
    all: bool,

    /// Follow symbolic links during scanning
    #[arg(short = 'L', long)]
    follow_symlinks: bool,

    /// Stay on the same filesystem (don't cross mount points)
    #[arg(short = 's', long)]
    same_filesystem: bool,

    /// Only show projects older than specified time (e.g., 30d, 2w, 6M)
    #[arg(short, long, value_name = "TIME")]
    older: Option<String>,

    /// Quiet mode (minimal output)
    #[arg(short, long)]
    quiet: bool,

    /// Dry run (show what would be deleted without actually deleting)
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// Output format
    #[arg(short = 'f', long, value_enum, default_value = "pretty")]
    format: OutputFormat,
}

/// Output format options
#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    /// Pretty colored output with formatting
    Pretty,
    /// Plain text output (no colors)
    Plain,
    /// JSON output
    Json,
}

// ============================================================================
// Main Entry Point
// ============================================================================

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Run the application and handle errors
    if let Err(e) = run(args) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

/// Main application logic
fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Determine paths to scan
    let paths = if args.paths.is_empty() {
        vec![env::current_dir()?]
    } else {
        args.paths
    };

    // Validate paths
    for path in &paths {
        if !path.exists() {
            return Err(format!("Path does not exist: {}", path.display()).into());
        }
        if !path.is_dir() {
            return Err(format!("Path is not a directory: {}", path.display()).into());
        }
    }

    // Parse age filter if provided
    let min_age_seconds = if let Some(ref age_str) = args.older {
        parse_age_filter(age_str)?
    } else {
        0
    };

    // Configure scan options
    let scan_options = ScanOptions {
        follow_symlinks: args.follow_symlinks,
        same_filesystem: args.same_filesystem,
        min_age_seconds,
    };

    // Print header
    if !args.quiet && matches!(args.format, OutputFormat::Pretty) {
        print_header();
    }

    // Scan for projects
    let mut projects = Vec::new();
    let mut total_artifact_size = 0u64;

    for path in &paths {
        if !args.quiet {
            println!(
                "{} {}",
                "Scanning:".cyan().bold(),
                path.display().to_string().white()
            );
        }

        // Scan the directory
        for result in scan_directory(path, &scan_options) {
            match result {
                Ok(project) => {
                    // Calculate artifact size
                    let artifact_size = project.calculate_artifact_size(&scan_options);

                    // Skip projects with no artifacts
                    if artifact_size == 0 {
                        continue;
                    }

                    total_artifact_size += artifact_size;
                    projects.push((project, artifact_size));
                }
                Err(e) => {
                    if !args.quiet {
                        eprintln!("{} {}", "Warning:".yellow(), e);
                    }
                }
            }
        }
    }

    // Sort projects by artifact size (largest first)
    projects.sort_by(|a, b| b.1.cmp(&a.1));

    if projects.is_empty() {
        if !args.quiet {
            println!("\n{}", "Scan Finished...".green().bold());
            println!("{}", "No projects with build artifacts found.".yellow());
            println!("\n{}", "This could mean:".bright_black());
            println!(
                "  {} No development projects in the scanned directories",
                "•".bright_black()
            );
            println!("  {} All projects are already clean", "•".bright_black());
            println!(
                "  {} Projects are too new (if using --older filter)",
                "•".bright_black()
            );
        }
        return Ok(());
    }

    // Display results
    if !args.quiet {
        println!(
            "\n{} {} projects with {} of artifacts\n",
            "Found:".green().bold(),
            projects.len().to_string().white().bold(),
            format_size(total_artifact_size).white().bold()
        );
    }

    // Display projects and prompt for cleaning
    let mut total_cleaned = 0u64;
    let mut projects_cleaned = 0usize;

    for (project, artifact_size) in projects {
        // Display project info
        if !args.quiet {
            display_project(&project, artifact_size, &scan_options);
        }

        // Determine if we should clean this project
        let should_clean = if args.all {
            true
        } else if args.dry_run {
            false
        } else {
            prompt_clean(&project)?
        };

        if should_clean {
            if args.dry_run {
                if !args.quiet {
                    println!(
                        "  {} Would delete {}",
                        "→".blue(),
                        format_size(artifact_size)
                    );
                }
                total_cleaned += artifact_size;
                projects_cleaned += 1;
            } else {
                // Actually clean the project
                match project.clean() {
                    Ok(deleted) => {
                        if !args.quiet {
                            println!(
                                "  {} Cleaned {}",
                                "✓".green().bold(),
                                format_size(deleted).green()
                            );
                        }
                        total_cleaned += deleted;
                        projects_cleaned += 1;
                    }
                    Err(e) => {
                        eprintln!("  {} Failed to clean: {}", "✗".red().bold(), e);
                    }
                }
            }
        }

        if !args.quiet {
            println!(); // Blank line between projects
        }
    }

    // Print summary
    if !args.quiet {
        print_summary(projects_cleaned, total_cleaned, args.dry_run);
    }

    Ok(())
}

// ============================================================================
// Display Functions
// ============================================================================

/// Prints the application header
fn print_header() {
    println!("{}", "╔═══════════════════════════════════════╗".cyan());
    println!("{}", "║         Dev Dust v1.0.0               ║".cyan());
    println!("{}", "║  Clean Development Project Artifacts  ║".cyan());
    println!("{}", "╚═══════════════════════════════════════╝".cyan());
    println!();
}

/// Displays information about a project
fn display_project(project: &Project, artifact_size: u64, options: &ScanOptions) {
    println!(
        "{} {} {}",
        "●".blue().bold(),
        project.display_name().white().bold(),
        format!("({})", project.project_type.name()).bright_black()
    );
    println!("  {} {}", "Path:".bright_black(), project.path.display());
    println!(
        "  {} {}",
        "Artifacts:".bright_black(),
        format_size(artifact_size).yellow().bold()
    );

    // Show last modified time if available
    if let Ok(last_modified) = project.last_modified(options) {
        if let Ok(elapsed) = last_modified.elapsed() {
            println!(
                "  {} {}",
                "Modified:".bright_black(),
                format_elapsed_time(elapsed.as_secs()).bright_black()
            );
        }
    }

    // List artifact directories
    println!("  {} Artifact directories:", "→".bright_black());
    for dir in project.project_type.artifact_directories() {
        let dir_path = project.path.join(dir);
        if dir_path.exists() {
            println!("    • {}", dir.bright_black());
        }
    }
}

/// Prints the final summary
fn print_summary(projects_cleaned: usize, total_cleaned: u64, dry_run: bool) {
    println!("{}", "═".repeat(50).cyan());

    if dry_run {
        println!(
            "{} {} projects, {} would be freed",
            "Dry run:".yellow().bold(),
            projects_cleaned.to_string().white().bold(),
            format_size(total_cleaned).white().bold()
        );
    } else {
        println!(
            "{} {} projects cleaned, {} freed!",
            "Summary:".green().bold(),
            projects_cleaned.to_string().white().bold(),
            format_size(total_cleaned).green().bold()
        );
    }
}

// ============================================================================
// User Interaction
// ============================================================================

/// Prompts the user to confirm cleaning a project
fn prompt_clean(project: &Project) -> Result<bool, Box<dyn std::error::Error>> {
    print!(
        "  {} Clean {} project? [y/N/a/q]: ",
        "?".yellow().bold(),
        project.display_name().white().bold()
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => Ok(true),
        "n" | "no" | "" => Ok(false),
        "a" | "all" => {
            // This would require refactoring to support "clean all remaining"
            // For now, just treat as "yes"
            Ok(true)
        }
        "q" | "quit" => {
            println!("{}", "Exiting...".yellow());
            process::exit(0);
        }
        _ => {
            println!("  {} Invalid input, skipping...", "!".red());
            Ok(false)
        }
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Parses an age filter string (e.g., "30d", "2w", "6M") into seconds
fn parse_age_filter(input: &str) -> Result<u64, String> {
    const MINUTE: u64 = 60;
    const HOUR: u64 = MINUTE * 60;
    const DAY: u64 = HOUR * 24;
    const WEEK: u64 = DAY * 7;
    const MONTH: u64 = DAY * 30;
    const YEAR: u64 = DAY * 365;

    if input.is_empty() {
        return Err("Age filter cannot be empty".to_string());
    }

    // Split into number and unit
    let (num_str, unit) = input.split_at(input.len() - 1);

    let number: u64 = num_str
        .parse()
        .map_err(|_| format!("Invalid number: {}", num_str))?;

    let multiplier = match unit {
        "m" => MINUTE,
        "h" => HOUR,
        "d" => DAY,
        "w" => WEEK,
        "M" => MONTH,
        "y" => YEAR,
        _ => return Err(format!("Invalid unit: {}. Use m, h, d, w, M, or y", unit)),
    };

    Ok(number * multiplier)
}
