# Chapter 15. Smart Pointers

- References are pointers that only borrow data; in contrast, in many cases, smart pointers *own* the data they point to
- The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the `Deref` and `Drop` traits
  - The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers
  - The `Drop` trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope
- The most common smart pointers in the standard library:
    * `Box<T>` for allocating values on the heap
    * `Rc<T>`, a reference counting type that enables multiple ownership
    * `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces
    the borrowing rules at runtime instead of compile time
- The *interior mutability* pattern where an immutable type exposes an API for mutating an interior value
- *Reference cycles*: how they can leak memory and how to prevent them

## Using `Box<T>` to Point to Data on the Heap
- **WHEN**
    * When you have a type whose size can’t be known at compile time and you want
    to use a value of that type in a context that requires an exact size
    * When you have a large amount of data and you want to transfer ownership but
    ensure the data won’t be copied when you do so
    * When you want to own a value and you care only that it’s a type that
    implements a particular trait rather than being of a specific type
- Example as [Listing 15-1: Storing an i32 value on the heap using a box](./listings/_01/src/main.rs)
### Using a `Box<T>` to Store Data on the Heap
### Enabling Recursive Types with Boxes
- One type whose size can’t be known at compile time is a *recursive type*, where a value can have as part of itself another value of the same type
- Example: cons list
  - Failed as [Listing 02-03](./listings/_02_03/src/main.rs)
    - Listing 15-2: The first attempt at defining an enum to represent a cons list data structure of i32 values
    - Listing 15-3: Using the List enum to store the list 1, 2, 3
  - Ok: [Listing 15-5: Definition of List that uses Box<T> in order to have a known size](./listings/_05/src/main.rs)
#### More Information About the Cons List
- Each item in a cons list contains two elements: the value of the current item and the next item. The last item in the list contains only a value called `Nil` without a next item

#### Computing the Size of a Non-Recursive Type
- To determine how much space to allocate for a `Message` value, Rust goes through each of the variants to see which variant needs the most space

  ```rust
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }
  ```

#### Using `Box<T>` to Get a Recursive Type with a Known Size
- The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references
- When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up because of the `Drop` trait implementation

## Treating Smart Pointers Like Regular References with the `Deref` Trait
- Implementing the `Deref` trait allows you to customize the behavior of the *dereference operator*, `*`

### Following the Pointer to the Value with the Dereference Operator
- Example as [Listing 15-6: Using the dereference operator to follow a reference to an i32 value](./listings/_06/src/main.rs)

### Using `Box<T>` Like a Reference
- Example as [Listing 15-7: Using the dereference operator on a `Box<i32>`](./listings/_07/src/main.rs)

### Defining Our Own Smart Pointer
- Examples as [customizing smart pointers without implementing `Deref` trait will error out](./listings/_08_09/src/main.rs)
  - Listing 15-8: Defining a `MyBox<T>` type
  - Listing 15-9: Attempting to use `MyBox<T>` in the same way we used references and `Box<T>`

### Treating a Type Like a Reference by Implementing the `Deref` Trait
- The `Deref` trait, provided by the standard library, requires us to implement one method named `deref` that borrows `self` and returns a reference to the inner data
    - Example as [Listing 15-10: Implementing Deref on `MyBox<T>`](listings/_10/src/main.rs)
- When we entered `*y` in Listing 15-9, behind the scenes Rust actually ran this code:
    ```rust,ignore
    *(y.deref())
    ```

### Implicit Deref Coercions with Functions and Methods
- Deref coercion converts a reference to a type that implements `Deref` into a reference to a type that `Deref` can convert the original type into
- Deref coercion happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition
- A sequence of calls to the `deref` method converts the type we provided into the type the parameter needs
- Example as [Listing 15-12: Calling hello with a reference to a MyBox<String> value, which works because of deref coercion](listings/_11_12/src/main.rs)
- Example if without deref coercion as [Listing 15-13: The code we would have to write if Rust didn’t have deref coercion](./listings/_13/src/main.rs)

### How Deref Coercion Interacts with Mutability
- Rust does deref coercion when it finds types and trait implementations in three cases:
    * From `&T` to `&U` when `T: Deref<Target=U>`
    * From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
    * From `&mut T` to `&U` when `T: Deref<Target=U>`
- Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile)

## Running Code on Cleanup with the `Drop` Trait
- The `Drop` trait lets you customize what happens when a value is about to go out of scope
- Specify the code to run when a value goes out of scope by implementing the `Drop` trait. The `Drop` trait requires you to implement one method named `drop` that takes a mutable reference to `self`
- The `Drop` trait is included in the prelude, so we don’t need to bring it into scope
- Example as [Listing 15-14: A CustomSmartPointer struct that implements the Drop trait where we would put our cleanup code](listings/_14/src/main.rs)

### Dropping a Value Early with `std::mem::drop`
- Rust doesn't let you call the `Drop` trait’s `drop` method manually
- You have to call the `std::mem::drop` function provided by the standard library if you want to force a value to be dropped before the end of its scope
    - Example as [Listing 15-16: Calling std::mem::drop to explicitly drop a value before it goes out of scope](./listings/_16/src/main.rs)
- Rust doesn't let us call `drop` explicitly because Rust would still automatically call `drop` on the value at the end of `main`. This would be a *double free* error because Rust would be trying to clean up the same value twice
    - Example as [Listing 15-15: Attempting to call the drop method from the Drop trait manually to clean up early](listings/_15/src/main.rs)
- `std::mem::drop` is in the prelude

## `Rc<T>`, the Reference Counted Smart Pointer
- The `Rc<T>` type keeps track of the number of references to a value which determines whether or not a value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid
- **WHEN**: we want to allocate some data on the heap for multiple parts of our program to read and we can't determine at compile time which part will finish using the data last
- `Rc<T>` is **ONLY** for use in single-threaded scenarios

### Using `Rc<T>` to Share Data
- Every time we call `Rc::clone`, the reference count to the data within the `Rc<List>` will increase, and the data won't be cleaned up unless there are zero references to it
- We could have called `a.clone()` rather than `Rc::clone(&a)`, but Rust's convention is to use `Rc::clone` in this case. The implementation of `Rc::clone` doesn't make a deep copy of all the data like most types' implementations of `clone` do. The call to `Rc::clone` only increments the reference count, which doesn't take much time
- Examples
  - [Listing 15-17: Demonstrating we're not allowed to have two lists using `Box<T>` that try to share ownership of a third list](./listings/_17/src/main.rs)
  - [Listing 15-18: A definition of List that uses `Rc<T>`](./listings/_18/src/main.rs)

### Cloning an `Rc<T>` Increases the Reference Count
- The `Rc::strong_count` function returns the reference count of `Rc<T>`
- Via immutable references, `Rc<T>` allows you to share data between multiple parts of your program for reading only
- Example as [Listing 15-19: Printing the reference count](./listings/_19/src/main.rs)

## `RefCell<T>` and the Interior Mutability Pattern
- *Interior mutability* is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
- To mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing
- We can use types that use the interior mutability pattern when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can't guarantee that

### Enforcing Borrowing Rules at Runtime with `RefCell<T>`
- With references and `Box<T>`, the borrowing rules' invariants are enforced at compile time. With `RefCell<T>`, these invariants are enforced *at runtime*
- With references, if you break these rules, you'll get a compiler error. With `RefCell<T>`, if you break these rules, your program will panic and exit
- The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, whereas they are disallowed by the compile-time checks
- `RefCell<T>` is **ONLY** for use in single-threaded scenarios
- A recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:
    * `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>`
    have single owners.
    * `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>`
    allows only immutable borrows checked at compile time; `RefCell<T>` allows
    immutable or mutable borrows checked at runtime.
    * Because `RefCell<T>` allows mutable borrows checked at runtime, you can
    mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is
    immutable.

### Interior Mutability: A Mutable Borrow to an Immutable Value
#### A Use Case for Interior Mutability: Mock Objects
- A *test double* is the general programming concept for a type used in place of
another type during testing. *Mock objects* are specific types of test doubles
that record what happens during a test so you can assert that the correct
actions took place
- Examples as [listing](./listings/_20_21_22/src/main.rs)
  - Listing 15-20: A library to keep track of how close a value is to a maximum value and warn when the value is at certain levels
  - Listing 15-21: An attempt to implement a MockMessenger that isn't allowed by the borrow checker
  - Ok as Listing 15-22: Using `RefCell<T>` to mutate an inner value while the outer value is considered immutable

#### Keeping Track of Borrows at Runtime with `RefCell<T>`
- `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time
- Example as [Listing 15-23: Creating two mutable references in the same scope to see that RefCell<T> will panic](./listings/_23/src/lib.rs)

### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`
- If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple owners *and* that you can mutate
- Because `Rc<T>` holds only immutable values, we can't change any of the values in the list once we've created them
- Example as [Listing 15-24: Using `Rc<RefCell<i32>>` to create a List that we can mutate](./listings/_24/src/main.rs)

## Reference Cycles Can Leak Memory
- Memory leaks are memory safe in Rust

### Creating a Reference Cycle
- Creating a reference cycle would be a logic bug in your program that you should use automated tests, code reviews, and other software development practices to minimize
- Another solution for avoiding reference cycles is reorganizing your data
structures so that some references express ownership and some references don’t
- Examples 
  - [Listing 15-25: A cons list definition that holds a `RefCell<T>` so we can modify what a Cons variant is referring to](./listings/_25/src/lib.rs)
  - [Listing 15-26: Creating a reference cycle of two List values pointing to each other](./listings/_26/src/main.rs)

### Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`
- Create a *weak reference* to the value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a reference to the `Rc<T>`. When you call `Rc::downgrade`, you get a smart pointer of type `Weak<T>`
- The difference is the `weak_count` doesn't need to be 0 for the `Rc<T>` instance to be cleaned up
- They won't cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0
- You must make sure the value still exists. Do this by calling the `upgrade` method on a `Weak<T>` instance, which will return an `Option<Rc<T>>`. You’ll get a result of `Some` if the `Rc<T>` value has not been dropped yet and a result of `None` if the `Rc<T>` value has been dropped

#### Creating a Tree Data Structure: a `Node` with Child Nodes
- Example as [Listing 15-27: Creating a leaf node with no children and a branch node with leaf as one of its children](./listings/_27/src/main.rs)

#### Adding a Reference from a Child to Its Parent
- Example as [Listing 15-28: A `leaf` node with a weak reference to its parent node `branch`](./listings/_28/src/main.rs)

#### Visualizing Changes to `strong_count` and `weak_count`
- Example as [Listing 15-29: Creating branch in an inner scope and examining strong and weak reference counts](./listings/_29/src/main.rs)