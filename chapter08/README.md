# Chapter 08. Common Collections

3 collections:
- A **vector** allows you to store a variable number of values next to each other.
- A **string** is a collection of characters
- A **hash map** allows you to associate a value with a particular key. It's a particular implementation
  of the more general data structure called a map

## Storing Lists of Values with Vectors
- Vectors can only store values of the same type

### Creating a New Vector
- Listing 8-1: Creating a new, empty vector to hold values of type `i32`
    ```rust
    //  we added a type annotation here
    let v: Vec<i32> = Vec::new();
    ```
- In more realistic code, Rust can often infer the type of value you want to store once you insert
  values, so you rarely need to do this type annotation
- It's more common to create a `Vec<T>` that has initial values, and Rust provides the `vec!` macro
  for convenience

    ```rust
    let v = vec![1, 2, 3];
    ```

### Updating a Vector
- Listing 8-3: Using the `push` method to add values to a vector
    ```rust
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    ```
    
### Dropping a Vector Drops Its Elements
- Listing 8-4: Showing where the vector and its elements are dropped
    ```rust
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v

    } // <- v goes out of scope and is freed here
    ```

### Reading Elements of Vectors
- Example as [listing 05](./listings/_05/src/main.rs): either with indexing syntax or the `get` method
    - by using the `get` method with the index passed as an argument, which gives us an `Option<&T>`
- Example as [listing 06](./listings/_06/src/main.rs): attempting to access the element at index 100 in a vector containing five elements
- Example as [listing 07](./listings/_07/src/main.rs): Attempting to add an element to a vector
  while holding a reference to an item
    - this violates the rule that you can't have mutable and immutable references in the same scope
    - Why should a reference to the first element care about what changes at the end of the vector? 
        - This error is due to the way vectors work: adding a new element onto the end of the vector
            might require allocating new memory and copying the old elements to the new space, if
            there isn't enough room to put all the elements next to each other where the vector 
            currently is

### Iterating over the Values in a Vector
- Examples
  - [listing 08](./listings/_08/src/main.rs): Printing each element in a vector by iterating over
    the elements using a `for` loop
  - [listing 09](./listings/_09/src/main.rs): Iterating over mutable references to elements in a vector

### Using an Enum to Store Multiple Types
- [listing 10: Defining an `enum` to store values of different types in one vector](listings/_10/src/main.rs)

## Storing UTF-8 Encoded Text with Strings
### What Is a String?
- The `String` type, which is provided by Rust's standard library rather than coded into the core
  language, is a growable, mutable, owned, UTF-8 encoded string type
- See how those names all end in `String` or `Str`? They refer to owned and borrowed variants, just
  like the `String` and `str` types you've seen previously
  
### Creating a New String
- [Examples](./listings/_12_13/src/main.rs)
    - Listing 8-12: Using the `to_string` method to create a `String` from a string literal
    - Listing 8-13: Using the `String::from` function to create a `String` from a string literal
- [Listing 8-14: Storing greetings in different languages in strings](listings/_14/src/main.rs)

### Updating a String
- by `push_str` or `push` method
- conveniently use the `+` operator or the `format!` macro to concatenate `String` values
#### Appending to a String with `push_str` and `push`
- `push_str` takes a string as parameter as 
  - [Listing 8-15: Appending a string slice to a `String` using the `push_str` method](./listings/_15/src/main.rs) 
  - [Listing 8-16: Using a string slice after appending its contents to a `String`](listings/_16/src/main.rs)
- `push` takes a char as parameter as [Listing 8-17: Adding one character to a `String` value using `push`](./listings/_17/src/main.rs) 

#### Concatenation with the `+` Operator or the `format!` Macro
- [Listing 18: Using the `+` operator to combine two `String` values into a new `String` value](listings/_18/src/main.rs)
    - The reason we're able to use `&s2` in the call to `add` is that the compiler can *coerce* the
        `&String` argument into a `&str`. When we call the `add` method, Rust uses a *deref coercion*, 
        which here turns `&s2` into `&s2[..]`
    - The `format!` macro works in the same way as `println!`, but instead of printing the output to
        the screen, it returns a `String` with the contents. The version of the code using `format!`
        is much easier to read and doesn't take ownership of any of its parameters

### Indexing into Strings
- [Listing 19: Attempting to use indexing syntax with a String](./listings/_19/src/main.rs)

#### Internal Representation
- A `String` is a wrapper over a `Vec<u8>`
- String `String::from("Здравствуйте")` is of length 24: that's the number of bytes it takes to
  encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of 
  storage
- An index into the string's bytes will not always correlate to a valid Unicode scalar value

#### Bytes and Scalar Values and Grapheme Clusters! Oh My!
- A final reason Rust doesn't allow us to index into a `String` to get a character is that indexing
  operations are expected to always take constant time `O(1)`

### Slicing Strings
- Indexing into a string is often a bad idea because it's not clear what the return type of the
  string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string
  slice
- use `[]` with a range to create a string slice containing particular bytes:

    ```rust
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    ```
    - **If either of indices is invalid char bound, the program will crash**

### Methods for Iterating Over Strings
- Calling `chars` on “नमस्ते” separates out and returns six values of type `char`, and you can
  iterate over the result to access each element
    ```rust
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // output:
    // न
    // म
    // स
    // ्
    // त
    // े
    ```
- The `bytes` method returns each raw byte, which might be appropriate for your domain:

    ```rust
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // output:
    // 224
    // 164
    // --snip--
    // 165
    // 135
    ```
- Valid Unicode scalar values may be made up of more than 1 byte

### Strings Are Not So Simple
- Programmers have to put more thought into handling UTF-8 data upfront

## Storing Keys with Associated Values in Hash Maps
- The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`
  
### Creating a New Hash Map
- [Listing 20: Creating a new hash map and inserting some keys and values](./listings/_20/src/main.rs)
- HashMap one is the least often used, so it's not included in the features brought into scope
  automatically in the prelude
- Hash maps are homogeneous: all of the keys must have the same type, and all of the values must
  have the same type
- [Listing 21: Creating a hash map from a list of teams and a list of scores](listings/_21/src/main.rs)
  - Constructing a hash map is by using 
    - the `collect` method on a vector of tuples, where each tuple consists of a key and its value
    - the `zip` method to create a vector of tuples where “Blue” is paired with 10, and so forth

### Hash Maps and Ownership
- [Listing 22: Showing that keys and values are owned by the hash map once they're inserted](./listings/_22/src/main.rs)
    - For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. 
    - For owned values like `String`, the values will be moved and the hash map will be the owner of
        those values
- If we insert references to values into the hash map, the values won't be moved into the hash map.
  The values that the references point to must be valid for at least as long as the hash map is valid

### Accessing Values in a Hash Map
- [Listing 23: Accessing the score for the Blue team stored in the hash map](./listings/_23/src/main.rs)
  - The result is wrapped in `Some` because `get` returns an `Option<&V>`; if there's no value for that key in the hash map, `get` will return `None`
- We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a `for` loop:

    ```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    ```

### Updating a Hash Map
- When you want to change the data in a hash map, you have to decide how to handle the case when a
  key already has a value assigned

#### Overwriting a Value
- [Listing 24: Replacing a value stored with a particular key](./listings/_24/src/main.rs)

#### Only Inserting a Value If the Key Has No Value
- `entry` method on the `HashMap` that takes the key you want to check as a parameter. The return
  value of the `entry` method is an enum called `Entry` that represents a value that might or might
  not exist
    - Example as [Listing 25: Using the `entry` method to only insert if the key does not already have a value](./listings/_25/src/main.rs)
        - The `or_insert` method on `Entry` is defined to return a mutable reference to the value
            for the corresponding `Entry` key if that key exists, and if not, inserts the parameter
            as the new value for this key and returns a mutable reference to the new value.

#### Updating a Value Based on the Old Value
- [Listing 26: Counting occurrences of words using a hash map that stores words and counts](./listings/_26/src/main.rs)
    - The `or_insert` method actually returns a mutable reference (`&mut V`) to the value for this key

### Hashing Functions
- By default, `HashMap` uses a “cryptographically strong”[^siphash] hashing function that can
  provide resistance to Denial of Service (DoS) attacks
