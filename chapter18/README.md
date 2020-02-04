# Chapter 18. Patterns and Matching

- A pattern consists of some combination of the following:
    * Literals
    * Destructured arrays, enums, structs, or tuples
    * Variables
    * Wildcards
    * Placeholders
- To use a pattern, we compare it to some value. If the pattern matches the
value, we use the value parts in our code

## All the Places Patterns Can Be Used
### `match` Arms
- Formally, `match` expressions are defined as the keyword `match`, a value to
match on, and one or more match arms that consist of a pattern and an
expression to run if the value matches that arm’s pattern, like this:
    ```text
    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }
    ```
- One requirement for `match` expressions is that they need to be *exhaustive* in
the sense that all possibilities for the value in the `match` expression must
be accounted for
- One way to ensure you’ve covered every possibility is to have
a catchall pattern for the last arm
- A particular pattern `_` will match anything, but it never binds to a variable,
so it’s often used in the last match arm

### Conditional `if let` Expressions
- Use `if let` expressions mainly as a shorter way to write the equivalent of a `match` that only matches one case
- Example as [Listing 18-1: Mixing `if let`, `else if`, `else if let`, and `else`](./listings/_01/src/main.rs)
    - `if let` can also introduce shadowed variables in the same way that `match` arms can: the line `if let Ok(age) = age` introduces a new shadowed `age` variable that contains the value inside the `Ok` variant
    - The shadowed `age` we want to compare to 30 isn’t valid until the new scope starts with the curly bracket
- The downside of using `if let` expressions is that the compiler doesn’t check
exhaustiveness, whereas with `match` expressions it does

### `while let` Conditional Loops
- The `while let` conditional loop allows a
`while` loop to run for as long as a pattern continues to match
- Example as [Listing 18-2: Using a `while let` loop to print values for as long as `stack.pop()` returns `Some`](./listings/_02/src/main.rs)

### `for` Loops
- In a `for` loop, the pattern is the value that directly follows the keyword `for`, so in `for x in y` the `x` is the pattern
- Example as [Listing 18-3: Using a pattern in a `for` loop to destructure a tuple](./listings/_03/src/main.rs)

### `let` Statements
- More formally, a `let` statement looks like this:
    ```text
    let PATTERN = EXPRESSION;
    ```
- Example as [_04_05](./listings/_04_05/src/main.rs)
    - Listing 18-4: Using a pattern to destructure a tuple and create three variables at once
    - Listing 18-5: Incorrectly constructing a pattern whose variables don’t match the number of elements in the tuple
- If we wanted to ignore one or more of the values in the tuple, we could use `_` or `..` (detailed later)

### Function Parameters
- Example as [Listing 18-7: A function with parameters that destructure a tuple](./listings/_07/src/main.rs)
- We can also use patterns in closure parameter lists in the same way as in
function parameter lists

## Refutability: Whether a Pattern Might Fail to Match
- Patterns that will match for any possible value passed are *irrefutable*
- Patterns that can fail to match for some possible value are *refutable*
- Function parameters, `let` statements, and `for` loops can only accept
irrefutable patterns, because the program cannot do anything meaningful when
values don’t match
- The `if let` and `while let` expressions accept refutable and irrefutable patterns
- Being familiar with the concept of refutability can help to debug errors 
    - Examples as 
      - [Listing 18-8: Attempting to use a refutable pattern with `let`](./listings/_08/src/main.rs)
      - [Listing 18-9: Using `if let` and a block with refutable patterns instead of `let`](./listings/_09/src/main.rs)
- Match arms must use refutable patterns, except for the last
arm, which should match any remaining values with an irrefutable pattern
    > Rust allows us to use an irrefutable pattern in a `match` with only one arm, but
    > this syntax isn’t particularly useful and could be replaced with a simpler
    > `let` statement

## Pattern Syntax
### Matching Literals
```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

### Matching Named Variables
- Named variables are irrefutable patterns that match any value
- Because `match` starts a new scope, variables declared as part of a pattern inside the `match` expression will shadow those with the same name outside the `match` construct, as is the case with all variables
  - Example as [Listing 18-11: A `match` expression with an arm that introduces a shadowed variable `y`](./listings/_11/src/main.rs)
- To create a `match` expression that compares the values of the outer `x` and
`y`, rather than introducing a shadowed variable, we would need to use a match
guard conditional instead

### Multiple Patterns
- Match multiple patterns using the `|` syntax, which means *or*
    ```rust
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    ```

### Matching Ranges of Values with `...`
- The `...` syntax allows us to match to an inclusive range of values
    ```rust
    let x = 5;

    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("something else"),
    }
    ```
- Ranges are only allowed with numeric values or `char` values, because the
compiler checks that the range isn’t empty at compile time. The only types for
which Rust can tell if a range is empty or not are `char` and numeric values
- Example 
    ```rust
    let x = 'c';

    match x {
        'a' ... 'j' => println!("early ASCII letter"),
        'k' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    ```

### Destructuring to Break Apart Values
#### Destructuring Structs
- Examples 
  - Full as [Listing 18-12: Destructuring a struct’s fields into separate variables](./listings/_12/src/main.rs)
  - Shorthand as [Listing 18-13: Destructuring struct fields using struct field shorthand](./listings/_13/src/main.rs)
- We can also destructure with literal values as part of the struct pattern
rather than creating variables for all the fields. Doing so allows us to test
some of the fields for particular values while creating variables to
destructure the other fields
    - Example as [Listing 18-14: Destructuring and matching literal values in one pattern]

#### Destructuring Enums
- The pattern to destructure an enum should correspond to the way the data stored within the enum is defined
- Example as [Listing 18-15: Destructuring enum variants that hold different kinds of values]

#### Destructuring Nested Structs and Enums
- Example as [Listing 18-16: Matching on nested enums]

#### Destructuring Structs and Tuples
- Example
    ```rust
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
    ```

### Ignoring Values in a Pattern
#### Ignoring an Entire Value with `_`
- Example as [Listing 18-17: Using `_` in a function signature]
- Ignoring a function parameter can be especially useful in some cases, for example, when
implementing a trait when you need a certain type signature but the function
body in your implementation doesn’t need one of the parameters
- The compiler will then not warn about unused function parameters, as it would if you used a name instead

#### Ignoring Parts of a Value with a Nested `_`
- Example as [Listing 18-18: Using an underscore within patterns that match `Some` variants when we don’t need to use the value inside the `Some`]
- We can also use underscores in multiple places within one pattern to ignore particular values
    - Example as [Listing 18-19: Ignoring multiple parts of a tuple]

#### Ignoring an Unused Variable by Starting Its Name with `_`
- If you create a variable but don’t use it anywhere, Rust will usually issue a
warning because that could be a bug
- Sometimes it’s useful to create a variable you won’t use yet, such as when you’re prototyping or just starting a project
- Tell Rust not to warn you about the unused variable by starting the name of the variable with an underscore
    - Example as [Listing 18-20: Starting a variable name with an underscore to avoid getting unused variable warnings]
- The syntax `_x` still binds the value to the variable, whereas `_` doesn’t bind at all
    - Examples 
      - Listing 18-21: An unused variable starting with an underscore still binds the value, which might take ownership of the value
      - Listing 18-22: Using an underscore does not bind the value

#### Ignoring Remaining Parts of a Value with `..`
- We want to operate only on the `x` coordinate and ignore the values in the `y` and `z` fields
  - Example as [Listing 18-23: Ignoring all fields of a `Point` except for `x` by using `..`]
- The syntax `..` will expand to as many values as it needs to be
  - Example as [Listing 18-24: Matching only the first and last values in a tuple and ignoring all other values]
- Using `..` must be unambiguous
    - Failed example as [Listing 18-25: An attempt to use `..` in an ambiguous way]

### Extra Conditionals with Match Guards
- A *match guard* is an additional `if` condition specified after the pattern in
a `match` arm that must also match, along with the pattern matching, for that
arm to be chosen
    - Example as [Listing 18-26: Adding a match guard to a pattern]
- Use match guards to solve our pattern-shadowing problem
    - Example as [Listing 18-27: Using a match guard to test for equality with an outer variable]
- You can also use the *or* operator `|` in a match guard to specify multiple
patterns; the match guard condition will apply to all the patterns
    - Example as [Listing 18-28: Combining multiple patterns with a match guard]

### `@` Bindings
- The *at* operator (`@`) lets us create a variable that holds a value at the
same time we’re testing that value to see whether it matches a pattern
    - Example as [Listing 18-29: Using `@` to bind to a value in a pattern while also testing it]
