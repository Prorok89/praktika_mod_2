use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    sync::{Arc, Mutex, RwLock},
    thread,
};

use crate::error::ServerError;

mod error;
mod generate_data;
mod model;

use clap::Parser;
use model::{Cli, Client};
use url::Url;

const CORRECT_COMMAND: &str = "correct command: STREAM udp://<host>:<post> <ticker1,ticker2>";

fn main() -> Result<(), ServerError> {
    let cli = Cli::parse();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", cli.port)).unwrap();
    println!("Server listening on port {}", cli.port);

    // Чтенеи файла с тикерами и запись в массив

    let mut tickers: Vec<String> = Vec::new();
    let clients: Vec<Client> = Vec::new();

    parse_file_tickers(&cli.file_path, &mut tickers)?;

    let arc_tickers = Arc::new(RwLock::new(tickers));
    let arc_clients = Arc::new(Mutex::new(clients));
    // let vault = Arc::new(Mutex::new(Vault::new(10))); // лимит 10 ячеек

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let arc_tickers_clone = Arc::clone(&arc_tickers);
                let arc_clients_clone = Arc::clone(&arc_clients);
                thread::spawn(move || {
                    if let Err(er) = handle_client(stream, arc_tickers_clone, arc_clients_clone) {
                        println!("{}", er);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
        println!("2 yes");
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
    clients: Arc<Mutex<Vec<Client>>>,
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
                let client_create = parse_command(&line, &tickers, &clients).map_err(|er| {
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
    };
    Ok(())
}

fn parse_command(
    line: &str,
    tickers: &Arc<RwLock<Vec<String>>>,
    clients: &Arc<Mutex<Vec<Client>>>,
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
        }
    }

    {
        let mut data_clients = clients.lock().map_err(|er| ServerError::SendServer {
            value: format!("Failed to acquire read lock for tickers: {:?}", er),
        })?;
        
        for data_client in data_clients.iter() {
            if *data_client == client {
                return  Err(ServerError::SendServer { value: "A stream with these settings has already been launched".to_string() });
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
