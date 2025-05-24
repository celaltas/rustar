use archive::Archiver;
use clap::{Parser, Subcommand};
use std::path::Path;

mod archive;
mod header;
mod validation;

type TarResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(version="1.0.0", about="RustyTar: A basic tar-like archiver rewritten in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Create { archive: String, files: Vec<String> },
    List { archive: String },
    Append { archive: String, files: Vec<String> },
    Extract { archive: String, output_dir: String },
}

pub fn run() -> TarResult<()> {
    let cli = Cli::parse();
    let allowed_extensions = vec!["tar".to_string(), "rustar".to_string()];
    let archiver = Archiver::new(allowed_extensions);

    match cli.command {
        Command::Create { archive, files } => {
            let archive = Path::new(&archive);
            archiver.create(archive, files)?;
        }
        Command::List { archive } => {
            let archive = Path::new(&archive);
            archiver.list(archive)?;
        }
        Command::Append { archive, files } => {
            let archive = Path::new(&archive);
            archiver.append(archive, files)?;
        }
        Command::Extract {
            archive,
            output_dir,
        } => {
            let archive = Path::new(&archive);
            let output_dir = Path::new(&output_dir);
            archiver.extract(archive, output_dir)?;
        }
    }
    Ok(())
}
