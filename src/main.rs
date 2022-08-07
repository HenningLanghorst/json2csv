use crate::cli::Parameters;
use clap::Parser;
use std::io;
use std::process::exit;

mod cli;
mod convert;

fn main() {
    let parameters: Parameters = Parameters::parse();

    match serde_json::from_reader(io::stdin()) {
        Ok(json) => match convert::convert(json, parameters.delimiter()) {
            Ok(csv) => {
                println!("{}", csv);
            }
            Err(e) => {
                eprintln!("Error converting json to csv: {}", e);
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error parsing json: {}", e);
            exit(1);
        }
    }
}
