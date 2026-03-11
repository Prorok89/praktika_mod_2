use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Server port
    #[arg(long, short)]
    pub tcp_server: String,
    #[arg(long, short)]
    pub udp_server: String,
    #[arg(long, short)]
    pub file_path: String,
}