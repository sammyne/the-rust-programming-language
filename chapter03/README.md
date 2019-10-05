# Chapter 03. Common Programming Concepts

> **Keywords**
> - The Rust language has a set of keywords that are reserved for use by the language only.
> - You cannot use these words as names of variables or functions.

## Variables and Mutability
- When a variable is immutable, once a value is bound to a name, you can’t change that value. [Example](./variables/src/bin/error.rs) as 
    ```bash
    cargo run -p variables --bin error
    ```
- Add `mut` in front of the variable name to make it mutable. [Example](./variables/src/bin/ok.rs) as 
    ```bash
    cargo run -p variables --bin ok
    ```

### Differences Between Variables and Constants
- You aren't allowed to use `mut` with constants. Constants aren’t just immutable by default—they’re always immutable
- You declare constants using the `const` keyword instead of the `let` keyword,
and the type of the value *must* be annotated
- Constants can be declared in any scope, including the global scope, which makes
them useful for values that many parts of code need to know about
- constants may be set only to a constant expression,
not the result of a function call or any other value that could only be
computed at runtime

> Rust's naming convention for
> constants is to use all uppercase with underscores between words, and
> underscores can be inserted in numeric literals to improve readability

- Constants are valid for the entire time a program runs, within the scope they
were declared in, making them a useful choice for values in your application
domain that multiple parts of the program might need to know about
- Naming hardcoded values used throughout your program as constants is useful in
    - Conveying the meaning of that value to future maintainers of the code
    - Need to change if the hardcoded value needed to be updated in the future

### Shadowing

- Shadow a variable by using the same variable’s name and repeating the use
of the `let` keyword as [example](./variables/src/bin/shadow.rs)
    ```bash
    cargo run -p variables --bin shadow
    ```
- Shadowing is different from marking a variable as `mut`, because we’ll get a
compile-time error if we accidentally try to reassign to this variable without
using the `let` keyword
- The other difference between `mut` and shadowing is that because we’re
effectively creating a new variable when we use the `let` keyword again, we can
change the type of the value but reuse the same name

## Data Types
- 2 types: scalar and compound
- Keep in mind that Rust is a *statically typed* language, which means that it
must know the types of all variables at compile time. The compiler can usually
infer what type we want to use based on the value and how we use it. In cases
when many types are possible, we must add a type annotation

### Scalar Types
- A *scalar* type represents a single value. Rust has four primary scalar types:
integers, floating-point numbers, Booleans, and characters

#### Integer Types
- An *integer* is a number without a fractional component
- Integer Types in Rust

    | Length  | Signed  | Unsigned |
    | ------- | ------- | -------- |
    | 8-bit   | `i8`    | `u8`     |
    | 16-bit  | `i16`   | `u16`    |
    | 32-bit  | `i32`   | `u32`    |
    | 64-bit  | `i64`   | `u64`    |
    | 128-bit | `i128`  | `u128`   |
    | arch    | `isize` | `usize`  |
- All number literals except the byte literal allow a type suffix, such as
`57u8`, and `_` as a visual separator, such as `1_000`, and more as 

    | Number literals  | Example       |
    | ---------------- | ------------- |
    | Decimal          | `98_222`      |
    | Hex              | `0xff`        |
    | Octal            | `0o77`        |
    | Binary           | `0b1111_0000` |
    | Byte (`u8` only) | `b'A'`        |
- integer types default to `i32`
- The primary situation in which you’d use `isize` or `usize` is when indexing some sort of collection

> **Integer Overflow**
> if overflow occurs, Rust performs *two’s complement wrapping*. In short, values
> greater than the maximum value the type can hold “wrap around” to the minimum
> of the values the type can hold. In the case of a `u8`, 256 becomes 0, 257
> becomes 1, and so on.  
> If you want to
> wrap explicitly, you can use the standard library type `Wrapping`

#### Floating-Point Types
- Rust's floating-point types are `f32` and `f64`,
which are 32 bits and 64 bits in size, respectively. The default type is `f64`

#### Numeric Operations
```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

#### The Boolean Type
- Two possible values: `true` and `false`
    ```rust
    fn main() {
        let t = true;

        let f: bool = false; // with explicit type annotation
    }
    ```
- Booleans are one byte in size

#### The Character Type
- Rust's `char` type is the language’s most primitive alphabetic type
- Specified with
single quotes, as opposed to string literals, which use double quotes
- Example
    ```rust
    fn main() {
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
    }
    ```
- Rust's `char` type is four bytes in size and represents a Unicode Scalar Value,
which means it can represent a lot more than just ASCII

### Compound Types 
- Two primitive compound types: tuples and arrays

#### The Tuple Type
- A tuple is a general way of grouping together some number of other values
with a variety of types into one compound type
- Tuples have a fixed length
- Create a tuple by writing a comma-separated list of values inside parentheses
- Example 
    ```rust
    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }
    ```
- To get the individual values out of a tuple, we can
use pattern matching to **destructure** a tuple value, like this:
    ```rust
    fn main() {
        let tup = (500, 6.4, 1);

        let (x, y, z) = tup;

        println!("The value of y is: {}", y);
    }
    ```
- access a tuple
element directly by using a period (`.`) followed by the index of the value we
want to access, like this
    ```rust
    fn main() {
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
    }
    ```

#### The Array Type
- every element of an array must have the same type
- arrays in Rust have a fixed length
- the values going into an array are written as a comma-separated list
inside square brackets
    ```rust
    fn main() {
        let a = [1, 2, 3, 4, 5];
    }
    ```
- Arrays are useful when you want your data allocated on the stack rather than
the heap or when you want to ensure you always have a fixed number of elements
- A vector is a similar collection type
provided by the standard library that *is* allowed to grow or shrink in size
- Example use case: representing the names of the months of the year
- write an array’s type by using square brackets, and within the
brackets include the type of each element, a semicolon, and then the number of
elements in the array, like so:
    ```rust
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    ```
- if you want to create an array that contains the same
value for each element, you can specify the initial value, followed by a
semicolon, and then the length of the array in square brackets, as shown here
    ```rust
    let a = [3; 5];
    ```
##### Accessing Array Elements
- access elements of an array using indexing, like this
    ```rust
    fn main() {
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];
    }
    ```
##### Invalid Array Element Access
- Error out due to index out of bound as [example](./arrays/src/main.rs)
    ```bash
    cargo run -p arrays
    ```
- When you attempt to access an
element using indexing, Rust will check that the index you’ve specified is less
than the array length. If the index is greater than or equal to the array
length, Rust will panic

## Functions

- Rust code uses snake case as the conventional style for function and variable names. In snake case, all letters are lowercase and underscores separate words
- [Example](./functions/src/bin/hello_world.rs) as 
    ```bash
    cargo run -p functions --bin hello_world
    ```
- Function definitions in Rust start with fn and have a set of parentheses after the function name. The curly brackets tell the compiler where the function body begins and ends

### Function Parameters

- parameters, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called arguments
- [Example](./functions/src/bin/one_param.rs) as 
    ```bash
    cargo run -p functions --bin one_param
    ```
- must declare the type of each parameter
- multiple parameters, separate the parameter declarations with commas, like [example](./functions/src/bin/two_params.rs)
    ```bash
    cargo run -p functions --bin two_params
    ```

### Statements and Expressions in Function Bodies
- Function bodies are made up of a series of statements optionally ending in an expression
- Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value
- Calling a function is an expression. Calling a macro is an expression. The block that we use to create new scopes, `{}`, is an expression
- Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement

### Functions with Return Values
- We don’t name return
values, but we do declare their type after an arrow (`->`)
- the return
value of the function is synonymous with the value of the final expression in
the block of the body of a function. You can return early from a function by
using the `return` keyword and specifying a value, but most functions return
the last expression implicitly
- [Example](./functions/src/bin/return_values.rs) as 
    ```bash
    cargo run -p functions --bin return_values
    ```
- Another [example](./functions/src/bin/return_values_plus_one.rs) as 
    ```bash
    cargo run -p functions --bin return_values_plus_one
    ```

    > statements don’t evaluate to a value, which is expressed by `()`, an empty tuple

## Comments
- programmers leave notes, or
*comments*, in their source code that the compiler will ignore but people
reading the source code may find useful. Example as 
    ```rust
    // hello, world
    ```
- In Rust, comments must start with two slashes and continue until the end of the
line. For comments that extend beyond a single line, you’ll need to include
`//` on each line, like this
    ```rust
    // So we’re doing something complicated here, long enough that we need
    // multiple lines of comments to do it! Whew! Hopefully, this comment will
    // explain what’s going on.
    ```
- Comments can also be placed at the end of lines containing code
    ```rust
    fn main() {
        let lucky_number = 7; // I’m feeling lucky today
    }
    ```

## Control Flow
### `if` Expressions
- [Example](./branches/src/bin/main.rs)
    ```bash
    cargo run -p branches
    ```
- Blocks of code associated with the conditions in `if` expressions are
sometimes called *arms*
- the condition checked by `if` *must* be a `bool`. [Example](./branches/src/bin/ non_boolean_condition.rs) as 
    ```bash
    cargo run -p branches --bin non_boolean_condition
    ```
#### Handling Multiple Conditions with `else if`
- [Example](./branches/src/bin/else_if.rs)
    ```bash
    cargo run -p branches --bin else_if
    ```
#### Using `if` in a `let` Statement
- [Example](./branches/src/bin/let_if.rs)
    ```bash
    cargo run -p branches --bin let_if
    ```
- the values that have the potential to be results from each arm of the `if` must be
the same type. [Example](./branches/src/bin/let_if_errors.rs) as
    ```bash
    cargo run -p branches --bin let_if_errors
    ```

### Repetition with Loops
- A loop runs through the code inside the loop
body to the end and then starts immediately back at the beginning

#### Repeating Code with `loop`
- [Example](./loops/src/bin/main.rs) as 
    ```bash
    cargo run -p loops
    ```
- place the `break` keyword within the loop to tell the program when to
stop executing the loop

#### Returning Values from Loops
- One of the uses of a `loop` is to retry an operation you know might fail, such
as checking whether a thread has completed its job
- you can
add the value you want returned after the `break` expression you use to stop
the loop; that value will be returned out of the loop so you can use it. [Example](./loops/src/bin/returning_values.rs) as 
    ```bash
    cargo run -p loops --bin returning_values
    ```
#### Conditional Loops with `while`
- [Example](./loops/src/bin/while.rs)
    ```bash
    cargo run -p loops --bin while
    ```

#### Looping Through a Collection with `for`
- [Example](./loops/src/bin/looping_collection_with_while.rs) with `while`
    ```bash
    cargo run -p loops --bin looping_collection_with_while
    ```
    > this approach is error prone; we could cause the program to panic if the
index length is incorrect. It’s also slow, because the compiler adds runtime
code to perform the conditional check on every element on every iteration
through the loop
- [Example](./loops/src/bin/looping_collection_with_for.rs) with `for`
    ```bash
    cargo run -p loops --bin looping_collection_with_for
    ```
- [Example](./loops/src/bin/range.rs) with `range`
    ```bash
    cargo run -p loops --bin range
    ```
