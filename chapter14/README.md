# Chapter 14. More About Cargo and Crates.io

## Customizing Builds with Release Profiles
- `release` profiles are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code
- 2 main profiles: the `dev` profile Cargo uses when you run `cargo build` and the `release` profile Cargo uses when you run `cargo build --release`.
- By adding `[profile.*]` sections to *Cargo.toml* for any profile you want to customize, you can override any subset of the default settings
    - The `opt-level` setting controls the number of optimizations Rust will apply to your code, with a range of 0 (least optimizations for `dev`) to 3 (most optimizatins for `release`)

## Publishing a Crate to Crates.io
### Making Useful Documentation Comments
- *documentation comment*: that will generate HTML documentation. The HTML
displays the contents of documentation comments for public API items intended
for programmers interested in knowing how to *use* your crate as opposed to how
your crate is *implemented*
    - [Listing 14-1: A documentation comment for a function](./listings/_01/src/main.rs)

#### Commonly Used Sections
- **Examples**
- **Panics**: The scenarios in which the function being documented could panic
- **Errors**: If the function returns a `Result`, describing the kinds of
  errors that might occur and what conditions might cause those errors to be
  returned can be helpful to callers so they can write code to handle the
  different kinds of errors in different ways
- **Safety**: If the function is `unsafe` to call (we discuss unsafety in
  Chapter 19), there should be a section explaining why the function is unsafe
  and covering the invariants that the function expects callers to uphold

#### Documentation Comments as Tests
- Running `cargo test` will run the code examples in your documentation as tests

#### Commenting Contained Items
- `//!`, adds documentation to the item that contains the comments rather than adding documentation to the items following the comments
  - We typically use these doc comments inside the crate root file (*src/lib.rs* by convention) or inside a module to document the crate or the module as a whole

### Exporting a Convenient Public API with `pub use`
- **WHY**: The structure of your public API is a major consideration when publishing a
crate. People who use your crate are less familiar with the structure than you
are and might have difficulty finding the pieces they want to use if your crate
has a large module hierarchy.
    - Example as [listing 03-04](./listings/_03_04/src/main.rs)
      - Listing 14-3: An `art` library with items organized into `kinds` and `utils` modules
      - Listing 14-4: A crate using the `art` crate's items with its internal structure exported
- **HOW**: re-export items to make a public structure that's different from your private structure by using `pub use`
    - Example code as [listing 05-06](./listings/_05_06/src/lib.rs)
      - Listing 14-5: Adding `pub use` statements to re-export items
      - Listing 14-6: A program using the re-exported items from the `art` crate
- Choosing `pub use` gives you flexibility in how you structure your crate internally and decouples that internal structure from what you present to your users

### Setting Up a Crates.io Account
- Before you can publish any crates, you need to create an account with Github on [crates.io](https://crates.io/) and get an API token
- Once logged in, retrieve API key at [https://crates.io/me/](https://crates.io/me/), and run command `cargo run ${your-api-key}`

### Adding Metadata to a New Crate
- **WHERE**: `[package]` section in Cargo.toml
- Crate names on [crates.io](https://crates.io/) are allocated on a first-come, first-served basis, without namespacing support
- Description and license are required 
    - For license,
      - If provided as license identifier value, ok values can be checked at the [Linux Foundation's Software Package Data Exchange (SPDX)][http://spdx.org/licenses/]
      - If provided as files, use the key `license-file` with value pointing the text file path
      - We can also specify multiple license identifiers separated by `OR` to have multiple licenses for your project

### Publishing to Crates.io
- The version can never be overwritten, and the code cannot be deleted
- Relevant command as `cargo publish`

### Publishing a New Version of an Existing Crate
- **HOW**: Change the `version` value specified in your *Cargo.toml* file and republish
- Use the [Semantic Versioning rules][http://semver.org/] to decide what an appropriate next version number is based on the kinds of changes you’ve made

### Removing Versions from Crates.io with `cargo yank`
- Yanking a version prevents new projects from starting to depend on that version
while allowing all existing projects that depend on it to continue to download
and depend on that version
- By adding `--undo` to the command, you can also undo a yank and allow projects
to start depending on a version again

## Cargo Workspaces
- **WHEN**: The library crate continues to get bigger and you want to split up your package further into multiple library crates

### Creating a Workspace
- A *workspace* is a set of packages that share the same *Cargo.lock* and output
directory
- By sharing one *target* directory, the crates can avoid unnecessary rebuilding
- Example as [adder](./listings/add/Cargo.toml)

### Creating the Second Crate in the Workspace
- Example as [Listing 14-7: Using the add-one library crate from the adder crate](./listings/_07/src/main.rs)

#### Depending on an External Crate in a Workspace
- Even though `rand` is used somewhere in the workspace, we can’t use it in other crates in the workspace unless we add `rand` to their *Cargo.toml* files as well

#### Adding a Test to a Workspace
- Running `cargo test` in a workspace structured like this one will run the tests for all the crates in the workspace
- We can also run tests for one particular crate in a workspace from the top-level directory by using the `-p` flag and specifying the name of the crate we want to test
- The `cargo publish` command does not have an `--all` flag or a `-p` flag, so you must change to each crate's directory and run `cargo publish` on each crate in the workspace to publish the crates

## Installing Binaries from Crates.io with cargo install
- You can only install packages that have binary targets
  - A *binary target* is the runnable program that is created if the crate has a *src/main.rs* file or another file specified as a binary, as opposed to a library target that isn't runnable on its own but is suitable for including within other programs
- All binaries installed with `cargo install` are stored in the installation root's *bin* folder (default as `$HOME/.cargo/bin`)

## Extending Cargo with Custom Commands
- If a binary in your `$PATH` is named `cargo-something`, you can run it as if it was a Cargo subcommand by running `cargo something`
    - Available subcommands can be queried using `cargo --list`
- Being able to use `cargo install` to install extensions and then run them just like the built-in Cargo tools is a super convenient benefit of Cargo's design
