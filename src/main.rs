fn main() {
    if let Err(e) = rustar::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}


// use perfermant read and write in case of large files
// handle glob patterns *.txt, file?.txt, file[0-9].txt ..etc