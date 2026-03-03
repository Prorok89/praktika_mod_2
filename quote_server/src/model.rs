use clap::{Parser, arg, command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Server port
    #[arg(long, short, default_value_t = 10000)]
    pub port: u16,
}