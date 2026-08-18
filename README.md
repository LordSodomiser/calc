# Abyssal Abacus

A simple command-line calculator written in Rust.

## Requirements

* [Rust](https://www.rust-lang.org/tools/install)
* Cargo (included with Rust)

## Building

Clone the repository and build the project:

```bash
git clone https://github.com/LordSodomiser/calc.git
cd calc
cargo build --release
```

The compiled binary will be located at:

```text
target/release/calc
```

## Running

Run the calculator directly with Cargo:

```bash
cargo run --release
```

Or run the compiled binary:

```bash
./target/release/calc
```

## Installing locally

To make `calc` available system-wide:

```bash
sudo cp target/release/calc /usr/local/bin/calc
```

You can then run it from anywhere:

```bash
calc
```

## Copying to a remote machine

If you want to copy the compiled binary to another machine:

```bash
scp target/release/calc user@othermachine:/usr/local/bin/calc
```

This requires appropriate SSH access and permissions on the remote machine.

## Exiting

To exit the calculator, enter:

```text
exit
```

or:

```text
quit
```
