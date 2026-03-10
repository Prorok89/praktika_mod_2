use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Server port
    #[arg(long, short)]
    pub server_addr: String,
    #[arg(long, short, default_value_t = 20000)]
    pub udp_port: u16,
    #[arg(long, short)]
    pub file_path: String,
}