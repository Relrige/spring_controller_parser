use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(
    name = "spring_controller_parser",
    version = "0.1.0",
    about = "A Spring controller parser implemented in Rust.",
    disable_help_subcommand = true,
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse { file: String },
    Help,
    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command{
        Commands::Parse { file } => {
            let controller_file = fs::read_to_string(file)
                .with_context(|| format!("Failed to read the file: {}", file))?;
            
        }

        Commands::Help => {
            println!("Spring controller parser CLI help");
            println!("To use: spring_controller_parser <Command>");
            println!();
            println!("Commands:");

            println!("  parse <file_path>  -   Parse the file");
            println!("  help               -   Show help");
            println!("  credits            -   Show credits(author)");
            println!();
            println!("Parse example");
            println!("  spring_controller_parser parse example.java");
        }

        Commands::Credits => {
            println!("Spring controller parser created by Stanislav Kulakevych");
        }
    }

    Ok(())
}