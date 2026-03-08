use std::sync::mpsc::Sender;

use clap::{Parser, arg, command};

use crate::generate_data::StockQuote;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Server port
    #[arg(long, short, default_value_t = 10000)]
    pub port: u16,
    #[arg(long, short)]
    pub file_path: String,
    #[arg(long, short, default_value_t = 1 )]
    pub interval: u32
}

#[derive(Debug, Clone)]
pub struct Client {
    pub adress: String,
    pub port: u16,
    pub ticker: Vec<String>,
    pub ts: Option<Sender<String>>,
    pub alive: bool,
}

impl Client {
    pub fn new() -> Self {
        Self {
            adress: "127.0.0.1".to_string(),
            port: 9999,
            ticker: Vec::new(),
            ts: None,
            alive : false,
        }
    }
}
