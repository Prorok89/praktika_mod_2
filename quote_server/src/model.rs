use std::sync::{Arc, atomic::AtomicBool, mpsc::Sender};

use clap::{Parser};

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
    pub address: String,
    pub port: u16,
    pub ticker: Vec<String>,
    pub ts: Option<Sender<String>>,
    pub alive: Arc<AtomicBool>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            address: String::new(),
            port: 9999,
            ticker: Vec::new(),
            ts: None,
            alive : Arc::new(AtomicBool::new(true)),
        }
    }
}
