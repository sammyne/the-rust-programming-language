# Chapter 16. Fearless Concurrency

- Fearless concurrency allows you to write code that is free of subtle bugs and is easy to refactor without introducing new bugs.

## Outline 
* How to create threads to run multiple pieces of code at the same time
* *Message-passing* concurrency, where channels send messages between threads
* *Shared-state* concurrency, where multiple threads have access to some piece
  of data
* The `Sync` and `Send` traits, which extend Rust’s concurrency guarantees to
  user-defined types as well as types provided by the standard library

## Using Threads to Run Code Simultaneously
- In most current operating systems, an executed program’s code is run in a
*process*, and the operating system manages multiple processes at once
- Within your program, you can also have independent parts that run simultaneously. The
features that run these independent parts are called *threads*.
- Multi-thread problems
    * Race conditions, where threads are accessing data or resources in an
    inconsistent order
    * Deadlocks, where two threads are waiting for each other to finish using a
    resource the other thread has, preventing both threads from continuing
    * Bugs that happen only in certain situations and are hard to reproduce and fix
    reliably
- 2 models
  - **1:1**: a language calls the operating system APIs to create threads, one operating system thread per one language thread
  - **M:N**: there are `M` green threads per `N` operating system threads, where `M` and `N` are not necessarily the same number
    > Programming language-provided threads are known as *green* threads, and languages that use these green threads will execute them in the context of a different number of operating system threads
- Runtime codes are included by the language in every binary
- The Rust standard library only provides an implementation of 1:1 threading

### Creating a New Thread with `spawn`
- To create a new thread, we call the `thread::spawn` function and pass it a closure containing the code we want to run in the new thread
- Example as [Listing 16-1: Creating a new thread to print one thing while the main thread prints something else](./listings/_01/src/main.rs)
    >  The calls to `thread::sleep` force a thread to stop its execution for a short duration, allowing a different thread to run

### Waiting for All Threads to Finish Using `join` Handles
- The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned
value that, when we call the `join` method on it, will wait for its thread to finish
- Example as [Listing 16-2: Saving a `JoinHandle` from `thread::spawn` to guarantee the thread is run to completion](./listings/_02/src/main.rs)

### Using `move` Closures with Threads
- Failed example as [Listing 16-3: Attempting to use a vector created by the main thread in another thread](./listings/_03/src/main.rs)
- Example as [Listing 16-5: Using the move keyword to force a closure to take ownership of the values it uses](./listings/_05/src/main.rs)

## Using Message Passing to Transfer Data Between Threads
- A channel in programming has two halves: a transmitter and a receiver
  - One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages
  - A channel is said to be *closed* if either the transmitter or receiver half is dropped
- Create a new channel using the `mpsc::channel` function; `mpsc` stands for
*multiple producer, single consumer*
- The abbreviations `tx` and `rx` are traditionally used in many fields for *transmitter* and *receiver* respectively
- The receiving end of a channel has two useful methods: `recv` and `try_recv`
  - `try_recv` doesn't block, but will instead return a `Result<T, E>` immediately: an `Ok` value holding a message if one is available and an `Err` value if there aren’t any messages this time
  - Using `try_recv` is useful if this thread has other work to do while waiting for messages
- Examples as [_06_08](./listings/_06_08/src/main.rs)
    - Listing 16-6: Creating a channel and assigning the two halves to tx and rx
    - Listing 16-7: Moving tx to a spawned thread and sending "hi"
    - Listing 16-8: Receiving the value "hi" in the main thread and printing it

### Channels and Ownership Transference
- Failed example as [Listing 16-9: Attempting to use val after we've sent it down the channel](./listings/_09/src/main.rs)
- The `send` function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it

### Sending Multiple Values and Seeing the Receiver Waiting
- Example as [Listing 16-10: Sending multiple messages and pausing between each](./listings/_10/src/main.rs)

### Creating Multiple Producers by Cloning the Transmitter
- Example as [Listing 16-11: Sending multiple messages from multiple producers](./listings/_11/src/main.rs)

## Shared-State Concurrency
- Shared memory concurrency is like multiple ownership: multiple threads
can access the same memory location at the same time

### Using Mutexes to Allow Access to Data from One Thread at a Time
- *Mutex* is an abbreviation for *mutual exclusion*, as in, a mutex allows only
one thread to access some data at any given time
- The mutex is described as *guarding* the data it holds via the locking syste
- Mutexes have a reputation for being difficult to use because you have to remember two rules:
    * You must attempt to acquire the lock before using the data.
    * When you’re done with the data that the mutex guards, you must unlock the
    data so other threads can acquire the lock.

#### The API of `Mutex<T>`
- Example as [Listing 16-12: Exploring the API of `Mutex<T>` in a single-threaded context for simplicity](./listings/_12/src/main.rs)
    - The call to `lock` would fail if another thread holding the lock panicked
- The call to `lock` *returns* a smart pointer called `MutexGuard`, wrapped in a
`LockResult` that we handled with the call to `unwrap`
    - The `MutexGuard` smart pointer implements `Deref` to point at our inner data
    - The smart pointer also has a `Drop` implementation that releases the lock automatically when a `MutexGuard` goes out of scope, which happens at the end of the inner scope

#### Sharing a `Mutex<T>` Between Multiple Threads
- Bad example as [Listing 16-13: Ten threads each increment a counter guarded by a `Mutex<T>`](./listings/_13/src/main.rs)

#### Multiple Ownership with Multiple Threads
- Failed example as [Listing 16-14: Attempting to use Rc<T> to allow multiple threads to own the Mutex<T>](./listings/_14/src/main.rs)
- `Rc<T>` is not safe to share across threads

#### Atomic Reference Counting with `Arc<T>`
- `Arc<T>` *is* a type like `Rc<T>` that is safe to use in concurrent situations
    - The *a* stands for *atomic*, meaning it's an *atomically reference counted* type
- Why standard library types aren't implemented to use `Arc<T>` by default. The reason is that thread safety comes with a performance penalty that you only want to pay when you really need to
- Example as [Listing 16-15: Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across multiple threads](./listings/_15/src/main.rs)

### Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`
- `RefCell<T>` allows us to mutate contents inside an `Rc<T>`, and we use `Mutex<T>` to mutate contents inside an `Arc<T>`
- using `Rc<T>` came with the risk of creating reference cycles, where two `Rc<T>` values refer to each other, causing memory leaks. Similarly, `Mutex<T>` comes with the risk of creating *deadlocks*

## Extensible Concurrency with the `Sync` and `Send` Traits
### Allowing Transference of Ownership Between Threads with `Send`
- The `Send` marker trait indicates that ownership of the type implementing
`Send` can be transferred between threads
- Any type composed entirely of `Send` types is automatically marked as `Send` as well

### Allowing Access from Multiple Threads with `Sync`
- The `Sync` marker trait indicates that it is safe for the type implementing
`Sync` to be referenced from multiple threads
- Any type `T` is `Sync` if `&T` (a reference to `T`) is `Send`, meaning the reference can be
sent safely to another thread
- Primitive types are `Sync`, and types composed entirely of types that are `Sync` are also `Sync`

### Implementing `Send` and `Sync` Manually Is Unsafe
- Because types that are made up of `Send` and `Sync` traits are automatically
also `Send` and `Sync`, we don't have to implement those traits manually
