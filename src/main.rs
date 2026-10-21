use std::env;
use std::fs;
use std::process;

extern crate bcrypt;
extern crate rpassword;

use bcrypt::{DEFAULT_COST, hash, verify};
use rpassword::prompt_password;

const PW_HASH_FILE: &str = "./.pw-trainer";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage(&args[0]);
        process::exit(1);
    }

    match args[1].as_str() {
        "set" => {
            // Input is read from the terminal with echo off, so it never lands in
            // scrollback. Entered twice: a typo here would train the wrong password.
            let pw = read_pw("Enter password: ");
            let again = read_pw("Enter it again: ");

            if pw != again {
                println!("Entries differ. Nothing written.");
                process::exit(1);
            }

            if pw.is_empty() {
                println!("Empty password. Nothing written.");
                process::exit(1);
            }

            let hash = hash(pw, DEFAULT_COST).unwrap();

            // This works even if the file already exists.
            fs::write(PW_HASH_FILE, hash)
                .expect("Couldn't write hash file.");

            println!("Hash written to {}.", PW_HASH_FILE);
        },
        "check" => {
            let hash = fs::read_to_string(PW_HASH_FILE)
                .expect("No hash file found. Use \"set\" first.");

            let pw = read_pw("Enter password: ");

            if verify(pw, hash.trim()).unwrap() {
                println!("Password correct");
            }
            else {
                println!("Password INCORRECT");
                process::exit(1);
            }
        },
        "--help" => {
            usage(&args[0]);
            process::exit(1);
        },
        op => {
            println!("Not an operation: {}.", op);
            process::exit(1);
        }
    }
}

fn read_pw(prompt: &str) -> String {
    prompt_password(prompt).expect("Couldn't read password from the terminal.")
}

fn usage(executable: &str) {
    println!("Usage: {} [set|check]", executable);
}
