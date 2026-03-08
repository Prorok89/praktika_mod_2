use std::{
    sync::{Arc, RwLock},
    thread,
    time::{self, Duration, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::{error::ServerError, model::Client};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockQuote {
    pub ticker: String,
    pub price: f64,
    pub volume: u32,
    pub timestamp: u64,
}

// Методы для сериализации/десериализации
impl StockQuote {
    pub fn new(ticker: &str) -> Self {
        let volume = match ticker {
            "AAPL" | "MSFT" | "TSLA" => 1000 + (rand::random::<f64>() * 5000.0) as u32,
            _ => 100 + (rand::random::<f64>() * 1000.0) as u32,
        };

        let last_price: f64 = 10.0;

        Self {
            ticker: ticker.to_string(),
            price: last_price,
            volume,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.ticker, self.price, self.volume, self.timestamp
        )
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() == 4 {
            Some(StockQuote {
                ticker: parts[0].to_string(),
                price: parts[1].parse().ok()?,
                volume: parts[2].parse().ok()?,
                timestamp: parts[3].parse().ok()?,
            })
        } else {
            None
        }
    }

    // Или бинарная сериализация
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.ticker.as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.price.to_string().as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.volume.to_string().as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.timestamp.to_string().as_bytes());
        bytes
    }

    pub fn update_data(&mut self) {
        
        if self.volume > 0 {
            let quantity_coefficient: f64 = rand::random_range(0.7..1.1);
            self.volume = (self.volume * ((quantity_coefficient * 100.0) as u32)) / 100;
        }
        else
        {
            let quantity_coefficient: u32 = rand::random_range(1000..2000);
            self.volume = quantity_coefficient;
        }
        
        if self.price > 0.0 {
            let cost_coefficient: f64 = rand::random_range(0.9..1.1);
            self.price = self.price * cost_coefficient;
        }
        else
        {
            self.price = 1.0;
        }    
    }
}

pub struct QuoteGenerator;

impl QuoteGenerator {
    pub fn generate_multiple(
        stock_quote: Arc<RwLock<Vec<StockQuote>>>,
        clients: Arc<RwLock<Vec<Client>>>,
        interval : Duration
    ) -> Result<(), ServerError> {
        loop {
            let mut lock_stock_quotes =
                stock_quote.write().map_err(|e| ServerError::SendServer {
                    value: format!("Failed to acquire read lock for tickers: {:?}", e),
                })?;

            for stock_quote in lock_stock_quotes.iter_mut() {
                stock_quote.update_data();

                let lock_clients = clients.read().map_err(|e| ServerError::SendServer {
                    value: format!("Failed to acquire read lock for clients: {:?}", e),
                })?;
                for client in lock_clients.iter() {
                    // if client.alive && client.ticker.iter().any(|el| *el == stock_quote.ticker) {
                    if client.ticker.iter().any(|el| *el == stock_quote.ticker) {
                        if let Some(cl) = &client.ts {
                            let str_stock_quote = serde_json::to_string(&stock_quote).map_err(|er| {
                                ServerError::SendServer { value: format!("Error: StockQuote to json: {}", er) }
                            })?;
                            _ = cl.send(str_stock_quote);
                        }
                    }
                }
            }

            thread::sleep(interval);
        }
    }
}
