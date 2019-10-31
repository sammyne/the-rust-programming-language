# Chapter 06. Enums and Pattern Matching 

## Defining an Enum
- Example 
    ```rust
    enum IpAddrKind {
        V4,
        V6,
    }
    ```
    - where `V4` and `V6` are called the *variants* of the enum `IpAddrKind`

### Enum Values
- The variants of the enum are namespaced under its identifier, and we use a double colon to separate the two
- Example as 
    ```rust
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    ```
- We can represent the same concept in a more concise way using just an enum,
rather than an enum inside a struct, by putting data directly into each enum
variant
    - Redundant implementation using `struct` as [listing 01](listings/_01/src/main.rs)
    - Concise implementation using enum as 
        ```rust
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));

        let loopback = IpAddr::V6(String::from("::1"));
        ```
- There's another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data
    - Example as 
        ```rust
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let home = IpAddr::V4(127, 0, 0, 1);

        let loopback = IpAddr::V6(String::from("::1"));
        ```
    - Another example as [listing 02](listings/_02/src/main.rs), where the alternative implementation using struct would produce different types.
- Enum can have methods
    ```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
    ```

### The `Option` Enum and Its Advantages over Null Values
- `Option` encodes the very common scenario in which a value could be something or it could be nothing
- Prototype as 
    ```rust
    enum Option<T> {
        Some(T),
        None,
    }
    ```
- The `Option<T>` enum is so useful that it's even included in the prelude; you don't need to bring it into scope explicitly
- because `Option<T>` and `T` (where `T` can be any type) are different types, the compiler won't let us use an `Option<T>` value as if it were definitely a valid value
    - Example as [option](listings/option/src/main.rs)
- One of the most common issues with null: assuming that something isn't null when it actually is
- In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value `Option<T>`. Then, when you use that value, you are required to explicitly handle the case when the value is null. Everywhere that a value has a type that isn’t an `Option<T>`, you *can* safely assume that the value isn’t null

## The `match` Control Flow Operator
- `match` allows you to compare a value against a series of patterns and then execute code based on which pattern matches
- Patterns can be made up of literal values, variable names, wildcards, and many other things
- Example as [listing 03](listings/_03/src/main.rs)
    - A match arm has two parts: a pattern and some code. The first arm here has a pattern that is the value `Coin::Penny` and then the `=>` operator that separates the pattern and the code to run
    - Each arm is separated from the next with a comma
    - When the `match` expression executes, it compares the resulting value against the pattern of each arm, in order. If a pattern matches the value, the code associated with that pattern is executed. If that pattern doesn't match the value, execution continues to the next arm
    - The code associated with each arm is an expression, and the resulting value of the expression in the matching arm is the value that gets returned for the entire `match` expression
    - If you want to run multiple lines of code in a match arm, you can use curly brackets
        ```rust
        enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky penny!");
                    1
                },
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
        ```
  
### Patterns That Bind to Values
- Another useful feature of match arms is that they can bind to the parts of the values that match the pattern
- Example as [listing 04](listings/_04/src/main.rs)

### Matching with `Option<T>`
- Example as [listing 05](listings/_05/src/main.rs)
- A common pattern a lot in Rust code: `match` against an enum, bind a variable to the data inside, and then execute code based on it

### Matches Are Exhaustive
- Example as [matches_are_exhaustive](listings/matches_are_exhaustive/src/main.rs)
- We must exhaust every last possibility in order for the code to be valid

### The `_` Placeholder 
- The `_` pattern will match any value that aren't specified before it
- Example as [the_underline_placeholder](listings/the_underline_placeholder/src/main.rs)

## Concise Control Flow with `if let`
- The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest
- Example using `match` as [listing 06](listings/_06/src/main.rs)
- The syntax `if let` takes a pattern and an expression separated by an equal sign, as 
- We can include an `else` with an `if let`. The block of code that goes with the `else` is the same as the block of code that would go with the `_` case in the `match` expression that is equivalent to the `if let` and `else`
- Example as [if_let](listings/if_let/src/main.rs)