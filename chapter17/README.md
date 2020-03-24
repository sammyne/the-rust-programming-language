# Chapter 17. Object-oriented programming features of rust

## Characteristics of Object-Oriented Languages 
- OOP languages share certain common characteristics, namely 
  - objects
  - encapsulation
  - inheritance
### Objects Contain Data and Behavior 
> Object-oriented programs are made up of objects. An object packages both **data** and the **procedures** that operate on that data. The procedures are typically called methods or operations. -- The Gang of Four
### Encapsulation That Hides Implementation Details 
- **WHAT**: The implementation details of an object aren't accessible to code using that object    
  - The only way to interact with an object is through its public API
  - Code using the object shouldn't be able to reach into the object's internals and change data or behavior directly
- Rust achieves encapsulation
- Example as [averaged-collection](./averaged-collection/src/lib.rs)

### Inheritance as a Type System and as Code Sharing 
- **WHAT**: A mechanism whereby an object can inherit from another object's definition, thus gaining the parent object's data and behavior without you having to define them again
- Rust lacks inheritance
- 2 merits of inheritance 
  - Code usability, which can be implemented with default trait method in Rust
  - Polymorphism: to enable a child type to be used in the same places as the parent type
    > Rust achieves bounded parametric polymorphism 
    >   - **Generics** to abstract over different possible types 
    >   - **Trait bounds** to impose constraints on what those types must provide

## Using Trait Objects That Allow for Values of Different Types 
- Use case: An example graphical user interface (GUI) tool that iterates through a list of items, calling a `draw` method on each one to draw it to the screen
- Implementation strategy with languages with inheritance, 
    - Define a class named `Component` that has a method named `draw` on it
    - The other classes, such as `Button`, `Image`, and `SelectBox`, would inherit from `Component` and thus inherit then override the `draw` method

### Defining a Trait for Common Behavior
- **WHAT**: A trait object points to both an instance of a type implementing our specified trait as well as a table used to look up trait methods on that type at runtime
- **HOW**: We create a trait object by specifying some sort of pointer, such as a `&` reference or a `Box<T>` smart pointer, then the `dyn` keyword, and then specifying the relevant trait
- Restriction: trait object can hold no data
- A **generic type parameter** can only be substituted with one concrete type at a time, whereas **trait objects** allow for multiple concrete types to fill in for the trait object at runtime
- Example as [`Draw`](./gui/src/bin/main.rs)

### Implementing the Trait 
> Being concerned only with the messages a value responds to rather than the value's concrete type -— is similar to the concept duck typing in dynamically typed languages

- Advantages of using trait objects and Rust's type system: Rust won't compile our code if the values don't implement the traits that the trait objects need
- Example as [Button](./gui/src/lib.rs) and [SelectBox](./gui/src/bin/main.rs)

### Trait Objects Perform Dynamic Dispatch 

- **Static Dispatch**: The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you're calling at compile time
- **Dynamic Dispatch**: the compiler can't tell at compile time which method you're calling, and emits code that at runtime will figure out which method to call
- Cost of dynamic dispatch 
  - runtime cost
  - prevents the compiler from choosing to inline a method's code, which in turn prevents some optimizations

### Object Safety Is Required for Trait Objects 
- A trait is object safe if all the methods defined in the trait have the following properties:
    - The return type isn't `Self`
    - There are no generic type parameters
- The `Self` keyword is an alias for the type we're implementing the traits or methods on
- An example of a trait whose methods are not object safe is the standard library's Clone trait
    ```rust
    pub trait Clone {
        fn clone(&self) -> Self;
    }
    ```

> More details on object safety, see [Rust RFC 255](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md)

## Implementing an Object-Oriented Design Pattern 
- **State Pattern**
  - An object-oriented design pattern
  - Crux: A value has some internal state, which is represented by a set of *state objects*, and the value's behavior changes based on the internal state
  - Each state object is responsible for its own behavior and for governing when it should change into another state
  - The value that holds a state object knows nothing about the different behavior of the states or when to transition between states.
- A demo blog as (prototype codes as [blog](./blog/src/bin/main.rs))
    1. A blog post starts as an empty draft.
    2. When the draft is done, a review of the post is requested.
    3. When the post is approved, it gets published.
    4. Only published blog posts return content to print, so unapproved posts can't accidentally be published.

### Defining Post and Creating a New Instance in the Draft State
see [Listing 17-12: Definition of a Post struct and a new function that creates a new Post instance, a State trait, and a Draft struct](./listinngs/_12/src/lib.rs)

### Storing the Text of the Post Content 
see [Listing 17-13: Implementing the `add_text` method to add text to a post’s `content`](./listinngs/_13/src/lib.rs)

### Ensuring the Content of a Draft Post Is Empty 

see [Listing 17-14: Adding a placeholder implementation for the content method on Post that always returns an empty string slice](./listings/../listinngs/_14/src/lib.rs)

### Requesting a Review of the Post Changes Its State
- Example code as [Listing 17-15: Implementing request_review methods on Post and the State trait](./listings/../listinngs/_15/src/lib.rs)
- Note: rather than having `self`, `&self`, or `&mut self` as the first parameter of the method, we have self: `Box<Self>`. This syntax means the method is only valid when called on a `Box` holding the type

### Adding the approve Method that Changes the Behavior of content 
- Examples
  - [Listing 17-16: Implementing the approve method on Post and the State trait](./listinngs/_16/src/lib.rs)
  - [Listing 17-17: Updating the content method on Post to delegate to a content method on State](./listinngs/_17/src/lib.rs)
  - [Listing 17-18: Adding the content method to the State trait](./listinngs/_18/src/lib.rs)

### Trade-offs of the State Pattern 
- To see the simplicity of maintaining code that uses the state pattern, try a few of these suggestions:
    - Add a `reject` method that changes the post's state from `PendingReview` back to `Draft`.
    - Require two calls to `approve` before the state can be changed to `Published`.
    - Allow users to add text content only when a post is in the `Draft` state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the `Post`.
- 2 downsides of the state pattern 
    - Because the states implement the transitions between states, some of the states are coupled to each other.
    - We've duplicated some logic

#### Encoding States and Behavior as Types
- Remove the `content` method from draft post
  - Example as [Listing 17-19: A Post with a content method and a DraftPost without a content method](./listings/_19/src/lib.rs) 

#### Implementing Transitions as Transformations into Different Types
- Example codes 
  - [Listing 17-20: A PendingReviewPost that gets created by calling request_review on DraftPost and an approve method that turns a PendingReviewPost into a published Post](./listings/_20/src/lib.rs)
  - [Listing 17-21: Modifications to main to use the new implementation of the blog post workflow](./listings/_21/src/main.rs)
- Our gain 
  - invalid states are now impossible because of the type system 
  - the type checking that happens at compile time
- Rust is capable of implementing object-oriented design patterns, other patterns, such as encoding
  state into the type system, are also available in Rust.
- Object-oriented patterns won't always be the best solution in Rust due to certain features, like
  ownership, that object-oriented languages don't have