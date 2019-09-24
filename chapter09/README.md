# Chapter 09. Error Handling

Rust groups errors into two major categories
- recoverable errors by means of `Result<T,E>`
- unrecoverable errors by means of `panic!` macro

## Unrecoverable Errors with `panic!`
- When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit
- Execution of `panic!` is a lot of work
- The alternative is to immediately *abort*, which ends the program without cleaning up
    - Switch from unwinding to aborting upon a panic by adding `panic = 'abort'` to the appropriate [profile] sections in your Cargo.toml file.
### Using a `panic!` Backtrace
- Example: access an element by index in a vector
    ```rust
    fn main() {
        let v = vec![1, 2, 3];

        v[99];
    }
    ```
- To protect your program from this sort of vulnerability (a.k.a *buffer overread*), if you try to read an element at an index that doesn’t exist, Rust will stop execution and refuse to continue
- A *backtrace* is a list of all the functions that have been called to get to this point
    - The key to reading the backtrace is to start from the top and read until you see files you wrote
    - Enable backtrace by setting the `RUST_BACKTRACE` environment variable to any value except 0
- In order to get backtraces with this information, debug symbols must be enabled. Debug symbols are enabled by default when using `cargo build` or `cargo run` without the `--release` flag
## Recoverable Errors with Result 
- Utility enum `Result<T, E>` as 
    ```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```
- Check returned type of function by looking at its API doc, or compile to trigger error report
- One way to handle the `Result` using a basic tool, the `match` expression
    
### Matching on Different Errors
- Use case: if `File::open` failed because the file doesn't exist, we want to create the file and return the handle to the new file. If `File::open` failed for any other reason, just `panic!`
- The `Result<T,E>` type has many methods that accept a closure and are implemented using `match` expressions. Using those methods will make your code more concise
### Shortcuts for Panic on Error: `unwrap` and `expect`
- If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us
- Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier
### Propagating Errors
- WHY: Gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code
#### A Shortcut for Propagating Errors: the `?` Operator
- If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword so the error value gets propagated to the calling code
- A difference between what the `match` expression and
the `?` operator: error values that have the `?` operator called on them go
through the `from` function, defined in the `From` trait in the standard
library, which is used to convert errors from one type into another. When the
`?` operator calls the `from` function, the error type received is converted
into the error type defined in the return type of the current function.`
- Chaining method calls after the `?` operator to shorten codes
- Utility function in standary library is `fs::read_to_string()`
#### The `?` Operator Can Only Be Used in Functions That Return `Result`
- One technique is to change the return type of your function to be `Result<T, E>` if you have no restrictions preventing that. The other technique is to use
a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>` in
whatever way is appropriate
- The `main` function is special, and there are restrictions on what its return
type must be. One valid return type for main is `()`, and conveniently, another
valid return type is `Result<T, E>`
## To panic! or Not to panic!
- Call `panic!` for any error situation, whether there's a possible way to recover or not, but
then you’re making the decision on behalf of the code calling your code that a
situation is unrecoverable. When you choose to return a `Result` value, you
give the calling code options rather than making the decision for it. The
calling code could choose to attempt to recover in a way that’s appropriate for
its situation, or it could decide that an `Err` value in this case is
unrecoverable, so it can call `panic!` and turn your recoverable error into an
unrecoverable one. Therefore, returning `Result` is a good default choice when
you're defining a function that might fail
### Examples, Prototype Code, and Tests
- Having robust error-handling code in the example as well can make the example less clear
- The `unwrap` and `expect` methods are very handy when prototyping, before you’re ready to decide how to handle errors
- Because `panic!` is how a test is marked as a failure, calling `unwrap` or `expect` is exactly what should happen
### Cases in Which You Have More Information Than the Compiler
- If you can ensure by manually inspecting the code that you'll never
have an `Err` variant, it’s perfectly acceptable to call `unwrap`
### Guidelines for Error Handling
- It's advisable to have your code panic when it’s possible that your code
could end up in a bad state
- A *bad state* is when some
assumption, guarantee, contract, or invariant has been broken, such as when
invalid values, contradictory values, or missing values are passed to your
code—plus one or more of the following:
    * The bad state is not something that's *expected* to happen occasionally.
    * Your code after this point needs to rely on not being in this bad state.
    * There’s not a good way to encode this information in the types you use.
- When failure is expected, it’s more appropriate to return a `Result`
than to make a `panic!` call
- When your code performs operations on values, your code should verify the
values are valid first and panic if the values aren’t valid. This is mostly for
safety reasons: attempting to operate on invalid data can expose your code to
vulnerabilities.
    > Contracts for a function, especially when a violation will cause a panic, should be explained in the API documentation for the function
- If your function has a particular type as a parameter, you can proceed with your code's logic
knowing that the compiler has already ensured you have a valid value

### Creating Custom Types for Validation
- HOW: make a new type and put the validations in a function to create
an instance of the type rather than repeating the validations everywhere