
use std::{collections::HashMap, error::Error, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread};

use crate::error::ServerError;

mod generate_data;
mod error;
mod model;

use clap::Parser;
use model::Cli;


fn main() ->Result<(), ()> {
    let cli = Cli::parse();

    println!("{}", cli.port);

    

    let listener = TcpListener::bind(format!("127.0.0.1:{}", cli.port)).unwrap();
    println!("Server listening on port 7878");

    // let vault = Arc::new(Mutex::new(Vault::new(10))); // лимит 10 ячеек

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // let vault = Arc::clone(&vault);
                thread::spawn(move || {
                    if let Err(er)  = handle_client(stream) {
                        println!("{}", er);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> Result<(),ServerError> {
    
    let mut writer = stream.try_clone().expect("failed to clone stream");
    let mut reader = BufReader::new(stream);

    let _ = writer.write_all(b"Welcome to the Vault!\n");
    let _ = writer.flush();

    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                println!("ERROR");
            }
            Ok(_) => {

                let mut iter: Vec<&str> = line.split_ascii_whitespace().collect();

                if iter.len() == 3 {
                    
                    
                    if iter[0] != "STREAM" {
                        return Err(ServerError::CommandFormat);
                    }
                }
                else
                {
                    // error
                    return Err(ServerError::CommandFormat);
                }

                println!("{}", line);
            }
            Err(_) => {
                println!("ERROR")
            }
        }
    }
 
}


fn parse_command() -> Result<(), ServerError>{



    Ok(())
}