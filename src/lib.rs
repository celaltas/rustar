use archive::Archiver;
use clap::{Parser, Subcommand};
mod archive;

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
    Extract { archive: String },
}

pub fn run() -> TarResult<()> {
    let cli = Cli::parse();
    let archiver = Archiver::new();

    match cli.command {
        Command::Create { archive, files } => {
            archiver.create(&archive, files)?;
        }
        Command::List { archive } => {
            archiver.list(&archive)?;
        }
        Command::Append { archive, files } => {
            archiver.append(&archive, files)?;
        }
        Command::Extract { archive } => {
            archiver.extract(&archive)?;
        }
    }
    Ok(())
}
