# Chapter 10. Generic types, traits, and lifetimes

## Removing Duplication by Extracting a Function
- Goal: finds the largest number in a list
- Example
    - [Listing 10-1: Code to find the largest number in a list of numbers](./listings/_01/src/main.rs)
    - [Listing 10-2: Code to find the largest number in two lists of numbers](./listings/_02/src/main.rs)
    - [Listing 10-3: Abstracted code to find the largest number in two lists](./listings/_03/src/main.rs)
- Lession: the steps taken to change the code from listing 10-1 to 10-3
    1. Identify duplicate code.
    2. Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
    3. Update the two instances of duplicated code to call the function instead.

## Generic Data Types 
### In Function Definitions 
- When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value 
- Example without generic：[Listing 10-4: Two functions that differ only in their names and the types in their signatures](./listings/_04/src/main.rs)
- Example with generic：[Listing 10-5: A definition of the largest function that uses generic type parameters but doesn’t compile yet](listings/_05/src/main.rs)

### In Struct Definitions 

Example as [Listing 10-6 to 10-8](./listings/_06_07_08/src/main.rs)

> When you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces

### In Enum Definitions 
- Examples 
    ```rust
    enum Option<T> {
        Some(T),
        None,
    }

    // Enums can use multiple generic types as well
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```

> When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types instead

### In Method Definitions
- Example as [Listing 10-9: Implementing a method named x on the Point<T> struct that will return areference to the x field of type T](./listings/_09/src/main.rs)
- Example of implement methods only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type
    ```rust
    // Listing 10-10: An impl block that only applies to a struct with a particular concrete type
    // for the generic type parameter T
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    ```
- Generic type parameters in a struct definition aren't always the same as those you use in that struct's method signatures as [Listing 10-11: A method that uses different generic types than its struct’s definition](./listings/_11/src/main.rs)

### Performance of Code Using Generics 
- No penalty w.r.t that with concrete types
- Rust accomplishes this by performing *monomorphization* of the code that is using generics **at compile time**
  - Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

## Traits: Defining Shared Behavior 
- A trait tells the Rust compiler about functionality a particular type has and can share with other types 
- We can use trait bounds to specify that a generic can be any type that has certain behavior

> Traits are similar to a feature often called interfaces in other languages, although with some differences

### Defining a Trait
- Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose
- Example 
    ```rust
    // Listing 10-12: A Summary trait that consists of the behavior provided by a summarize method
    // Summary is a media aggregator library that can display summaries of data that might be stored
    // in a NewsArticle or Tweet instance
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    ```
- A trait can have multiple methods in its body 
  - The method signatures are listed one per line 
  - Each line ends in a semicolon

### Implementing a Trait on a Type
- Example as [Listing 10-13: Implementing the Summary trait on the NewsArticle and Tweet types](./listings/_13/src/main.rs)
- One restriction 
  - A trait can be implemented on a type **only if either the trait or the type is local to our crate**
    > This restriction is part of a property of programs called **coherence** (a.k.a. **the orphan rule**), which ensures that other people's code can't break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use
### Default Implementations 
- Why: offering default implementations in trait makes it optional to override if we're comfortable with the default one
- Example as [Listing 10-14: Definition of a Summary trait with a default implementation of the summarize method](./listings/_14/src/main.rs)
- Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation
    - Example as [Listing 10-14-2](./listings/_14_2/src/main.rs)
- Restriction: it is impossible to call the default implementation from an overriding implementation of that same method
### Traits as Parameters 

- Example 
    ```rust
    // notify demonstrates the impl Trait syntax, which specifies the item accepts any type that 
    // implements the Summary trait
    pub fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    ```

#### Trait Bound Syntax
- The impl Trait syntax is actually syntax sugar for a longer form called *trait bound* like this
    ```rust
    pub fn notify<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }
    ```
- Why trait bound: to force both parameters to have the same type, like this:
    ```rust
    pub fn notify<T: Summary>(item1: T, item2: T) {
    ```
#### Specifying Multiple Trait Bounds with the `+` Syntax
```rust
// notify requires item implements both Summary and Display
pub fn notify(item: impl Summary + Display) {
    ...
}

// alternatively as 
pub fn notify<T: Summary + Display>(item: T) {
    ...
}
```
#### Clearer Trait Bounds with `where` Clauses
- Why: functions with multiple generic type parameters can contain lots of trait bound information between the function's name and its parameter list, making the function signature hard to read
- `where` clause goes as 
```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where 
        T: Display + Clone,
        U: Clone + Debug
{
    ...
}

// which is equivalent to 
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    ...
}
```

### Returning Types that Implement Traits
- Only for returning a single type
    ```rust
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
    ```
- Failed for
    ```rust
    fn returns_summarizable(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                ...
            }
        } else {
            Tweet {
                ...
            }
        }
    }
    ```

### Fixing the `largest` Function with Trait Bounds
- Cause
  - Types like `i32` and `char` that have a known size can be stored on the stack, so they implement the `Copy` trait
  - When we made the largest function generic, it became possible for the list parameter to have types in it that don't implement the `Copy` trait
- Fix as [Listing 10-15: A working definition of the largest function that works on any generic type that implements the PartialOrd and Copy traits](./listings/_15/src/main.rs)
- Alternative solutions
  - Replace `Copy` with `Clone`, which would iccur more heap allocations
  - Return a reference to a `T` value in the slice
### Using Trait Bounds to Conditionally Implement Methods
- Example as [Listing 10-16: Conditionally implement methods on a generic type depending on trait bounds](listings/_16/src/main.rs)
- Blanket implementations: implementating a trait on any type that satisfies the trait bounds
    ```rust
    // std library
    impl<T: Display> ToString for T {
        // --snip--
    }

    // which makes below possible 
    let s = 3.to_string();
    ```
- Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior

## Validating References with Lifetimes 
- **WHAT**: the scope for which that reference is valid
- Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred
- **WHEN**: we must annotate lifetimes when the lifetimes of references could be related in a few different ways
- Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid
### Preventing Dangling References with Lifetimes
- The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it's intended to reference
    - Example as [Listing 10-17: An attempt to use a reference whose value has gone out of scope](./listings/_17/src/main.rs)
- If we try to use a variable before giving it a value, we'll get a compile-time error, which shows that Rust indeed does not allow null values

### The Borrow Checker 
- The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid
- Lifetime analysis
    - Error
        ```rust
        // r live long as 'a, but x just 'b<'a
        fn main() {
            let r;                  // ---------+-- 'a
                                    //          |
            {                       //          |
                let x = 5;          // -+-- 'b  |
                r = &x;             // |        |
            }                       // -+       |
                                    //          |
            println!("r: {}", r);   //          |
        }                           // ---------+
        ```
    - Possible fix
        ```rust
        // Listing 10-19: A valid reference because the data has a longer lifetime than the
        // reference
        {
            let x = 5;              // ----------+-- 'b
                                    //           |
            let r = &x;             // --+-- 'a  |
                                    //   |       |
            println!("r: {}", r);   //   |       |
                                    // --+       |
        }                           // ----------+       
        ```

### Generic Lifetimes in Functions 
- Non-compilable example：[Listing 10-21: A main function that calls the longest function to find the longer of two string slices](listings/_20/src/main.rs)

### Lifetime Annotation Syntax 
- Lifetime annotations don't change how long any of the references live
- Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes
- Syntax: the names of lifetime parameters must start with a single quote (`'`) and are usually all lowercase and very short, like generic types, usually as `'a`
- We place lifetime parameter annotations after the `&` of a reference, using a space to separate the annotation from the reference's type
    ```rust
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
    ```
### Lifetime Annotations in Function Signatures
- Example as [Listing 10-22: The longest function definition specifying that all the references in the signature must have the same lifetime 'a](./listings/_22/src/lib.rs)
- When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. Rust can analyze the code within the function without any help
- When a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes might be different each time the function is called. This is why we need to annotate the lifetimes manually
- Examples
    - [Listing 10-23: Using the longest function with references to String values that have different concrete lifetimes](./listings/_23/src/main.rs)
    - [Listing 10-24: Attempting to use result after string2 has gone out of scope](./listings/_24/src/main.rs)

### Thinking in Terms of Lifetimes 
```rust
// ok
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// not ok
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```
- Lifetime syntax is about connecting the lifetimes of various parameters and return values of functions

### Lifetime Annotations in Struct Definitions
- It's possible for structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct's definition
- Example as [Listing 10-25: A struct that holds a reference, so its definition needs a lifetime annotation](./listings/_25/src/main.rs)

### Lifetime Elision 
- Example as [Listing 10-26: A function we defined in Listing 4-9 that compiled without lifetime annotations, even though the parameter and return type are references](./listings/_26/src/lib.rs)
- **WHAT**: the patterns programmed into Rust's analysis of references
    > They're a set of particular cases that the compiler will consider, and if your code fits these cases, you don't need to write the lifetimes explicitly
- The elision rules don't provide full inference. If with all rules applied still produces ambiguity as to what lifetimes the references have, the compiler would just error out 
- Lifetimes on function or method parameters are called **input lifetimes**, and lifetimes on return values are called **output lifetimes**
- The compiler uses three rules to figure out what lifetimes references have when there aren't explicit annotations
    1. Each parameter that is a reference gets its own lifetime parameter
        ```rust
        fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
        ```
    2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
        ```rust
        fn foo<'a>(x: &'a i32) -> &'a i32
        ```
    3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters
        ```rust
        impl<'a> ImportantExcerpt<'a> {
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
        ```
### Lifetime Annotations in Method Definitions 
- Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct's name, because those lifetimes are part of the struct's type
    ```rust
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    ```
### The Static Lifetime 
- Decorated with `'static`, denoting the entire duration of the program
- All string literals have the `'static` lifetime
    ```rust
    let s: &'static str = "I have a static lifetime.";
    ```
## Generic Type Parameters, Trait Bounds, and Lifetimes Together 
```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    // println! requires ann of T impl Display
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```