mod batch;
mod commands;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "litemark")]
#[command(about = "A lightweight photo parameter watermark tool")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add watermark to a single image
    Add {
        /// Input image path
        #[arg(short, long)]
        input: String,

        /// Template name or path to template JSON
        #[arg(short, long, default_value = "classic")]
        template: String,

        /// Output image path
        #[arg(short, long)]
        output: String,

        /// Author name (overrides EXIF data)
        #[arg(long)]
        author: Option<String>,

        /// Custom font file path (uses default if not specified)
        #[arg(long)]
        font: Option<String>,

        /// Logo file path (overrides template and environment variable)
        #[arg(long)]
        logo: Option<String>,
    },

    /// Batch process images in a directory
    Batch {
        /// Input directory path
        #[arg(short, long)]
        input_dir: String,

        /// Template name or path to template JSON
        #[arg(short, long, default_value = "classic")]
        template: String,

        /// Output directory path
        #[arg(short, long)]
        output_dir: String,

        /// Author name (overrides EXIF data)
        #[arg(long)]
        author: Option<String>,

        /// Custom font file path (uses default if not specified)
        #[arg(long)]
        font: Option<String>,

        /// Logo file path (overrides template and environment variable)
        #[arg(long)]
        logo: Option<String>,

        /// Maximum concurrent tasks (default: auto-detect CPU cores × 2)
        #[arg(short, long)]
        concurrency: Option<usize>,

        /// Disable progress bar
        #[arg(long)]
        no_progress: bool,
    },

    /// List available templates
    Templates,

    /// Show template details
    ShowTemplate {
        /// Template name
        template: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            input,
            template,
            output,
            author,
            font,
            logo,
        } => {
            commands::process_single_image(
                &input,
                &template,
                &output,
                author.as_deref(),
                font.as_deref(),
                logo.as_deref(),
            )?;
        }
        Commands::Batch {
            input_dir,
            template,
            output_dir,
            author,
            font,
            logo,
            concurrency,
            no_progress,
        } => {
            batch::process_batch(
                &input_dir,
                &template,
                &output_dir,
                author.as_deref(),
                font.as_deref(),
                logo.as_deref(),
                concurrency,
                !no_progress,
            )?;
        }
        Commands::Templates => {
            commands::list_templates();
        }
        Commands::ShowTemplate { template } => {
            commands::show_template(&template)?;
        }
    }

    Ok(())
}
