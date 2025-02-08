use clap::Parser;
use png::Decoder;
use std::fs::File;
use std::io;
use std::io::prelude::*;

/// fm_metapng
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// PNG file path
    path: String,

    /// Do not pause before exiting
    #[arg(short = 'n', long = "no-pause")]
    no_pause: bool,
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
            let prompt = text.text.replace("Negative prompt: ", "\nNegative prompt: \n");
            println!("Positive prompt:");
            println!("{}", prompt);
            displayed = true;
        }
    }
    if !displayed {
        println!("prompt info is not found.");
    }

    if !args.no_pause {
        println!("Press Enter to exit...");
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
        write!(stdout, "").unwrap();
        stdout.flush().unwrap();

        // Read a single byte and discard
        let _ = stdin.read(&mut [0u8]).unwrap();
    }

    Ok(())
}
