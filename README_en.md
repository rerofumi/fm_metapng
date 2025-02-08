# fm_metapng

[日本語](README.md)

fm_metapng is a tool that extracts and displays AI-generated prompt information embedded within PNG files.
This repository analyzes the text data within PNGs (specifically the "parameters" keyword) and shows the prompt details.

## Features
- Extraction of embedded prompt information from PNG files
- Reformats the "Negative prompt:" section with line breaks for improved readability
- Notifies the user when no prompt information is found

## Usage
1. Run the program with the path to a PNG file:
    ```sh
    cargo run -- path/to/your_image.png
    ```
2. When a valid PNG file is provided, the prompt information is output to the console.
3. After execution, the command waits for user input (press Enter).  
   To skip this pause, run the command with the `-n` or `--no-pause` option.

## Pre-built Executables
Releases provide pre-built executables for:
- Windows
- Linux

## Dependencies
- [clap](https://crates.io/crates/clap) - Command line argument parser
- [png](https://crates.io/crates/png) - PNG file parser

## License
This project is licensed under the [MIT License](./LICENSE).

## Changelog
### v0.2.0
- Implemented Enter pause feature. Use the `-n` or `--no-pause` option to skip waiting for input.