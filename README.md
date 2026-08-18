# Abyssal Abacus

A simple command-line calculator written in Rust.

Abyssal Abacus supports basic arithmetic operations using two numeric operands.

## Features

* Addition (`+`)
* Subtraction (`-`)
* Multiplication (`*`)
* Division (`/`)
* Modulo (`%`)
* Decimal numbers
* Negative numbers
* Case-insensitive `exit` and `quit` commands
* Friendly error messages for invalid input and division/modulo by zero

## Download

Pre-built binaries are available on the
[Releases](https://github.com/LordSodomiser/Abyssal-Abacus/releases) page.

### Linux x86-64

Download `calc-linux-x86_64.tar.xz`, then extract it:

```bash
tar -xJf calc-linux-x86_64.tar.xz
```

Run the calculator:

```bash
./calc
```

If necessary, make the binary executable:

```bash
chmod +x calc
```

To install it system-wide:

```bash
sudo cp calc /usr/local/bin/calc
```

You can then run it from anywhere:

```bash
calc
```

### Windows x86-64

Download `calc-windows-x86_64.zip` and extract it.

From PowerShell:

```powershell
.\calc.exe
```

Or from Command Prompt:

```cmd
calc.exe
```

## Usage

Start the calculator and enter an expression:

```text
> 10+5
15

> 10-3
7

> 6*7
42

> 20/4
5

> 10%3
1
```

Decimal numbers are supported:

```text
> 10.5+2.5
13
```

Negative numbers are supported:

```text
> -5+3
-2

> 5*-3
-15
```

To exit the calculator:

```text
> exit
Goodbye!
```

You can also use:

```text
> quit
Goodbye!
```

## Error Handling

Invalid expressions return an error instead of crashing the calculator:

```text
> hello
Error: Invalid expression
```

Division and modulo by zero are also rejected:

```text
> 10/0
Error: Cannot divide by zero
```

## Building from Source

### Requirements

* [Rust](https://www.rust-lang.org/tools/install)
* Cargo, which is included with Rust

Clone the repository:

```bash
git clone https://github.com/LordSodomiser/calc.git
cd calc
```

Build a release version:

```bash
cargo build --release
```

The resulting binary will be located at:

```text
target/release/calc
```

Run it directly:

```bash
./target/release/calc
```

## Project Structure

```text
Abyssal-Abacus/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── src/
│   └── main.rs
└── .gitignore
```

Build artifacts and release packages are not tracked in Git:

```text
target/
dist/
```

Pre-built binaries are distributed through GitHub Releases.

## License

See the [LICENSE](LICENSE) file for license information.
