use std::env;
use std::process;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;

use jwt::dangerous_unsafe_decode;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

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

    let res = decode_token(&arg);
    match res {
        Ok(claims) => println!("{}", claims),
        Err(err) => {
            println!("failed to decode token: {}", err);
            process::exit(1);
        }
    }
}

// Decodes a token string into its JSON string representation
fn decode_token(token: &str) -> Result<String, jwt::errors::Error> {
    let token_data = dangerous_unsafe_decode::<Claims>(&token)?;
    Ok(format!("{:?}", token_data))
}

fn usage(msg: &str) {
    let cmd = env::args().nth(0).unwrap();
    println!("{}: {}", cmd, msg);
    println!("usage: {} token-string", cmd)
}
