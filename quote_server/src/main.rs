use core::time;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream, UdpSocket},
    sync::{
        Arc, RwLock,
        mpsc::{self, Receiver},
    },
    thread, time::Duration,
};

use crate::{error::ServerError, generate_data::QuoteGenerator};

use common::model::StockQuote;

mod error;
mod generate_data;
mod model;

use clap::Parser;
use model::{Cli, Client};
use url::Url;

const CORRECT_COMMAND: &str = "correct command: STREAM udp://<host>:<port> <ticker1,ticker2>";

fn main() {
    if let Err(e) = start_server() {
        eprint!("{:?}", e);
    }
}

fn start_server() -> Result<(), ServerError> {
    let cli: Cli = Cli::parse();

    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", cli.port)).map_err(ServerError::IoError)?;
    println!("Server listening on port {}", cli.port);

    let interval = cli.interval;

    let clients: Vec<Client> = Vec::new();
    let stock_quote: Vec<StockQuote> = Vec::new();

    let tickers: Vec<String> =
        common::parse_file_tickers(&cli.file_path).map_err(ServerError::IoError)?;

    let (quote_sender, quote_receiver) = mpsc::channel::<String>();

    let arc_tickers: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(tickers));
    let arc_clients: Arc<RwLock<Vec<Client>>> = Arc::new(RwLock::new(clients));
    let arc_stock_quote: Arc<RwLock<Vec<StockQuote>>> = Arc::new(RwLock::new(stock_quote));

    let arc_stock_quote_clone: Arc<RwLock<Vec<StockQuote>>> = Arc::clone(&arc_stock_quote);
    let arc_clients_clone: Arc<RwLock<Vec<Client>>> = Arc::clone(&arc_clients);
    thread::spawn(move || {
        QuoteGenerator::generate_multiple(
            arc_stock_quote_clone,
            arc_clients_clone,
            time::Duration::from_secs(interval.into()),
        )
    });

    let clone_stock_quote_to_process: Arc<RwLock<Vec<StockQuote>>> = Arc::clone(&arc_stock_quote);
    thread::spawn(move || {
        _ = process_quotes(quote_receiver, clone_stock_quote_to_process);
    });

    // let clone_clients_to_sending: Arc<RwLock<Vec<Client>>> = Arc::clone(&arc_clients);

    // thread::spawn(move || {
    //     _ = sending_data_to_clients(clone_stock_quote_to_sending, clone_clients_to_sending)
    //         .map_err(|e| {
    //             println!("{}", e);
    //         });
    // });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let arc_tickers_clone = Arc::clone(&arc_tickers);
                let arc_clients_clone = Arc::clone(&arc_clients);
                let arc_clients_clone_wtite = Arc::clone(&arc_clients);

                let sender_quote_clone = quote_sender.clone();
                thread::spawn(move || {
                    match handle_client(
                        stream,
                        arc_tickers_clone,
                        arc_clients_clone,
                        sender_quote_clone,
                    ) {
                        Err(er) => println!("{}", er),
                        Ok(mut client) => match arc_clients_clone_wtite.write() {
                            Err(e) => {
                                ServerError::SendServer {
                                    value: format!(
                                        "Failed to acquire read lock for clients: {:?}",
                                        e
                                    ),
                                };
                            }
                            Ok(mut data_clients) => {
                                let (ts, tr) = mpsc::channel::<String>();
                                client.ts = Some(ts);
                                let address = format!("{}:{}", client.adress, client.port);
                                thread::spawn(move || create_udp_connect(address, tr));
                                data_clients.push(client);
                            }
                        },
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    println!("!");
    Ok(())
}

fn create_udp_connect(address: String, tr: Receiver<String>) -> Result<(), ServerError> {
    // Связываем сокет с локальным портом (127.0.0.1:0 - случайный свободный порт)
    let socket = UdpSocket::bind("127.0.0.1:0").map_err(ServerError::IoError)?;

    for quote in tr {
        match socket.send_to(format!("{}\n", quote).as_bytes(), &address) {
            Ok(bytes_sent) => println!("Sent {} bytes: {} to {} \n", bytes_sent, quote, address),
            Err(e) => println!("Send error to {}: {}\n", address, e),
        }
    }

    thread::spawn(move || {

        loop {
            thread::sleep(Duration::from_secs(2));
            
            match socket.send_to(b"PING", &address) {
                Ok(_) => println!("Sent PING to {}", address),
                Err(e) => eprintln!("Failed to send PING: {}", e),
            }
        }
    });

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
            data_stock_quote.push(StockQuote::new(&quote));
        }
    }

    Ok(())
}

fn handle_client(
    stream: TcpStream,
    tickers: Arc<RwLock<Vec<String>>>,
    clients: Arc<RwLock<Vec<Client>>>,
    stock_quote: mpsc::Sender<String>,
) -> Result<Client, ServerError> {
    let mut writer = stream.try_clone().expect("failed to clone stream");
    let mut reader = BufReader::new(stream);

    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                return Err(ServerError::ConnectClosed);
            }
            Ok(_) => {
                let client = parse_command(&line, &tickers, &clients, &stock_quote).map_err(|er| {
                    _ = write!(writer, "{}\n", er);
                    _ = writer.flush();
                });

                if let Ok(client_ok) = client {
                    _ = write!(writer, "OK");
                    _ = writer.flush();
                    return Ok(client_ok);
                }
            }
            Err(_) => {
                return Err(ServerError::ConnectClosed);
            }
        }
    }
}

fn parse_command(
    line: &str,
    tickers: &Arc<RwLock<Vec<String>>>,
    clients: &Arc<RwLock<Vec<Client>>>,
    sender_quote: &mpsc::Sender<String>,
) -> Result<Client, ServerError> {
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

    (client.adress, client.port) =
        common::validate_udp_address(iter[1]).map_err(|e| ServerError::SendServer {
            value: format!("{}", e),
        })?;

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

            _ = sender_quote.send(ticker.to_string());
        }
    }

    {
        let data_clients = clients.read().map_err(|er| ServerError::SendServer {
            value: format!("Failed to acquire read lock for clients: {:?}", er),
        })?;

        for data_client in data_clients.iter() {
            if data_client.adress == client.adress && data_client.port == client.port {
                return Err(ServerError::SendServer {
                    value: "A stream with these settings has already been launched".to_string(),
                });
            }
        }
    }

    Ok(client)
}
