# pw-trainer-rust
Password training app, CLI, Rust

Practise recalling a password without keeping it anywhere readable. `set` stores
only a bcrypt hash (cost 12) in `./.pw-trainer`; `check` tells you whether what you
typed matches.

```sh
pw-trainer-rust set     # asks twice; writes nothing unless both entries match
pw-trainer-rust check   # prints "Password correct" (exit 0) or "INCORRECT" (exit 1)
```

Input is read from the terminal with echo off, so the password never appears on
screen or in scrollback, and never goes on the command line (shell history).

## Building for an offline machine

`Cargo.lock` is committed and pins every dependency with a checksum. Build while
the machine still has network, then take it offline:

```sh
cargo build --release --locked
./target/release/pw-trainer-rust check
```

To build after going offline instead, run `cargo vendor` while online and follow
the config snippet it prints.

## Setting a password you must get exactly right

`set` cannot know whether what you typed is the real password, only that you typed
the same thing twice. If there is an external checksum for it (e.g. a hardware
wallet's receive address for a BIP-39 passphrase), confirm against that first,
then `set` immediately with the same entry.
