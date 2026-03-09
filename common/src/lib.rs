use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    net::ToSocketAddrs,
};

use url::Url;

use crate::error::CommonError;

pub mod error;
pub mod model;

pub fn parse_file_tickers(path: &str) -> Result<Vec<String>, io::Error> {
    let mut tickers: Vec<String> = Vec::new();

    let file = File::open(path)?;

    let buf = BufReader::new(file);

    for line in buf.lines() {
        if let Ok(l) = line {
            tickers.push(l);
        }
    }

    Ok(tickers)
}

pub fn validate_udp_address(address: &str) -> Result<(String, u16), CommonError> {
    if !address.starts_with("udp://") {
        return Err(CommonError::CommonError(
            "Invalid UDP address format. Expected: udp://<host>:<port>".to_string(),
        ));
    }

    let mut res_host: String = String::new();
    let mut res_port: u16 = 0;

    match Url::parse(address) {
        Ok(url) => {
            if url.scheme() != "udp" {
                return Err(CommonError::CommonError(
                    "Invalid UDP address format. Scheme must be udp".to_string(),
                ));
            }

            match url.host() {
                Some(host) => {
                    res_host = host.to_string();
                }
                None => {
                    return Err(CommonError::CommonError(
                        "Invalid UDP address format. Host is missing".to_string(),
                    ));
                }
            }

            match url.port() {
                Some(port) => {
                    res_port = port;
                }
                None => {
                    return Err(CommonError::CommonError(
                        "Invalid UDP address format. Port is missing".to_string(),
                    ));
                }
            }

            Ok((res_host, res_port))
        }
        Err(_) => {
            return Err(CommonError::CommonError(
                "Invalid UDP address format. Cannot parse URL".to_string(),
            ));
        }
    }
}

pub fn validate_tcp_address(address: &str) -> Result<String, CommonError> {
    let correct_address = address
        .to_socket_addrs()
        .map_err(|_| CommonError::CommonError("Addess not correcr".to_string()))?
        .next()
        .ok_or_else(|| CommonError::CommonError("Addess not correcr".to_string()))?;

    Ok(correct_address.to_string())
}
