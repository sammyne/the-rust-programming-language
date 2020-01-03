# Chapter 19. Advanced Features

## Unsafe Rust
- Unsafe Rust exists because, by nature, static analysis is conservative. When
the compiler tries to determine whether or not code upholds the guarantees,
it’s better for it to reject some valid programs rather than accept some
invalid programs
- Another reason Rust has an unsafe alter ego is that the underlying computer
hardware is inherently unsafe. If Rust didn’t let you do unsafe operations, you
couldn’t do certain tasks

### Unsafe Superpowers
- To switch to unsafe Rust, use the `unsafe` keyword and then start a new block
that holds the unsafe code
- *unsafe superpowers* as 5 actions
    * Dereference a raw pointer
    * Call an unsafe function or method
    * Access or modify a mutable static variable
    * Implement an unsafe trait
    * Access fields of `union`s ?
- It’s important to understand that `unsafe` doesn’t turn off the borrow checker
or disable any other of Rust’s safety checks
- By requiring these four
unsafe operations to be inside blocks annotated with `unsafe` you’ll know that
any errors related to memory safety must be within an `unsafe` block
- To isolate unsafe code as much as possible, it’s best to enclose unsafe code
within a safe abstraction and provide a safe API

### Dereferencing a Raw Pointer
- raw pointers can be immutable or mutable and are written as `*const T` and `*mut T`
- *immutable* means that the pointer can’t be directly assigned to after being dereferenced
- Different from references and smart pointers, raw pointers:
    * Are allowed to ignore the borrowing rules by having both immutable and
    mutable pointers or multiple mutable pointers to the same location
    * Aren’t guaranteed to point to valid memory
    * Are allowed to be null
    * Don’t implement any automatic cleanup
- We can create raw pointers in safe code; we just can’t dereference raw pointers outside an
unsafe block
- We’ve created raw pointers by using `as` to cast an immutable and a mutable
reference into their corresponding raw pointer types
    - Example as [Listing 19-{01,02}](./listings/_01_02/src/main.rs)
- Creating a pointer does no harm; it’s only when we try to access the value that
it points at that we might end up dealing with an invalid value
- With raw pointers, we can create a mutable pointer and an immutable pointer to the
same location and change data through the mutable pointer, potentially creating
a data race
- why would you ever use raw pointers? 
  - One major use case is when interfacing with C code
  - Another case is when building up safe abstractions that the borrow checker doesn’t understand

### Calling an Unsafe Function or Method
- Unsafe functions and methods look exactly like regular functions and
methods, but they have an extra `unsafe` before the rest of the definition. The
`unsafe` keyword in this context indicates the function has requirements we
need to uphold when we call this function, because Rust can’t guarantee we’ve
met these requirements
- Bodies of unsafe functions are effectively `unsafe` blocks, so to perform other
unsafe operations within an unsafe function, we don’t need to add another
`unsafe` block

#### Creating a Safe Abstraction over Unsafe Code
- Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice
- Example as [listing 19-{04,05,06}](./listings/_04_05_06/src/main.rs)

#### Using `extern` Functions to Call External Code
- Rust has a keyword, `extern`, that facilitates the creation and use of a *Foreign Function Interface (FFI)*
    - An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions
- Functions declared within `extern` blocks are always unsafe to call from Rust code
- The `"C"` part defines which *application binary interface (ABI)* the external function uses: the ABI defines how to call the function at the assembly level. The `"C"` ABI is the most common and follows the C programming language’s ABI

> #### Calling Rust Functions from Other Languages
> to add a `#[no_mangle]` annotation to tell the Rust compiler not to mangle
> the name of this function. *Mangling* is when a compiler changes the name
> we’ve given a function to a different name that contains more information for
> other parts of the compilation process to consume but is less human readable

### Accessing or Modifying a Mutable Static Variable
- In Rust, global variables are called *static* variables
    - Example as [listing 19-09](./listings/_09/src/main.rs)
- The names of static variables are in
`SCREAMING_SNAKE_CASE` by convention, and we *must* annotate the variable’s
type, which is `&'static str` in this example
- Static variables can only store
references with the `'static` lifetime, which means the Rust compiler can
figure out the lifetime; we don’t need to annotate it explicitly
- Constants and immutable static variables might seem similar, but 
  - A subtle difference is that values in a static variable have a fixed address in memory
  - Another difference between constants and static variables is that static variables can be mutable
    - Example as [listing 19-10](./listings/_10/src/main.rs)
- With mutable data that is globally accessible, it’s difficult to ensure there
are no data races, which is why Rust considers mutable static variables to be
unsafe

### Implementing an Unsafe Trait
- A trait is unsafe when at least one of its methods has some invariant that the
compiler can’t verify. We can declare that a trait is `unsafe` by adding the
`unsafe` keyword before `trait` and marking the implementation of the trait as
`unsafe` too


## Advanced Traits
### Specifying Placeholder Types in Trait Definitions with Associated Types
- *Associated types* connect a type placeholder with a trait such that the trait
method definitions can use these placeholder types in their signatures
- The difference is that when using generics, as in Listing 19-13, we must
annotate the types in each implementation
- when a trait has a
generic parameter, it can be implemented for a type multiple times, changing
the concrete types of the generic type parameters each time. When we use the
`next` method on `Counter`, we would have to provide type annotations to
indicate which implementation of `Iterator` we want to use
- With associated types, we don’t need to annotate types because we can’t
implement a trait on a type multiple times
- Example as [listing 19-{12,13}](./listings/_12_13/src/lib.rs)

### Default Generic Type Parameters and Operator Overloading
- The syntax for specifying a
default type for a generic type is `<PlaceholderType=ConcreteType>` when
declaring the generic type
- *Operator overloading* is customizing the behavior of an operator
(such as `+`) in particular situations
- But you can overload the operations and corresponding traits listed
in `std::ops` by implementing the traits associated with the operator
- *default type parameters* (@TODO add to listing)
- use default type parameters in two main ways:
    * To extend a type without breaking existing code
    * To allow customization in specific cases most users won’t need

### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
- When calling methods with the same name, you’ll need to tell Rust which one you
want to use
- When we call `fly` on an instance of `Human`, the compiler defaults to calling
the method that is directly implemented on the type
- Specifying the trait name before the method name clarifies to Rust which
implementation of `fly` we want to call
- associated functions that are part of traits don’t have a `self`
parameter. When two types in the same scope implement that trait, Rust can’t
figure out which type you mean unless you use *fully qualified syntax*
- fully qualified syntax
    ```rust
    <Type as Trait>::function(receiver_if_method, next_arg, ...);
    ```
  
### Using Supertraits to Require One Trait’s Functionality Within Another Trait
- We can do that in the trait definition by specifying `OutlinePrint: Display`

### Using the Newtype Pattern to Implement External Traits on External Types
- The tuple struct
will have one field and be a thin wrapper around the type we want to implement
a trait for. Then the wrapper type is local to our crate, and we can implement
the trait on the wrapper
- There is no runtime performance penalty for using this
pattern, and the wrapper type is elided at compile time
- The downside of using this technique is that `Wrapper` is a new type, so it
doesn’t have the methods of the value it’s holding
- If we wanted the new type to have every method the inner type has,
implementing the `Deref` trait (discussed in Chapter 15 in the “Treating Smart
Pointers Like Regular References with the `Deref` Trait” on the `Wrapper` to return
the inner type would be a solution

## Advanced Types
### Using the Newtype Pattern for Type Safety and Abstraction
- Another use of the newtype pattern is in abstracting away some implementation
details of a type
- Newtypes can also hide internal implementation

### Creating Type Synonyms with Type Aliases
- using this method, we don’t get the type checking benefits
that we get from the newtype pattern discussed earlier
- Writing this lengthy type in function signatures and as type annotations all
over the code can be tiresome and error prone
- Type aliases are also commonly used with the `Result<T, E>` type for reducing
repetition
- The type alias helps in two ways: it makes code easier to write *and* it gives
us a consistent interface across all of `std::io`

### The Never Type that Never Returns
- Rust has a special type named `!` that’s known in type theory lingo as the
*empty type* because it has no values. We prefer to call it the *never type*
because it stands in the place of the return type when a function will never
return
- Functions that return never are called *diverging functions*
- `continue` has a `!` value
- expressions of type `!` can be coerced into any other type
- One final expression that has the type `!` is a `loop`

### Dynamically Sized Types and the `Sized` Trait
- *dynamically sized types*. Sometimes referred to
as *DSTs* or *unsized types*
- not `&str`, but `str` on its own, is a DST
- a `&str` is *two* values: the address of the `str` and its
length
- this is the way in which dynamically sized types are used in Rust: they have an extra bit of
metadata that stores the size of the dynamic information. The golden rule of
dynamically sized types is that we must always put values of dynamically sized
types behind a pointer of some kind
- To work with DSTs, Rust has a particular trait called the `Sized` trait to
determine whether or not a type’s size is known at compile time
- By default, generic functions will work only on types that have a known size at
compile time
- A trait bound on `?Sized` is the opposite of a trait bound on `Sized`: we would
read this as "`T` may or may not be `Sized`"
    - This syntax is only available for `Sized`, not any other traits

## Advanced Functions and Closures
### Function Pointers
- Functions coerce to the type `fn`
- The `fn` type is called a *function pointer*
- Unlike closures, `fn` is a type rather than a trait, so we specify `fn` as the
parameter type directly rather than declaring a generic type parameter with one
of the `Fn` traits as a trait bound
- Function pointers implement all three of the closure traits (`Fn`, `FnMut`, and
`FnOnce`), so you can always pass a function pointer as an argument for a
function that expects a closure. It’s best to write functions using a generic
type and one of the closure traits so your functions can accept either
functions or closures
- An example of where you would want to only accept `fn` and not closures is when
interfacing with external code that doesn’t have closures: C functions can
accept functions as arguments, but C doesn’t have closures
- As an example of where you could use either a closure defined inline or a named
function, let’s look at a use of `map`
- another useful pattern that exploits an implementation detail of tuple
structs and tuple-struct enum variants
  - use `()` as initializer syntax
  - The initializers are actually implemented as functions returning an instance that’s constructed from their arguments
  - we can specify the initializer functions as arguments for methods that take closures

### Returning Closures
- you can’t do that with closures because they don’t have a
concrete type that is returnable; you’re not allowed to use the function
pointer `fn` as a return type

## Macros
- The term *macro* refers to a family of features in Rust: *declarative* macros with `macro_rules!` and three kinds of *procedural* macros:
    * Custom `#[derive]` macros that specify code added with the `derive` attribute
    used on structs and enums
    * Attribute-like macros that define custom attributes usable on any item
    * Function-like macros that look like function calls but operate on the tokens
    specified as their argument

### The Difference Between Macros and Functions
- macros are a way of writing code that writes other code, which
is known as *metaprogramming*
- macros are a way of writing code that writes other code, which
is known as *metaprogramming*
- A function signature must declare the number and type of parameters the
function has. Macros, on the other hand, can take a variable number of
parameters
- macros are expanded
before the compiler interprets the meaning of the code, so a macro can, for
example, implement a trait on a given type. A function can’t
- The downside to implementing a macro instead of a function is that macro
definitions are more complex than function definitions because you’re writing
Rust code that writes Rust code
- Another important difference between macros and functions is that you must
define macros or bring them into scope *before* you call them in a file, as
opposed to functions you can define anywhere and call anywhere

### Declarative Macros with `macro_rules!` for General Metaprogramming
- A.K.A. “macros by example,” “`macro_rules!` macros,” or just plain “macros”
- Macros also compare a value to patterns that are
associated with particular code: in this situation, the value is the literal
Rust source code passed to the macro; the patterns are compared with the
structure of that source code; and the code associated with each pattern, when
matched, replaces the code passed to the macro
- To define a macro, you use the `macro_rules!` construct
- The `#[macro_export]` annotation indicates that this macro should be made
available whenever the crate in which the macro is defined is brought into
scope
- Start the macro definition with `macro_rules!` and the name of the
macro we’re defining *without* the exclamation mark
- For the full macro pattern syntax, see [the reference](https://doc.rust-lang.org/stable/reference/macros.html)
- First, a set of parentheses encompasses the whole pattern. A dollar sign (`$`)
is next, followed by a set of parentheses that captures values that match the
pattern within the parentheses for use in the replacement code. Within `$()` is
`$x:expr`, which matches any Rust expression and gives the expression the name
`$x` (@TODO: add to listing)
- The comma following `$()` indicates that a literal comma separator character
could optionally appear after the code that matches the code in `$()`. The `*`
specifies that the pattern matches zero or more of whatever precedes the `*` (@TODO: add to listing)
- There are some strange edge cases with `macro_rules!`
- To learn more about how to write macros, consult
the online documentation or other resources, such as [“The Little Book of Rust
Macros”](https://danielkeep.github.io/tlborm/book/index.html)

### Procedural Macros for Generating Code from Attributes
- Procedural macros accept some code as an input,
operate on that code, and produce some code as an output rather than matching
against patterns and replacing the code with other code as declarative macros
do
- The three kinds of procedural macros (custom derive, attribute-like, and
function-like) all work in a similar fashion
- When creating procedural macros, the definitions must reside in their own crate
with a special crate type
- Using procedural macros looks like the code in
Listing 19-29, where `some_attribute` is a placeholder for using a specific
macro

### How to Write a Custom `derive` Macro
- At the time of this writing,
procedural macros need to be in their own crate
- The convention for structuring crates and macro crates is as
follows: for a crate named `foo`, a custom derive procedural macro crate is
called `foo_derive`
- split the code into the `hello_macro_derive` function, which
is responsible for parsing the `TokenStream`, and the `impl_hello_macro`
function, which is responsible for transforming the syntax tree: this makes
writing a procedural macro more convenient
- The `proc_macro` crate comes with Rust
- The `syn` crate parses Rust code from a string into a data structure that we
can perform operations on. The `quote` crate turns `syn` data structures back
into Rust code
- The `parse` function in
`syn` takes a `TokenStream` and returns a `DeriveInput` struct representing the
parsed Rust code
- It’s necessary for our procedural macro to panic on errors because
`proc_macro_derive` functions must return `TokenStream` rather than `Result` to
conform to the procedural macro API
- The `quote!` macro lets us define the Rust code that we want to return
- The `quote!` macro also provides some very cool templating mechanics: we can
enter `#name`, and `quote!` will replace it with the value in the variable
`name`
- The `stringify!` macro used here is built into Rust. It takes a Rust
expression, such as `1 + 2`, and at compile time turns the expression into a
string literal, such as `"1 + 2"`
- Using `stringify!` also
saves an allocation by converting `#name` to a string literal at compile time

### Attribute-like macros
- they allow you to create new attributes
- `derive` only works for structs and enums, attributes can be applied to other items as well, such as functions
- The signature of the macro definition function would look like this:
    ```rust
    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    ```
    - The first is for the
    contents of the attribute: the `GET, "/"` part. The second is the body of the
    item the attribute is attached to: in this case, `fn index() {}` and the rest
    of the function’s body

### Function-like macros
- Function-like macros define macros that look like function calls
- Example
    ```rust
    let sql = sql!(SELECT * FROM posts WHERE id=1);
    ```