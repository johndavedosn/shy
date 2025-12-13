use clap::Parser;
use std::fs::File;
use sha2::{Digest, Sha256};
use std::path::Path;
#[derive(Debug, Parser)]
#[command(long_about=None, author="Ali moumen", version="1.0", about="Small CLI utility to SHA256-hash files and strings.")]
#[command(arg_required_else_help = true)]
struct Args {
    #[arg(short = 'f', long = "file")]

    file: Option<String>,
    #[arg(short = 's', long = "string")]
    string: Option<String> 
}
fn main() -> Result<(), std::io::Error>{
    let args = Args::parse();
    if !args.file.is_none() && !args.string.is_none() {
        println!("You cannot provide both options,  either a file or a string.");
        println!("Exiting.");
        return Ok(());
    }
    if !args.string.is_none() {
        let hash = Sha256::digest(args.string.unwrap());
        println!("Hash: {hash:x}");
    }
    if !args.file.is_none() {
        if !Path::new(&args.file.clone().unwrap()).exists() {
            println!("Hello? You can't hash a file that doesn't exist,  twin.");
            return Ok(());
        }
        let mut file = File::open(args.file.unwrap())
            .expect("Couldn't open file");
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        let hash_result = hasher.finalize();
        println!("Hash: {hash_result:x}")

    }
    Ok(())
}
