# Chapter 04. Understanding Ownership

## What Is Ownership?
- 3 ways of memory management
    - Periodic garbage collection
    - Explicitly allocate and free
    - For Rust, it's managed through a system of ownership with a set of rules that the compiler
        checks at compile time

> ### The Stack and the Heap
> - All data stored on the stack must have a known, fixed size. Data with an unknown size at compile
>   time or a size that might change must be stored on the heap instead
> - Pushing to the stack is faster than allocating on the heap because the operating system never
    has to search for a place to store new data; that location is always at the top of the stack
> - Accessing data in the heap is slower than accessing data on the stack because you have to follow
    a pointer to get there
> - Keeping track of what parts of code are using what data on the heap, minimizing the amount of
    duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of
    space are all problems that ownership addresses

### Ownership Rules
* Each value in Rust has a variable that's called its *owner*.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

### Variable Scope
- The variable is valid from the point at which it's declared until the end of the current *scope*
  - Example as [Listing 4-1: A variable and the scope in which it is valid](listings/_01/src/main.rs)

### The String Type
- String literals are convenient, but they are unsuitable for every situation in which we may want
  to use text, because 
    - They're immutable
    - Not every string value can be known when we write our code
      - e.g., what if we want to take user input and store it?

### Memory and Allocation 
- In languages with a *garbage collector (GC)*, the GC keeps track and cleans up memory that isn't
  being used anymore, and we don't need to think about it
- Without a GC, it's our responsibility to identify when memory is no longer being used and call
  code to explicitly return it, just as we did to request it
    - Doing this correctly has historically been a difficult programming problem
- In Rust, the memory is automatically returned once the variable that owns it goes out of scope
    - When a variable goes out of scope, Rust calls a special function for us

    > Note: In C++, this pattern of deallocating resources at the end of an item's
    > lifetime is sometimes called *Resource Acquisition Is Initialization (RAII)*.
    > The `drop` function in Rust will be familiar to you if you've used RAII
    > patterns.

#### Ways Variables and Data Interact: Move
- Example
    ```rust
    let s1 = String::from("hello");
    // Rust **move** s1 into s2, and **invalidates** s1.
    // Memory pointed by s1 wouldn't need freeing when s1 is out of scope
    let s2 = s1;

    // calling below would error out because Rust prevents you from using the invalidated reference
    // println!("{}, world!", s1);
    ```

#### Ways Variables and Data Interact: Clone
```rust
let s1 = String::from("hello");
// s2 is deep copy of s1
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

#### Stack-Only Data: Copy
- Example 
    ```rust
    let x = 5;
    // x is copy to y
    let y = x;

    println!("x = {}, y = {}", x, y);
    ```
    - Types such as integers that have a known size at compile time are stored entirely on the
        stack, so copies of the actual values are quick to make
- `Copy` trait
  - we can place on types like integers that are stored on the stack (more about traits in [Chapter 10](../chapter10/README.md))
  - If a type has the `Copy` trait, an older variable is still usable after assignment
  - Rust won't let us annotate a type with the `Copy` trait if the type, or any of its parts, has
    implemented the `Drop` trait
  - If the type needs something special to happen when the value goes out of scope and we add the
    `Copy` annotation to that type, we'll get a compile-time error
- As a general rule, any group of simple scalar values can be `Copy`, and nothing that requires
  allocation or is some form of resource is `Copy`
- `Copy` examples
    * All the integer types, such as `u32`.
    * The Boolean type, `bool`, with values `true` and `false`.
    * All the floating point types, such as `f64`.
    * The character type, `char`.
    * Tuples, if they only contain types that are also `Copy`. For example, `(i32, i32)` is `Copy`,
        but `(i32, String)` is not.

### Ownership and Functions
- Passing a variable to a function will move or copy, just as assignment does
    - Example as [Listing 4-3: Functions with ownership and scope annotated](listings/_03/src/main.rs)

### Return Values and Scope
- The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it
- When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless the data has been moved to be owned by another variable
    - Example as [Listing 4-4: Transferring ownership of return values](listings/_04/src/main.rs)
- One workaround would be **moving** the input parameter to one of return values as [Listing 4-5: Returning ownership of parameters](listings/_05/src/main.rs)

## References and Borrowing
- References allow you to refer to some value without taking ownership of it
- Having references as function parameters is called **borrowing**
- Modifying borrowed immutable values wouldn't work
  - Example as [Listing 4-6: Attempting to modify a borrowed value](listings/_06/src/main.rs)
- References are immutable by default

### Mutable References
- One big retriction: you can have only one mutable reference to a particular piece of data in a
  particular scope
    - The benefit of having this restriction is that Rust can prevent data races at compile time
-  A *data race* is similar to a race condition and happens when these three behaviors occur:
    * Two or more pointers access the same data at the same time.
    * At least one of the pointers is being used to write to the data.
    * There's no mechanism being used to synchronize access to the data.
- We *also* cannot have a mutable reference while we have an immutable one
- Multiple immutable references are okay

### Dangling References
- *Dangling pointer* references a location in memory that may have been given to someone else, by
  freeing some memory while preserving a pointer to that memory
- In Rust, if you have a reference to some data, the compiler will ensure that the data will not go
  out of scope before the reference to the data does

### The Rules of References
- At any given time, you can have *either* one mutable reference *or* any number of immutable
  references.
- References must always be valid.

## The Slice Type
- Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection

### A small programming problem
- Requirement: write a function that takes a string and returns the first word it finds in that
  string. If the function doesn't find a space in the string, the whole string must be one word, so
  the entire string should be returned.
- Level-1 solution as [Listing 4-8: Storing the result from calling the first_word function and then changing the String contents](listings/_08/src/main.rs)

### String Slices
- A *string slice* is a reference to part of a `String`, and it looks like this:

    ```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    ```
- We can create slices using a range within brackets by specifying `[starting_index..ending_index]`,
  where `starting_index` (if omitted, default as 0) is the first position in the slice and 
  `ending_index` (if omitted, default as the length of being slicing) is one more than the last
  position in the slice 
    > Note: String slice range indices must occur at valid UTF-8 character
    > boundaries. If you attempt to create a string slice in the middle of a
    > multibyte character, your program will exit with an error

- Level-2 solution for the *small programming problem* as [string-slices](listings/string-slices/src/main.rs)

- String literals are slices

### String Slices as Parameters 

- Level-3 solution for the *small programming problem* as [Listing 4-9: Improving the first_word function by using a string slice for the type of the s parameter](listings/_09/src/main.rs)

### Other Slices

```rust
// default type as [i32]
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```
