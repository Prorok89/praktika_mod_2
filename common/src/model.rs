use std::{fmt::Display, time::{SystemTime, UNIX_EPOCH}};
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockQuote {
    pub ticker: String,
    pub price: f64,
    pub volume: u32,
    pub timestamp: u64,
}

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

    pub fn update_data(&mut self) {
        if self.volume > 0 {
            let quantity_coefficient: f64 = rand::random_range(0.7..1.1);
            self.volume = (self.volume * ((quantity_coefficient * 100.0) as u32)) / 100;
        } else {
            let quantity_coefficient: u32 = rand::random_range(1000..2000);
            self.volume = quantity_coefficient;
        }

        if self.price > 0.0 {
            let cost_coefficient: f64 = rand::random_range(0.9..1.1);
            self.price *= cost_coefficient;
        } else {
            self.price = 1.0;
        }

        self.timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
    }

}

impl Display for StockQuote{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = match DateTime::from_timestamp((self.timestamp / 1000) as i64, 0) {
            Some(d) => d.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => "".to_string()
        };

        write!(f, "{}\t{}\t{:<.5}\t{}", self.ticker, self.volume, self.price, date)
    }
}
