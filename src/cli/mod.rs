pub use clap::Parser;

/// Converts JSON from standard input to CSV, TSV etc. which is output to standard output.
#[derive(Debug, Parser)]
#[clap(version)]
pub struct Parameters {
    /// Delimiter to be used for separating columns.
    #[clap(short, long, value_parser, default_value = ";", group = "delim")]
    delimiter: char,
    /// Use tabulator as delimiter to be used for separating columns.
    #[clap(
        short = 't',
        long,
        value_parser,
        default_value = "false",
        group = "delim"
    )]
    delimiter_tab: bool,
}

impl Parameters {
    /// Returns the specified delimiter which is tabulator if tab flag is true
    /// and otherwise the specified delimiter
    pub fn delimiter(&self) -> char {
        if self.delimiter_tab {
            '\t'
        } else {
            self.delimiter
        }
    }
}
