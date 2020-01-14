# Chapter 11. Writing Automated Tests
## How to Write Tests
- The bodies of test functions typically perform these three actions:
    1. Set up any needed data or state.
    2. Run the code you want to test.
    3. Assert the results are what you expect.
### The Anatomy of a Test Function
- A test in Rust is a function that’s annotated with the `test` attribute
  - Attributes are metadata about pieces of Rust code
- To change a function into a test function, add `#[test]` on the line before `fn`
- When you run your tests with the `cargo test` command, Rust builds a test runner binary that runs the functions annotated with the `test` attribute and reports on whether each test function passes or fails
- When we make a new library project with Cargo, a test module with a test
function in it is automatically generated for us
- We could also have non-test functions in the `tests` module to help set up common scenarios or perform common operations. `#[tests]` is to filter out the functions to test.
- Example as [Listing 11-1: The test module and function generated automatically by `cargo new`](./listings/_1/src/lib.rs)
- Other tests
  - Benchmark tests are, as of this writing, only available in nightly Rust
  - Documentation tests
- Tests fail when something in the test function panics
    - Example as [Listing 11-3: Adding a second test that will fail because we call the `panic!` macro](./listings/_3/src/lib.rs)

### Checking Results with the `assert!` Macro
- The `assert!` macro, provided by the standard library, is useful when you want
to ensure that some condition in a test evaluates to `true`
- We give the `assert!` macro an argument that evaluates to a Boolean
  - If the value is `true`, `assert!` does nothing and the test passes
  - If the value is `false`, the `assert!` macro calls the `panic!` macro, which causes the test to fail
- Example as [Listing 11-6: A test for `can_hold` that checks whether a larger rectangle can indeed hold a smaller rectangle](./listings/_5_6/src/lib.rs)

### Testing Equality with the `assert_eq!` and `assert_ne!` Macros
- They’ll also print the two values if the assertion fails, which makes it easier to see *why* the test failed
- Example as [Listing 11-7: Testing the function `add_two` using the `assert_eq!` macro](./listings/_7/src/lib.rs)
- The `assert_ne!` macro will pass if the two values we give it are not equal and fail if they’re equal
- When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the `PartialEq` and `Debug` traits

### Adding Custom Failure Messages
- Any arguments specified after the one required argument to `assert!` or the two
required arguments to `assert_eq!` and `assert_ne!` are passed along to the
`format!` macro
- Example as [greet](./listings/greet/src/lib.rs)

### Checking for Panics with `should_panic`
- This attribute makes a test pass if the code inside the function panics; the test will fail if the code inside the function doesn’t panic
- Examples 
  - [Listing 11-8: Testing that a condition will cause a `panic!`](./listings/_8/src/lib.rs)
  - [Listing 11-9: Testing that a condition will cause a `panic!` with a particular panic message](./listings/_9/src/lib.rs)
    - This test will pass because the value we put in the `should_panic` attribute’s `expected` parameter is a substring of the message that the `Guess::new` function panics with
    - What you choose to specify in the expected parameter for `should_panic` depends on how much of the panic message is unique or dynamic and how precise you want your test to be

### Using `Result<T, E>` in Tests
- Example 
    ```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() -> Result<(), String> {
            if 2 + 2 == 4 {
                Ok(())
            } else {
                Err(String::from("two plus two does not equal four"))
            }
        }
    }
    ```
- Writing tests so they return a `Result<T, E>` enables you to use the question
mark operator in the body of tests, which can be a convenient way to write
tests that should fail if any operation within them returns an `Err` variant.
- You can’t use the `#[should_panic]` annotation on tests that use `Result<T, E>`. Instead, you should return an `Err` value directly when the test should fail

## Controlling How Tests Are Run
- The default behavior of the binary produced by `cargo test` is to run all the tests in parallel and capture output generated during test runs, preventing the output from being displayed and making it easier to read the output related to the test results
- Some command line options go to `cargo test`, and some go to the resulting test
binary. To separate these two types of arguments, you list the arguments that
go to `cargo test` followed by the separator `--` and then the ones that go to
the test binary
    - Running `cargo test --help` displays the options you can use with `cargo test`
    - Running `cargo test -- --help` displays the options you can use after the separator `--`

### Running Tests in Parallel or Consecutively
- Because the tests are running at the same time, make sure your tests don’t depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables
- Send the `--test-threads` flag and the number of threads you want to use to the **test binary**

### Showing Function Output
- By default, if a test passes, Rust’s test library captures anything printed to standard output
- Example as [Listing 11-10: Tests for a function that calls `println!`](./listings/_10/src/lib.rs)
- Tell Rust to also show the output of successful tests at the end with `--show-output`

### Running a Subset of Tests by Name
- Choose which tests to run by passing `cargo test` the name or names of the test(s) you want to run as an argument
- Example as [Listing 11-11: Three tests with three different names](./listings/_11/src/lib.rs)

#### Running Single Tests
- Pass the name of any test function to `cargo test` to run only that test
    ```bash
    cargo test one_hundred
    ```

#### Filtering to Run Multiple Tests
- Specify part of a test name, and any test whose name matches that value will be run
    ```bash
    cargo test add
    ```
- The module in which a test appears becomes part of the test’s name, so we can run all the tests in a module by filtering on the module’s name

### Ignoring Some Tests Unless Specifically Requested
- Annotate the time-consuming tests using the `ignore` attribute to exclude them
- Example as [ignore](./listings/ignore/src/lib.rs)
- If we want to run only the ignored tests, we can use `cargo test -- --ignored`
    ```bash
    cargo test -- --ignored
    ```

## Test Organization
- Two main categories: *unit tests* and *integration tests*
  - Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces
  - Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test

### Unit Tests
- The purpose of unit tests is to test each unit of code in isolation from the
rest of the code to quickly pinpoint where code is and isn’t working as
expected
- The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`.

#### The Tests Module and `#[cfg(test)]`
- The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run
the test code only when you run `cargo test`, not when you run `cargo build`
- Unit tests go in the same files as the code, you’ll use `#[cfg(test)]` to specify that they shouldn’t be included in the compiled result

#### Testing Private Functions
- Example as [Listing 11-12: Testing a private function](./listings/_12/src/lib.rs)

### Integration Tests
- They can only call functions that are part of your library’s public API
- Their purpose is to test whether many parts of your library work together correctly

#### The *tests* Directory
- Cargo knows to look for integration test files in this directory
- Cargo treats the `tests` directory specially and compiles files in this directory only when we run `cargo test`
- Example as [Listing 11-13: An integration test of a function in the `adder` crate](./listings/_13/tests/integration_test.rs)
- To run all the tests in a particular integration test file, use the `--test` argument of `cargo test` followed by the name of the file
  
#### Submodules in Integration Tests
- Files in subdirectories of the *tests* directory don’t get compiled as separate crates or have sections in the test output
- Example as [submodules](./listings/submodules/tests/common/mod.rs)

#### Integration Tests for Binary Crates
- If our project is a binary crate that only contains a *src/main.rs* file and
doesn’t have a *src/lib.rs* file, we can’t create integration tests in the
*tests* directory and bring functions defined in the *src/main.rs* file into
scope with a `use` statement