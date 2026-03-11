use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpStream, UdpSocket},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use clap::Parser;
use common::model::StockQuote;

use crate::{error::ClientError, model::Cli};

mod error;
mod model;
mod test;

fn main() {
    env_logger::init();
    if let Err(e) = start_client() {
        log::error!("Client error: {:?}", e);
    }
}

fn start_client() -> Result<(), ClientError> {
    let cli: Cli = Cli::parse();

    let tickers: Vec<String> =
        common::parse_file_tickers(&cli.file_path).map_err(ClientError::IoError)?;

    let (udp_address, udp_port)  =
        common::validate_udp_address(&cli.udp_server).map_err(|e| ClientError::SendServer {
            value: format!("{}", e),
        })?;

    let tsp_address =
        common::validate_tcp_address(&cli.tcp_server).map_err(|e| ClientError::SendServer {
            value: format!("{}", e),
        })?;

    let mut stream = TcpStream::connect(&tsp_address).map_err(|e| ClientError::SendServer {
        value: e.to_string(),
    })?;

    let command = format!("STREAM udp://{}:{} {}", udp_address, udp_port, tickers.join(","));

    stream
        .write_all(command.as_bytes())
        .map_err(|er| ClientError::SendServer {
            value: er.to_string(),
        })?;
    stream
        .write_all(b"\n")
        .map_err(|er| ClientError::SendServer {
            value: er.to_string(),
        })?;

    // Чтение ответа
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader
        .read_line(&mut response)
        .map_err(|er| ClientError::SendServer {
            value: er.to_string(),
        })?;

    if response == "OK" {
        let socket = UdpSocket::bind(format!("{}:{}", udp_address, udp_port)).map_err(|er| {
            ClientError::SendServer {
                value: er.to_string(),
            }
        })?;

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .map_err(|er| ClientError::SendServer {
            value: er.to_string(),
        })?;

        let mut buf = [0; 1024];

        while running.load(Ordering::SeqCst) {
            match socket.recv_from(&mut buf) {
                Ok((len, addr)) => {
                    let data = String::from_utf8_lossy(&buf[..len]);
                    if data.trim() == "PING" {
                        socket
                            .send_to(b"PONG", addr)
                            .map_err(|er| ClientError::SendServer {
                                value: er.to_string(),
                            })?;
                    } else {
                        let s = serde_json::from_str::<StockQuote>(&data).map_err(|er| {
                            ClientError::SendServer {
                                value: er.to_string(),
                            }
                        })?;
                        println!("{}", s)
                    }
                }
                Err(e) => {
                    log::error!("UDP error: {}", e);
                }
            }
        }
    }

    Ok(())
}
