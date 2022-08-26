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
    let input: Box<dyn Read> = Box::new(io::stdin());

    match convert_input_to_csv(input, parameters.delimiter()) {
        Ok(csv) => {
            println!("{}", csv);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
