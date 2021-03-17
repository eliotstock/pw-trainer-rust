use std::env;

extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        usage(&args[0]);
        std::process::exit(1);
    }

    let op = &args[1];
    let pw = &args[2];

    match op.as_str() {
        "set" => {
            let hash = hash(pw, DEFAULT_COST).unwrap();

            println!("TODO: Put the hash in storage.");
        },
        "check" => {
            // TODO: Get the hash of the pw from storage.
            let hash = "sdfklskdjlc";

            let valid = verify(pw, &hash).unwrap();

            if valid {
                println!("Password correct");
            }
            else {
                println!("Password INCORRECT");
            }
        },
        "--help" => {
            usage(&args[0]);
            std::process::exit(1);
        },
        _ => {
            println!("Not an operation: {}.", op);
        }
    }
}

fn usage(executable: &str) {
    println!("Usage: {} [set|check] password", executable);
}
