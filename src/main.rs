use crate::cli::Parameters;
use crate::convert::convert_input_to_csv;
use clap::Parser;
use std::io;
use std::io::Read;
use std::process::exit;

mod cli;
mod convert;

fn main() {
    let parameters: Parameters = Parameters::parse();

    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error: {}", e);
        exit(1);
    }

    match convert_input_to_csv(&input, parameters.delimiter()) {
        Ok(csv) => {
            println!("{}", csv);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
