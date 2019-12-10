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
```rust
// declare the name of the type parameter inside angle brackets
// just after the name of the struct
// @note x and y must be of the same type
struct Point<T> {
    x: T,
    y: T,
}

// Point with x and y of different types
struct PointV2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let integer_and_float = PointV2 { x: 5, y: 4.0 };
}
```

> When you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces

### In Enum Definitions 
- Examples 
    ```rust
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```

> When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types instead

### In Method Definitions
- Example 
    ```rust
    struct Point<T> {
        x: T,
        y: T,
    }

    // @note T must go just after impl so we can use it to specify that we're implementing methods on the type Point<T>
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    fn main() {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());
    }
    ```
- Example of implement methods only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type
    ```rust
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    ```
- Generic type parameters in a struct definition aren't always the same as those you use in that struct's method signatures
    ```rust
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn main() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
    ```

### Performance of Code Using Generics 
- No penalty w.r.t that with concrete types
- Rust accomplishes this by performing *monomorphization* of the code that is using generics **at compile time**. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

## Traits: Defining Shared Behavior 
- A trait tells the Rust compiler about functionality a particular type has and can share with other types - We can use trait bounds to specify that a generic can be any type that has certain behavior

> Traits are similar to a feature often called interfaces in other languages, although with some differences

### Defining a Trait
- Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose
- Example 
    ```rust
    // Summary is a media aggregator library that can display summaries of data that might be stored in a NewsArticle or Tweet instance
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    ```
- A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon
### Implementing a Trait on a Type
- Example 
    ```rust
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // Implementing a trait on a type is similar to implementing regular methods.
    // The difference is that after `impl`, we
    //  1. put the trait name that we want to implement
    //  2. then use the `for` keyword,
    //  3. then specify the name of the type we want to implement the trait for
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    fn main() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }
    ```
- One restriction 
  - A trait can be implemented on a type **only if either the trait or the type is local to our crate**
    > This restriction is part of a property of programs called **coherence** (a.k.a. **the orphan rule**), which ensures that other people's code can't break your code and vice versa
### Default Implementations 
- Why: offering default implementations in trait makes it optional to override if we're comfortable with the default one
- Example 
    ```rust
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
    }

    fn main() {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best
    hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());

        // Output:
        // New article available! (Read more...)
    }
    ```
- Restriction: it is impossible to call the default implementation from an overriding implementation of that same method
### Traits as Parameters 

- Example 
```rust
// notify demonstrates the impl Trait syntax, which specifies the item accepts any type that implements the Summary trait
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
- Fix as
    ```rust
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }
    ```
- Alternative solutions
  - Replace `Copy` with `Clone`, which would iccur more heap allocations
  - Return a reference to a `T` value in the slice
### Using Trait Bounds to Conditionally Implement Methods
- Example
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // Pair<T> only implements the cmp_display method if its inner type `T` implements the `PartialOrd` trait that enables comparison and the `Display` trait that enables printing
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
- Blanket implementations: implementating a trait on any type that satisfies the trait bounds
```rust
// std library
impl<T: Display> ToString for T {
    // --snip--
}

// which makes below possible 
let s = 3.to_string();
```
## Validating References with Lifetimes 
- Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred
- **WHEN**: way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways
### Preventing Dangling References with Lifetimes
- The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it's intended to reference
```rust
// This code won't compile because the value r is referring to has gone out of scope before we try to use it.
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

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
- Non-compilable example
```rust
// longest failed because the borrow checker doesn’t know how the lifetimes of x and y relate to the lifetime of the return value
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```
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
- Example 
    ```rust
    // @note declare generic lifetime parameters inside angle brackets between the function name and the parameter list
    // @dev The constraint we want to express in this signature is that all the references in the parameters and the return value must have the same lifetime
    // @dev the lifetime of the returned value will be the overlapping scope of x and y
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```
- When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. Rust can analyze the code within the function without any help
- When a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes might be different each time the function is called. This is why we need to annotate the lifetimes manually
- Examples
    ```rust
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // ok
    // fn main() {
    //     let string1 = String::from("long string is long");
    //
    //     {
    //         let string2 = String::from("xyz");
    //         let result = longest(string1.as_str(), string2.as_str());
    //         println!("The longest string is {}", result);
    //     }
    // }

    // not ok
    fn main() {
        let string1 = String::from("long string is long");
        let result;

        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }

        println!("The longest string is {}", result);
    }
    ```
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
### Lifetime Annotations in Struct Definitions
- It's possible for structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct's definition

- Example 
    ```rust
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");

        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    ```

### Lifetime Elision 
- **WHAT**: the patterns programmed into Rust's analysis of references
    > They're a set of particular cases that the compiler will consider, and if your code fits these cases, you don't need to write the lifetimes explicitly
- The elision rules don't provide full inference. If with all rules applied still produces ambiguity as to what lifetimes the references have, the compiler would just error out 
- Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes
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