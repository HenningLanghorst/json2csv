use std::io;
use std::process::exit;

mod convert;

fn main() {
    match serde_json::from_reader(io::stdin()) {
        Ok(json) => match convert::convert(json) {
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
