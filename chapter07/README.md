# Chapter 07. Managing Growing Projects with Packages, Crates, and Modules

- Benefits of encapsulating implementation details:
    - Grouping functionality
    - Reuse code at a higher level
- The nested context in which code is written has a set of names that are defined as “in scope.”
- The *module system* allows you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs
    * **Packages:** A Cargo feature that lets you build, test, and share crates
    * **Crates:** A tree of modules that produces a library or executable
    * **Modules** and **use:** Let you control the organization, scope, and
    privacy of paths
    * **Paths:** A way of naming an item, such as a struct, function, or module

## Packages and Crates
- A crate is a binary or library. The *crate root* is a source file that the Rust
compiler starts from and makes up the root module of your crate
- A *package* is one or more crates that provide a set of functionality
- A package 
  - contains a *Cargo.toml* file that describes how to build those crates
  - *must* contain zero or one library crates, and no more
  - can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary)
- Cargo follows a convention 
  - *src/main.rs* is the crate root of a binary crate with the same name as the package
  - if the package directory contains *src/lib.rs*, the package contains a library crate with the same name as the package, and *src/lib.rs* is its crate root
- A package can have multiple binary crates by placing files in the *src/bin* directory: each file will be a separate binary crate
- A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects

## Defining Modules to Control Scope and Privacy
- *paths* that allow you to name items; the `use` keyword that brings a path into scope; and the `pub` keyword to make items public
- other concepts: the `as` keyword, external packages, and the glob operator
- *Modules* let us organize code within a crate into groups for readability and
easy reuse. Modules also control the *privacy* of items, which is whether an
item can be used by outside code (*public*) or is an internal implementation
detail and not available for outside use (*private*)
- Example: a library crate that provides the functionality of a restaurant
    - [Listing 7-1: A front_of_house module containing other modules that then contain functions](listings/_01/src/lib.rs)
- *src/main.rs* and *src/lib.rs* are called crate
roots. The reason for their name is that the contents of either of these two
files form a module named `crate` at the root of the crate’s module structure,
known as the *module tree*
- The entire module tree is rooted under the implicit module named `crate`

## Paths for Referring to an Item in the Module Tree
- A path can take two forms:
    * An *absolute path* starts from a crate root by using a crate name or a
    literal `crate` as [Listing 7-3: Calling the `add_to_waitlist` function using absolute and relative paths](./listings/_03/src/lib.rs) (Listing 7-3 isn't ready for running yet)
    * A *relative path* starts from the current module and uses `self`, `super`, or
    an identifier in the current module.
- Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`)
- Modules define Rust’s
*privacy boundary*: the line that encapsulates the implementation details
external code isn’t allowed to know about, call, or rely on. So, if you want to
make an item like a function or struct private, you put it in a module
- The way privacy works in Rust is 
  - All items (functions, methods, structs, enums, modules, and constants) are private by default 
  - Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules
    > The reason is that child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined

### Exposing Paths with the `pub` Keyword
- Making the module public doesn’t make its contents public
  - Example as [Listing 7-5: Declaring the `hosting` module as `pub` to use it from `eat_at_restaurant`](./listings/_05/src/lib.rs)
- Ok example as [Listing 7-7: Adding the `pub` keyword to `mod hosting` and `fn add_to_waitlist` lets us call the function from `eat_at_restaurant`](./listings/_07/src/lib.rs)

### Starting Relative Paths with `super`
- Construct relative paths that begin in the parent module by using `super` at the start of the path
- WHEN: two modules are likely to stay in the same relationship to each other and get moved
- Example as [Listing 7-8: Calling a function using a relative path starting with `super`](./listings/_08/src/main.rs)

### Making Structs and Enums Public
- If we use `pub` before a struct definition, we make the struct public, but the struct’s fields will still be private
- If we make an enum public, all of its variants are then public. We only need the `pub` before the `enum` keyword

## Bringing Paths into Scope with the `use` Keyword
- Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem
- You can also bring an item into scope with `use` and a relative path as Listing 7-12

### Creating Idiomatic `use` Paths
- The idiomatic way to bring a function into scope with `use`. Bringing the
function’s parent module into scope with `use` so we have to specify the parent
module when calling the function makes it clear that the function isn’t locally
defined while still minimizing repetition of the full path
- When bringing in structs, enums, and other items with `use`,
it’s idiomatic to specify the full path
- The exception to this idiom is if we’re bringing two items with the same name
into scope with `use` statements, because Rust doesn’t allow that

### Providing New Names with the `as` Keyword

### Re-exporting Names with `pub use`
- When we bring a name into scope with the `use` keyword, the name available in
the new scope is private
- With `pub use`, we can write our code with one structure but expose a different
structure. Doing so makes our library well organized for programmers working on
the library and programmers calling the library

### Using External Packages
- The standard library (`std`) is also a crate that’s external to our
package. Because the standard library is shipped with the Rust language, we
don’t need to change *Cargo.toml* to include `std`

### Using Nested Paths to Clean Up Large `use` Lists
- HOW: Specify the common part of the path, followed by two colons, and then curly brackets around a list of the parts of the paths that differ

### The Glob Operator
- WHY: bring *all* public items defined in a path into scope
- The glob operator is often used when testing to bring everything under test into the `tests` module
- The glob operator is also sometimes used as part of the prelude pattern 

## Separating Modules into Different Files
- Using a semicolon after `mod front_of_house` rather than using a block tells
Rust to load the contents of the module from another file with the same name as
the module