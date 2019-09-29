# Chapter 01. Getting started

## Installation
- Tool: `rustup`, a command line tool for managing Rust versions and associated tools
### Installing rustup on Linux or macOS
- Execute following command in terminal
    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```

    If the install is successful, the following line will appear:

    ```bash
    Rust is installed now. Great!
    ```
- The installation script automatically adds Rust to your system PATH **after your next login**.
    To use Rust without restarting your terminal, run the manually:
    ```bash
    source $HOME/.cargo/env
    ```

> Some common Rust packages depend on C code and will need a C compiler

### Installing rustup on Windows 
- Go to [https://www.rust-lang.org/tools/install][install] and follow the instructions for
    installing Rust

### Updating and Uninstalling 
- Update 
    ```bash
    rustup update
    ```
- Uninstall
    ```bash
    rustup self uninstall
    ```
### Troubleshooting 
- Check rust version
    ```bash
    rustc --version
    ```
    should produce something of pattern
    ```bash
    rustc x.y.z (abcabcabc yyyy-mm-dd)
    ```
- Otherwise, ask for help on
  - [the official Rust Discord](https://discord.gg/rust-lang)
  - [the Users forum](https://users.rust-lang.org/)
  - [Stack Overflow](http://stackoverflow.com/questions/tagged/rust/)

### Local Documentation 
- Run `rustup doc` to open the local documentation in your browser 

## Hello, World!

### Writing and Running a Rust Program

- Convention: If you’re using more than one word in your filename, use an underscore to separate
    them
- Make a file named `main.rs` with content as 
    ```rust
    // main function is the entrypoint for running the program.
    // main no parameters and returns nothing
    fn main() { // the function body is wrapped in curly brackets
                // It's good style to place the opening curly bracket on the same line as the function
                // declaration, adding one space in between
        // Rust style is to indent with four spaces, not a tab
        // end the line with a semicolon (`;`), which indicates that this expression is over and
        // the next one is ready to begin
        println!("Hello, world");
    }
    ```
    The full project goes as [hello_world](./hello_world)
- `rustfmt` can format your code in a particular style

### Compiling and Running Are Separate Steps
- Compile with `rustc main.rs`
- Run with `./main # or .\main.exe on Windows`

## Hello, Cargo!
- Cargo is Rust’s build system and package manager, capable of 
  - building your code
  - downloading the libraries your code depends on
  - building those libraries

> Because the vast majority of Rust projects use Cargo, the rest of this book assumes that you're
> using Cargo too

- Check whether Cargo is installed with 
    ```bash
    cargo --version
    ```

### Creating a Project with Cargo
- Command `cargo new hello_cargo` would produce a Rust project named `hello_cargo`, with project 
  structure as 
    ```text
    |--hello_cargo
        |--Cargo.toml
        |--src
            |--main.rs
    ```

    where `Cargo.toml` goes as 

    ```toml
    [package]
    name = "hello_cargo"
    version = "0.1.0"
    authors = ["Your Name <you@example.com>"]
    edition = "2018"

    [dependencies]
    ```
    - `[package]` is a section heading that indicates that the following statements are
        configuring a package
    - `[dependencies]` is the start of a section for you to list any of your project's
        dependencies. In Rust, packages of code are referred to as *crates*.
    - The full project goes as [hello_cargo](./hello_cargo)

- For a non-cargo project to a cargo project, move the project code into the *src* directory and
    create an appropriate *Cargo.toml* file

### Building and Running a Cargo Project
- Build with `cargo build`
- Run with `cargo run`: compile the code and then run the resulting executable
- Check with `cargo check`: quickly checks your code to make sure it compiles but doesn't produce
    an executable

### Building for Release
- Command: `cargo build --release`
- If you're benchmarking your code's running time, be sure to run `cargo build --release` and 
    benchmark with the executable in *target/release*