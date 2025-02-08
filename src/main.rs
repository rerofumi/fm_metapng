use clap::Parser;
use png::Decoder;
use std::fs::File;

/// fm_metapng
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// PNG file path
    #[arg(short, long, required = true)]
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = File::open(args.path)?;
    let decoder = Decoder::new(file);
    let reader = decoder.read_info()?;
    let info = reader.info();

    let mut displayed = false;
    for text in info.uncompressed_latin1_text.iter() {
        if text.keyword == "parameters" {
            let prompt = text.text.replace("Negative prompt: ", "\nNegative prompt:\n");
            println!("Positive prompt:");
            println!("{}", prompt);
            displayed = true;
        }
    }    
    if !displayed {
        println!("prompt info is not found.");
    }

    Ok(())
}
