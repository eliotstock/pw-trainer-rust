use std::env;
use std::fs;

extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use std::io::{self, BufRead};

const PW_HASH_FILE: &str = "./.pw-trainer";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage(&args[0]);
        std::process::exit(1);
    }

    let op = &args[1];

    println!("Enter password:");

    let stdin = io::stdin();
    let pw = stdin.lock().lines().next().unwrap().unwrap();

    match op.as_str() {
        "set" => {
            let hash = hash(pw, DEFAULT_COST).unwrap();

            // This works even if the file already exists.
            fs::write(PW_HASH_FILE, hash)
                .expect("Couldn't write hash file.");
        },
        "check" => {
            let hash = fs::read_to_string(PW_HASH_FILE)
                .expect("No hash file found. Use \"set\" first.");

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
            std::process::exit(1);
        }
    }
}

fn usage(executable: &str) {
    println!("Usage: {} [set|check]", executable);
}
