# Chapter 13. Functional Language Features: Iterators and Closures

## Closures: Anonymous Functions that Can Capture Their Environment
- Rust’s closures are anonymous functions you can save in a variable or pass as
arguments to other functions
- Closures can capture values from the scope in which they’re defined

### Creating an Abstraction of Behavior with Closures
- Examples as 13-[1-3]

#### Refactoring Using Functions
- Example 

#### Refactoring with Closures to Store Code
- Example 
- To define a closure
  - We start with a pair of vertical pipes (`|`), inside which we specify the parameters separated with commas to the closure
  - After the parameters, we place curly brackets that hold the body of the
closure—these are optional if the closure body is a single expression
- Example 

### Closure Type Inference and Annotation
- Closures don’t require you to annotate the types of the parameters or the
return value like `fn` functions do. Type annotations are required on functions
because they’re part of an explicit interface exposed to your users
- We can add type annotations if we want to increase
explicitness and clarity at the cost of being more verbose than is strictly necessary
  - Example 
- Closure definitions will have one concrete type inferred for each of their
parameters and for their return value
- Example 
    - Parameter types are then locked in to the closure in `example_closure`, and we get a type error if we try to use a different type with the same closure

### Storing Closures Using Generic Parameters and the `Fn` Traits
- *memoization*/*lazy evaluation*
  - We can create a struct that will hold the closure and the resulting value of calling the closure
  - The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result
- Each closure instance has its own unique anonymous type: that is, even
if two closures have the same signature, their types are still considered different
- All closures implement at least one of the traits: `Fn`, `FnMut`, or `FnOnce`
- Examples as 13-[9-11]

### Limitations of the `Cacher` Implementation
- A `Cacher` instance assumes it will always get the same value for the parameter `arg` to the `value` method
  - Failed example
  - Solution: try modifying `Cacher` to hold a hash map rather than a single value
- The current `Cacher` implementation is that it only
accepts closures that take one parameter of type `u32` and return a `u32`

### Capturing the Environment with Closures
- Example 
- When a closure captures a value from its environment, it uses memory to store
the values for use in the closure body. This use of memory is overhead that we
don’t want to pay in more common cases where we want to execute code that
doesn’t capture its environment
- Closures can capture values from their environment in three ways, which
directly map to the three ways a function can take a parameter: taking
ownership, borrowing mutably, and borrowing immutably. These are encoded in the
three `Fn` traits as follows:

    * `FnOnce` consumes the variables it captures from its enclosing scope, known
    as the closure’s *environment*. To consume the captured variables, the
    closure must take ownership of these variables and move them into the closure
    when it is defined. The `Once` part of the name represents the fact that the
    closure can’t take ownership of the same variables more than once, so it can
    be called only once.
    * `FnMut` can change the environment because it mutably borrows values.
    * `Fn` borrows values from the environment immutably.

- When you create a closure, Rust infers which trait to use based on how the
closure uses the values from the environment
- If you want to force the closure to take ownership of the values it uses in the
environment, you can use the `move` keyword before the parameter list
    - Example 

## Processing a Series of Items with Iterators
- The iterator pattern allows you to perform some task on a sequence of items in turn
- An iterator is responsible for the logic of iterating over each item and
determining when the sequence has finished. When you use iterators, you don’t
have to reimplement that logic yourself
- Iterators are *lazy*, meaning they have no effect until you call
methods that consume the iterator to use it up
- Example 13-[13-14]

### The `Iterator` Trait and the `next` Method
- Trait
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```
- The `Iterator` trait only requires implementors to define one method: the
`next` method, which returns one item of the iterator at a time wrapped in
`Some` and, when iteration is over, returns `None`
- Example 
    - we needed to make `v1_iter` mutable: calling the `next` method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence
    - We didn’t need to make `v1_iter` mutable when we used a `for` loop because the loop took ownership of `v1_iter` and made it mutable behind the scenes
- The `iter` method produces an iterator over immutable references
    - If we want to create an iterator that takes ownership of `v1` and returns owned values, we can call `into_iter` instead of `iter`
    - If we want to iterate over mutable references, we can call `iter_mut` instead of `iter`

### Methods that Consume the Iterator
- Methods that call `next` are called *consuming adaptors*, because calling them uses up the iterator
- Example

### Methods that Produce Other Iterators
- Other methods defined on the `Iterator` trait, known as *iterator adaptors*, allow you to change iterators into different kinds of iterators
- You can chain multiple calls to iterator adaptors to perform complex actions in a readable way
    - Because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors
- Example 13-18

### Using Closures that Capture Their Environment
- The `filter` method on an iterator takes a closure that takes each item from
the iterator and returns a Boolean
- Example

### Creating Our Own Iterators with the `Iterator` Trait
- Example as 13-[20-21]

#### Using Our `Counter` Iterator’s `next` Method
- Example 

#### Using Other `Iterator` Trait Methods
- Example

## Improving Our I/O Project
### Removing a `clone` Using an Iterator
- Reproduced example
- How
    - Using the Returned Iterator Directly
    - Using `Iterator` Trait Methods Instead of Indexing
- Final example

### Making Code Clearer with Iterator Adaptors
- The functional programming style prefers to minimize the amount of mutable state to make code clearer
  - Example

## Comparing Performance: Loops vs. Iterators
- Iterators are one of Rust’s *zero-cost abstractions*, by which we mean using the abstraction
imposes no additional runtime overhead