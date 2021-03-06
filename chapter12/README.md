# An I/O Project: Building a Command Line Program

- Trick: print error messages to the standard error console stream (`stderr`) instead of standard 
  output (`stdout`), so, for example, the user can redirect successful output to a file while still
  seeing error messages onscreen

## Accepting Command Line Arguments
### Reading the Argument Values
- By function provided in Rust’s standard library `std::env::args`
- Two details about iterators
  - iterators produce a series of values
  - call the `collect` method on an iterator to turn it into a collection, such as a vector, containing all the elements the iterator produces
- Example as [Listing 12-1: Collecting the command line arguments into a vector and printing them](listings/_01/src/main.rs)

    > If your program needs to accept arguments containing invalid
    > Unicode, use `std::env::args_os` instead

- It's often convenient to have access to the program name in case you want to print it in messages or 
  change behavior of the program based on what command line alias was used to invoke the program

### Saving the Argument Values in Variables
- Example as [Listing 12-2: Creating variables to hold the query argument and filename argument](./listings/_02/src/main.rs)

## Reading a File
- Example as [Listing 12-4: Reading the contents of the file specified by the second argument](listings/_04/src/main.rs)
    - Run by `cargo run -- the poem.txt`

## Refactoring to Improve Modularity and Error Handling
### Separation of Concerns for Binary Projects
- How
    * Split your program into a *main.rs* and a *lib.rs* and move your program's
    logic to *lib.rs*.
    * As long as your command line parsing logic is small, it can remain in
    *main.rs*.
    * When the command line parsing logic starts getting complicated, extract it
    from *main.rs* and move it to *lib.rs*.
- The responsibilities that remain in the `main` function after this process
should be limited to the following:
    * Calling the command line parsing logic with the argument values
    * Setting up any other configuration
    * Calling a `run` function in *lib.rs*
    * Handling the error if `run` returns an error

#### Extracting the Argument Parser
- Example as [Listing 12-5: Extracting a parse_config function from main](./listings/_05/src/main.rs)

#### Grouping Configuration Values
> Note: Using primitive values when a complex type would be more appropriate is
> an anti-pattern known as *primitive obsession*.

- Example as [Listing 12-6: Refactoring parse_config to return an instance of a Config struct](./listings/_06/src/main.rs)

#### Creating a Constructor for `Config`
- Example as [Listing 12-7: Changing parse_config into Config::new](./listings/_07/src/main.rs)

### Fixing the Error Handling
#### Improving the Error Message
- Example as [Listing 12-8: Adding a check for the number of arguments](./listings/_08/src/main.rs)
- Still we also have extraneous information we don't want to give to our users

#### Returning a `Result` from `new` Instead of Calling `panic!`
- Example as [Listing 12-9: Returning a Result from Config::new](./listings/_09/src/main.rs)
    - Not ready to run yet

#### Calling `Config::new` and Handling Errors
- A nonzero exit status is a convention to signal to the process that called our program that the
  program exited with an error state.
- Example as [Listing 12-10: Exiting with an error code if creating a new Config fails](./listings/_10/src/main.rs)
  - Important method `unwrap_or_else`
    - If the `Result` is an `Ok` value, this method's behavior is similar to `unwrap`: it returns the inner value `Ok` is wrapping
    - If the value is an `Err` value, this method calls the code in the *closure*, which is an anonymous function we define and pass as an argument to `unwrap_or_else`

### Extracting Logic from `main`
- Example as [Listing 12-11: Extracting a run function containing the rest of the program logic src/main.rs](./listings/_11/src/main.rs)

#### Returning Errors from the `run` Function
- Example as [Listing 12-12: Changing the run function to return Result](./listings/_12/src/main.rs)
    - 3 significant changes
        - For the error type, we used the *trait object* `Box<dyn Error>`
        - We've removed the call to `expect` in favor of the `?` operator
        - The `run` function now returns an `Ok` value in the success case
          - Using `()` like this is the idiomatic way to indicate that we're calling `run` for its side effects only 

#### Handling Errors Returned from `run` in `main`
- Example snippet as 
    ```rust
    fn main() {
        // --snip--
        println!("Searching for {}", config.query);
        println!("In file {}", config.filename);

        if let Err(e) = run(config) {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
    ```

### Splitting Code into a Library Crate
- Example as [_13_14](./listings/_13_14/src/main.rs)
    - Listing 12-13: Moving Config and run into [src/lib.rs](./listings/_13_14/src/lib.rs)
    - Listing 12-14: Using the minigrep crate in [src/main.rs](./listings/_13_14/src/main.rs)

## Developing the Library's Functionality with Test-Driven Development
- How
    1. Write a test that fails and run it to make sure it fails for the reason you
    expect.
    2. Write or modify just enough code to make the new test pass.
    3. Refactor the code you just added or changed and make sure the tests
    continue to pass.
    4. Repeat from step 1!

### Writing a Failing Test
- Example as [_15_16](./listings/_15_16/src/lib.rs)
    - Listing 12-15: Creating a failing test for the search function we wish we had
    - Listing 12-16: Defining just enough of the search function so our test will compile

### Writing Code to Pass the Test
- Implementation detail of `search`
    * Iterate through each line of the contents.
    * Check whether the line contains our query string.
    * If it does, add it to the list of values we're returning.
    * If it doesn't, do nothing.
    * Return the list of results that match.

- Example as [_17_19](./listings/_17_19/src/lib.rs)
    - Iterating Through Lines with the `lines` Method as Listing 12-17: Iterating through each line in contents
    - Searching Each Line for the Query as Listing 12-18: Adding functionality to see whether the line contains the string in query
    - Storing Matching Lines as Listing 12-19: Storing the lines that match so we can return them

#### Using the `search` Function in the `run` Function
- Example as [run_with_search](./listings/run_with_search/src/main.rs)
- Test
    ```bash
    cargo run -- frog poem.txt
    cargo run -- body poem.txt
    cargo run -- monomorphization poem.txt
    ```

## Working with Environment Variables
### Writing a Failing Test for the Case-Insensitive `search` Function
- Example as [Listing 12-20: Adding a new failing test for the case-insensitive function we're about to add](./listings/_20/src/main.rs)

### Implementing the `search_case_insensitive` Function
- Examples as [_21_23](./listings/_21_23/src/lib.rs)
  - We're using the `is_err` method on the `Result` to check whether it's an error and therefore unset, which means it *should* do a case-sensitive search

## Writing Error Messages to Standard Error Instead of Standard Output

### Checking Where Errors Are Written
- Test
    ```bash
    cargo run > output.txt
    ```

- The `>` syntax tells the shell to write the contents of standard output to *output.txt* instead of the screen

### Printing Errors to Standard Error
- The standard library provides the `eprintln!` macro that prints to the standard error stream
- Example as [Listing 12-24: Writing error messages to standard error instead of standard output using `eprintln`!](./listings/_24/src/main.rs)
- Test
    ```bash
    cargo run -- to poem.txt > output.txt

    // example output
    Are you nobody, too?
    How dreary to be somebody!
    ```
