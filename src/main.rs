use std::env;
use std::process;
use std::io::{self, Read};
extern crate base64;

use base64::decode;

fn main() {
    if env::args().len() != 2 {
        usage("expected exactly one argument");
        process::exit(1);
    }

    let arg = env::args().nth(1).unwrap();
    if arg == "--help" {
        usage("decodes JSON web tokens");
        process::exit(0);
    }
    
    let res;
    // Support for piping from stdin
    if arg == "-" {
        res = decode_token_stdin();
    } else {
        res = decode_token(&arg);
    }

    match res {
        Ok(raw) => println!("{}", raw),
        Err(err) => {
            println!("failed to decode token: {:?}", err);
            process::exit(1);
        }
    }
}

#[derive(Debug)]
enum DecodeError {
    StringFormat(String),
    Base64(base64::DecodeError),
    StringEncoding(std::string::FromUtf8Error),
    JSONDecoding(serde_json::error::Error),
    IoError(std::io::Error),
}

impl From<io::Error> for DecodeError {
    fn from(error: io::Error) -> Self {
        DecodeError::IoError(error)
    }
}

// Decodes a token string from stdin
fn decode_token_stdin() -> Result<String, DecodeError> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    decode_token(&buffer)
}

// Decodes a token string into its JSON string representation
fn decode_token(token: &str) -> Result<String, DecodeError> {
    let parts: Vec<&str> = token.split(".").collect();
    if parts.len() != 3 {
        return Err(DecodeError::StringFormat(String::from(
            format!("expected a well-formed JWT, got: {}", token),
        )));
    }
    let header_b64 = parts[0];
    let claims_b64 = parts[1];
    let header_data = decode_part(header_b64)?;
    let claims_data = decode_part(claims_b64)?;
    Ok(format!("{} {}", header_data, claims_data))
}

// Decodes an individual JWT component into its JSON representation
fn decode_part(part_b64: &str) -> Result<serde_json::Value, DecodeError> {
    match decode(part_b64) {
        Ok(part) => match String::from_utf8(part) {
            Ok(decoded) => match serde_json::from_str(&decoded) {
                Ok(j) => Ok(j),
                Err(err) => Err(DecodeError::JSONDecoding(err)),
            },
            Err(err) => Err(DecodeError::StringEncoding(err)),
        },
        Err(err) => Err(DecodeError::Base64(err)),
    }
}

fn usage(msg: &str) {
    let cmd = env::args().nth(0).unwrap();
    println!("{}: {}", cmd, msg);
    println!("usage: {} token-string", cmd)
}
