use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

use ctxgen::{generate_context_markdown, write_output_files};

/// Generate AGENTS.md and CLAUDE.md from a .context folder
#[derive(Parser, Debug)]
#[command(name = "ctxgen")]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the .context folder (defaults to .context in current directory)
    #[arg(short, long, default_value = ".context")]
    context_dir: PathBuf,

    /// Output directory for generated files (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    output_dir: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate context directory exists
    if !args.context_dir.exists() {
        anyhow::bail!(
            "Context directory '{}' does not exist",
            args.context_dir.display()
        );
    }

    if !args.context_dir.is_dir() {
        anyhow::bail!(
            "'{}' is not a directory",
            args.context_dir.display()
        );
    }

    // Generate the markdown content
    let markdown_content = generate_context_markdown(&args.context_dir)
        .context("Failed to generate context markdown")?;

    // Write output files
    write_output_files(&args.output_dir, &markdown_content)
        .context("Failed to write output files")?;

    println!("✓ Generated AGENTS.md and CLAUDE.md in {}", args.output_dir.display());

    Ok(())
}

