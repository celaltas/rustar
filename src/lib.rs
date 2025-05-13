use archive::Archiver;
use clap::Parser;
mod archive;

type TarResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(version="1.0.0", about="RustyTar: A basic tar-like archiver rewritten in Rust", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    create: bool,
    #[arg(long)]
    delet: bool,
    #[arg(short = 't', long)]
    list: bool,
    #[arg(short = 'x', long)]
    extract: bool,
    #[arg(short = 'r', long)]
    append: bool,
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    files: Vec<String>,
}

pub fn run() -> TarResult<()> {
    let args = Args::parse();
    let archiver = Archiver::new();
    if args.create {
        match archiver.create("test.txt", args.files){
            Ok(_) => println!("Archive created successfully."),
            Err(e) => eprintln!("Error creating archive: {}", e),
        }
    }
    Ok(())
}
