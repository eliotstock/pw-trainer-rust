use std::env;

extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let op = &args[1];
    let pw = &args[2];

    match op.as_str() {
        "set" => {
            let hashed = hash("foo", DEFAULT_COST).unwrap();

            println!("TODO: Put the hash in storage.");
        },
        "check" => {
            // TODO: Get the hash of the pw from storage.
            let hashed = hash("foo", DEFAULT_COST).unwrap();

            let valid = verify(pw, &hashed).unwrap();
            println!("Valid: {}", valid);
        },
        _ => {
            println!("Not an operation: {}.", op);
        }
    }
}
