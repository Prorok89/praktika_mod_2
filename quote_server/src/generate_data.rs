use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use common::model::StockQuote;

use crate::{error::ServerError, model::Client};

pub struct QuoteGenerator;

impl QuoteGenerator {
    pub fn generate_multiple(
        stock_quote: Arc<RwLock<Vec<StockQuote>>>,
        clients: Arc<RwLock<Vec<Client>>>,
        interval: Duration,
    ) -> Result<(), ServerError> {
        loop {
            let mut lock_stock_quotes =
                stock_quote.write().map_err(|e| ServerError::SendServer {
                    value: format!("Failed to acquire read lock for tickers: {:?}", e),
                })?;

            {
                let mut lock_clients = clients.write().map_err(|e| ServerError::SendServer {
                    value: format!("Failed to acquire read lock for clients: {:?}", e),
                })?;
                lock_clients.retain(|c| c.alive.load(std::sync::atomic::Ordering::Relaxed));
            }

            for stock_quote in lock_stock_quotes.iter_mut() {
                stock_quote.update_data();

                let lock_clients = clients.read().map_err(|e| ServerError::SendServer {
                    value: format!("Failed to acquire read lock for clients: {:?}", e),
                })?;

                for client in lock_clients.iter() {
                    if client.ticker.contains(&stock_quote.ticker)
                        && let Some(cl) = &client.ts
                    {
                        let str_stock_quote =
                            serde_json::to_string(&stock_quote).map_err(|er| {
                                ServerError::SendServer {
                                    value: format!("Error: StockQuote to json: {}", er),
                                }
                            })?;
                        _ = cl.send(str_stock_quote);
                    }
                }
            }

            thread::sleep(interval);
        }
    }
}
