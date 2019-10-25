# Chapter 05. Using Structs to Structure Related Data

## Defining and Instantiating Structs
- To define a struct
  - enter the keyword `struct` and name the entire struct
  - inside curly brackets, define the names and types of the pieces of data, which we call *fields*
  - example
    ```rust
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    ```
- To get a specific value from a struct, we can use dot notation
- The entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable
- Example as [listing 01-04](listings/_01_04/src/main.rs)

### Using the Field Init Shorthand When Variables and Fields Have the Same Name
- Example as [listing 05](listings/_05/src/main.rs)

### Creating Instances from Other Instances with Struct Update Syntax
- Example as [listing 06-07](listings/_06_07/src/main.rs)

### Using Tuple Structs Without Named Fields to Create Different Types
- Tuple structs have the added meaning the struct name provides but don’t have names associated with
  their fields; rather, they just have the types of the fields
- **WHEN TO USE**: You want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each field as in a regular struct would be verbose or redundant
- To define a tuple struct, start with the struct keyword and the struct name followed by the types in the tuple
    - Example as [tuple_struct](listings/tuple_struct/src/main.rs)
- Each struct you define is its own type, even though the fields within the struct have the same types
- Tuple struct instances behave like tuples
  - You can destructure them into their individual pieces
  - You can use a `.` followed by the index to access an individual value

### Unit-Like Structs Without Any Fields
- **WHEN**: You need to implement a trait on some type but don’t have any data that you want to store in the type itself

## An Example Program Using Structs
- Example without struct as [listing 08](listings/_08/src/main.rs)

### Refactoring with Tuples
- Example goes as [listing 09](listings/_09/src/main.rs)
    - Demerit: less clear because tuples don't name their elements

### Refactoring with Structs: Adding More Meaning
- Example goes as [listing 10](listings/_10/src/main.rs)

### Adding Useful Functionality with Derived Traits
- By default, the curly brackets tell `println!` to use formatting known as `Display:` output intended for direct end user consumption
- The primitive types we've seen so far implement `Display` by default
- Putting the specifier `:?` inside the curly brackets tells
`println!` we want to use an output format called `Debug`
- Example as [listing 11-12](listings/_11_12/src/main.rs)

## Method Syntax
- Methods are different from functions 
  - They're defined within the context of a struct 
  - Their first parameter is always `self`, which represents the instance of the struct the method is being called on

### Defining Methods
- Example as [listing 13](listings/_13/src/main.rs)
- Methods can take ownership of `self`, borrow `self` immutably as we’ve done here, or borrow `self` mutably, just as they can any other parameter
- The main benefit of using methods instead of functions 
    - Not having to repeat the type of `self` in every method's signature
    - For organization

### Methods with More Parameters
- Example as [listing 14-15](listings/_14_15/src/main.rs)

### Associated Functions
- **WHAT**: functions within `impl` blocks that *don't* take `self` as a parameter
- **WHEN**: Often used for constructors that will return a new instance of the struct
- Example as [associated_func](listings/associated_func/src/main.rs)

### Multiple `impl` Blocks
- Example as [listing 16](listings/_16/src/main.rs)