# Chapter 02. Programming a Guessing Game

A guessing game
- the program will generate a random integer between 1 and 100
- It will then prompt the player to enter a guess
- After a guess is entered, the program will indicate whether the guess is too low or too high
- If the guess is correct, the game will print a congratulatory message and exit.

## Setting Up a New Project 

```bash
cargo new guessing_game
cd guessing_game
```


## Processing a Guess 
- Task
  - Ask for user input
  - Process that input
  - Check that the input is in the expected form
- [Example](https://github.com/sammyne/the-rust-programming-language/commit/8b4ce81dc972eb4f1d0da44afc789dc8e5f2ed61)

### Storing Values with Variables 
- A `let` statement, which is used to create a *variable*
- Variables are immutable by default
> The `//` syntax starts a comment that continues until the end of the line
- The `::` syntax in the `::new` line indicates that `new` is an *associated
function* (a.k.a *static methods* in other languages) of the `String` type
- The `&` indicates that this argument is a *reference*, which gives you a way to
let multiple parts of your code access one piece of data without needing to
copy that data into memory multiple times
### Handling Potential Failure with the Result Type 
- When you call a method with the `.foo()` syntax, it’s often wise to introduce a
newline and other whitespace to help break up long lines
- `read_line` puts what the user types into the string we're passing it, but it also returns a value—in this case, an `io::Result`
    - The `Result` types are *enumerations*, often referred to as *enums*
    - An enumeration is a type that can have a fixed set of values, and those values are called the enum's *variants*
    - For `Result`, the variants are `Ok` or `Err`
      - The `Ok` variant indicates the operation was successful, and inside `Ok` is the successfully generated value
      - The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed
    - The purpose of these `Result` types is to encode error-handling information
    - An instance of `io::Result` has an `expect` method that you can call. If this instance of `io::Result` is an `Err` value, `expect` will cause the program to crash and display the message that you passed as an argument to `expect`
    - If you don't call `expect`, the program will compile, but you'll get a warning
        ```bash
        cargo build
        Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
        warning: unused `std::result::Result` which must be used
        --> src/main.rs:10:5
        |
        10 |     io::stdin().read_line(&mut guess);
        |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        |
        = note: #[warn(unused_must_use)] on by default
        ```
### Printing Values with `println!` Placeholders 
- Example
    ```rust
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);
    ```
### Testing the First Part
```bash
cargo run
```
## Generating a Secret Number
- Rust doesn't yet include random number functionality in its standard
library. However, the Rust team does provide a [`rand` crate](https://crates.io/crates/rand)
### Using a Crate to Get More Functionality 
- Cargo fetches the latest versions of
everything from the *registry*, which is a copy of data from
[Crates.io](https://crates.io/). Crates.io is where people in the Rust ecosystem post
their open source Rust projects for others to use
#### Ensuring Reproducible Builds with the *Cargo.lock* File
- The Cargo.lock file was created the first time you ran `cargo build` and is now in your guessing_game directory
- When you build a project for the first time, Cargo figures out all the versions
of the dependencies that fit the criteria and then writes them to the
Cargo.lock file
- When you build your project in the future, Cargo will use the existing Cargo.lock if any

#### Updating a Crate to Get a New Version
- The `update` command which will ignore the *Cargo.lock* file and figure out all the latest versions that fit your specifications in *Cargo.toml*. If that works, Cargo will write those versions to the *Cargo.lock* file
### Generating a Random Number
- Example code as [Listing 2-3: Adding code to generate a random number](https://github.com/sammyne/the-rust-programming-language/commit/2f1c4e936b35138906e9ed8ad4e509c8ed013fb4)
- The `rand::thread_rng` function
will give us the particular random number generator that we're going to use:
one that is local to the current thread of execution and seeded by the
operating system
## Comparing the Guess to the Secret Number
- Example code as [Listing 2-4: Handling the possible return values of comparing two numbers](https://github.com/sammyne/the-rust-programming-language/commit/49c4f9c8a91bbff46cb97b61f06002092968d276)
- A `match` expression is made up of *arms*. An arm consists of a *pattern* and
the code that should be run if the value given to the beginning of the `match`
expression fits that arm’s pattern
- Rust allows us to *shadow* the previous value of `guess` with a new one
## Allowing Multiple Guesses with Looping
### Quitting After a Correct Guess
- Example code as commit [cb29859d1762718f70dafa12dd8fdb5c30c05c07](https://github.com/sammyne/the-rust-programming-language/commit/cb29859d1762718f70dafa12dd8fdb5c30c05c07)
### Handling Invalid Input
- Example code as [Listing 2-5: Ignoring a non-number guess and asking for another guess instead of crashing the program](https://github.com/sammyne/the-rust-programming-language/commit/b1bd33c3bb0cc5e98df696c75da34aeac990f7e6)
- Switching from an `expect` call to a `match` expression is how you generally
move from crashing on an error to handling the error
- The underscore, `_`, is a catchall value