use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
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

fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", cli.port)).unwrap();
    println!("Server listening on port {}", cli.port);

    // let vault = Arc::new(Mutex::new(Vault::new(10))); // лимит 10 ячеек

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // let vault = Arc::clone(&vault);
                thread::spawn(move || {
                    if let Err(er) = handle_client(stream) {
                        println!("{}", er);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> Result<(), ServerError> {
    let mut writer = stream.try_clone().expect("failed to clone stream");
    let mut reader = BufReader::new(stream);

    let _ = writer.write_all(b"Welcome to the Vault!\n");
    let _ = writer.flush();

    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                return Err(ServerError::ConnectClosed);
            }
            Ok(_) => {
                match parse_command(&line) {
                    Err(er) => {
                        _ = write!(writer, "{}\n", er);
                        _ = writer.flush();
                    }
                    _ => println!("While I dont to do"),
                }
                println!("{}", line);
            }
            Err(_) => {
                return Err(ServerError::ConnectClosed);
            }
        }
    }
}

fn parse_command(line: &str) -> Result<(), ServerError> {
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

    println!("port - {}, address - {}", client.port, client.adress);

    Ok(())
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
