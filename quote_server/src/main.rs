use core::time;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    sync::{Arc, RwLock, mpsc},
    thread,
};

use crate::{error::ServerError, generate_data::StockQuote};

mod error;
mod generate_data;
mod model;

use clap::Parser;
use model::{Cli, Client};
use url::Url;

const CORRECT_COMMAND: &str = "correct command: STREAM udp://<host>:<post> <ticker1,ticker2>";

fn main() {
    if let Err(e) = start_server() {
        eprint!("{:?}", e);
    }
}

fn start_server() -> Result<(), ServerError> {
    let cli = Cli::parse();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", cli.port)).unwrap();
    println!("Server listening on port {}", cli.port);

    let mut tickers: Vec<String> = Vec::new();
    let clients: Vec<Client> = Vec::new();
    let stock_quote: Vec<StockQuote> = Vec::new();

    parse_file_tickers(&cli.file_path, &mut tickers)?;

    let (quote_sender, quote_receiver) = mpsc::channel::<String>();

    let arc_tickers: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(tickers));
    let arc_clients: Arc<RwLock<Vec<Client>>> = Arc::new(RwLock::new(clients));
    let arc_stock_quote: Arc<RwLock<Vec<StockQuote>>> = Arc::new(RwLock::new(stock_quote));

    let clone_stock_quote_to_process: Arc<RwLock<Vec<StockQuote>>> = Arc::clone(&arc_stock_quote);

    thread::spawn(move || {
        _ = process_quotes(quote_receiver, clone_stock_quote_to_process);
    });

    let clone_stock_quote_to_sending: Arc<RwLock<Vec<StockQuote>>> = Arc::clone(&arc_stock_quote);
    let clone_clients_to_sending: Arc<RwLock<Vec<Client>>> = Arc::clone(&arc_clients);

    thread::spawn(move || {
        _ = sending_data_to_clients(clone_stock_quote_to_sending, clone_clients_to_sending)
            .map_err(|e| {
                println!("{}", e);
            });
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let arc_tickers_clone = Arc::clone(&arc_tickers);
                let arc_clients_clone = Arc::clone(&arc_clients);
                let sender_quote_clone = quote_sender.clone();
                thread::spawn(move || {
                    if let Err(er) = handle_client(
                        stream,
                        arc_tickers_clone,
                        arc_clients_clone,
                        sender_quote_clone,
                    ) {
                        println!("{}", er);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    println!("!");
    Ok(())
}

fn sending_data_to_clients(
    stock_quote: Arc<RwLock<Vec<StockQuote>>>,
    clients: Arc<RwLock<Vec<Client>>>,
) -> Result<(), ServerError> {
    loop {
        {
            println!("Start");

            println!("Satrt 1");
            let data_clients = clients.read().map_err(|er| ServerError::SendServer {
                value: format!("Failed to acquire read lock for clients: {:?}", er),
            })?;
            println!("Satrt 2");

            if !data_clients.is_empty() {
                let stock_quote_data = stock_quote.read().map_err(|e| {
                    println!("Start 01: {:?}", e);
                    ServerError::SendServer {
                        value: format!("Failed to acquire read lock for stock_quote: {:?}", e),
                    }
                })?;

                for data_client in data_clients.iter() {
                    for q in stock_quote_data.iter() {
                        println!("{} - {:?}", data_client.port, q);
                    }
                }
            }
            println!("End");
        }
        thread::sleep(time::Duration::from_secs(2));
    }

    Ok(())
}

fn process_quotes(
    receiver: mpsc::Receiver<String>,
    stock_quote: Arc<RwLock<Vec<StockQuote>>>,
) -> Result<(), ServerError> {
    for quote in receiver {
        let mut data_stock_quote = stock_quote.write().map_err(|er| ServerError::SendServer {
            value: format!("Failed to acquire read lock for stock_quote: {:?}", er),
        })?;
        if !data_stock_quote
            .iter()
            .any(|sq| sq.ticker == quote.to_string())
        {
            data_stock_quote.push(StockQuote::new(quote.to_string()));
        }
        println!("{:?}", data_stock_quote);
    }


    Ok(())
}

fn parse_file_tickers(path: &str, tickers: &mut Vec<String>) -> Result<(), ServerError> {
    let path = Path::new(path);

    if path.exists() {
        let mut file = File::open(path).map_err(ServerError::IoError)?;

        let buf = BufReader::new(file);

        for line in buf.lines() {
            match line {
                Ok(l) => {
                    tickers.push(l);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    } else {
        println!("file not found: {:?}", path);
    }

    Ok(())
}

fn handle_client(
    stream: TcpStream,
    tickers: Arc<RwLock<Vec<String>>>,
    clients: Arc<RwLock<Vec<Client>>>,
    stock_quote: mpsc::Sender<String>,
) -> Result<(), ServerError> {
    let mut writer = stream.try_clone().expect("failed to clone stream");
    let mut reader = BufReader::new(stream);

    let _ = writer.write_all(b"Welcome to the QuoteServer!\n");
    let _ = writer.flush();

    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                return Err(ServerError::ConnectClosed);
            }
            Ok(_) => {
                let client_create =
                    parse_command(&line, &tickers, &clients, &stock_quote).map_err(|er| {
                        _ = write!(writer, "{}\n", er);
                        _ = writer.flush();
                    });

                if let Ok(_) = client_create {
                    break;
                }
            }
            Err(_) => {
                return Err(ServerError::ConnectClosed);
            }
        }
    }
    Ok(())
}

fn parse_command(
    line: &str,
    tickers: &Arc<RwLock<Vec<String>>>,
    clients: &Arc<RwLock<Vec<Client>>>,
    sender_quote: &mpsc::Sender<String>,
) -> Result<bool, ServerError> {
    let iter: Vec<&str> = line.split_ascii_whitespace().collect();

    if iter.len() != 3 {
        return Err(ServerError::SendServer {
            value: CORRECT_COMMAND.to_string(),
        });
    }

    if iter[0] != "STREAM" {
        return Err(ServerError::SendServer {
            value: CORRECT_COMMAND.to_string(),
        });
    }

    let mut client: Client = Client::new();

    validate_udp_address(iter[1], &mut client)?;

    let ticker_str = iter[2];
    let ticker_list: Vec<&str> = ticker_str.split(",").collect();

    if ticker_list.len() > 0 {
        let ticker_data = tickers.read().map_err(|e| ServerError::SendServer {
            value: format!("Failed to acquire read lock for tickers: {:?}", e),
        })?;

        for ticker in ticker_list {
            if !ticker_data.contains(&ticker.to_string()) {
                return Err(ServerError::TickerNotFound(ticker.to_string()));
            }
            client.ticker.push(ticker.to_string());

            sender_quote.send(ticker.to_string());
        }
    }

    {
        let mut data_clients = clients.write().map_err(|er| ServerError::SendServer {
            value: format!("Failed to acquire read lock for clients: {:?}", er),
        })?;

        for data_client in data_clients.iter() {
            if data_client.adress == client.adress && data_client.port == client.port {
                return Err(ServerError::SendServer {
                    value: "A stream with these settings has already been launched".to_string(),
                });
            }
        }

        data_clients.push(client);
    }

    Ok(true)
}

fn validate_udp_address(address: &str, client: &mut Client) -> Result<(), ServerError> {
    if !address.starts_with("udp://") {
        return Err(ServerError::SendServer {
            value: "Invalid UDP address format. Expected: udp://<host>:<port>".to_string(),
        });
    }

    match Url::parse(address) {
        Ok(url) => {
            if url.scheme() != "udp" {
                return Err(ServerError::SendServer {
                    value: "Invalid UDP address format. Scheme must be udp".to_string(),
                });
            }

            match url.host() {
                Some(host) => {
                    client.adress = host.to_string();
                }
                None => {
                    return Err(ServerError::SendServer {
                        value: "Invalid UDP address format. Host is missing".to_string(),
                    });
                }
            }

            match url.port() {
                Some(port) => {
                    client.port = port;
                }
                None => {
                    return Err(ServerError::SendServer {
                        value: "Invalid UDP address format. Port is missing".to_string(),
                    });
                }
            }

            Ok(())
        }
        Err(_) => {
            return Err(ServerError::SendServer {
                value: "Invalid UDP address format. Cannot parse URL".to_string(),
            });
        }
    }
}
