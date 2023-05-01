<!-- hello_world, hello_cargo, guessing_game -->
## Some basics

Rust extension: `.rs`; indentation is made by 4 spaces

File naming convention: separate words by an underscore

Function conventions: all fn(...) require brackets. The opening one should be placed on the same line of the declaration, separated by space.

```rust
rustc <file> // compiles a rust program, getting in input the source file
rustfmt <file> // format file
```

**main function**: always the first code that runs in a Rust executable

When calling a function, the _! after the name_ means that what's getting called is a **MACRO**. Macros are similar to fn but don't always follow the same rules...

The standard library is std; I/O is managed by std::io. To use fn from libraries, we're calling the usage:
`use std::io;`

## Cargo

Rust’s build system and package manager: `cargo --version`

```rust
cargo new <prj_name> // creates new prj using cargo
// build the prj into target/
cargo build 
    :args
        [--release] // release target
        [-p, --package] // choose package from workspace setup  
cargo run // build and run
cargo check // checks that the code compiles
```

Cargo configuration format is in TOML:

- [package]: package configuration (required are pkg: **name** - **version**, **Rust edition**)
- [dependencies]: list of pkg dependencies (known as _crates_ in Rust)

It expects source code under src, keeping the root free for configs and not-code files.

## Some basics 2

To define variables, the keyword is **let**.  
Variables are immutable by default; to make one mutable, it must be declared with **mut**

```rust
let myVar = 5;
let mut myVar = 5;
let mut myString = String::new();
```

**::** syntax indicates an associated function:

> An **associated function** is a function that’s implemented on a type

`.read_line(&mut guess)` calls a fn, passing a parameter. This parameter is specified as:

- mutable (to let the fn change it)
- reference, using **&**

`&guess != &mut guess`: the first is a var by reference, the 2nd a mutable var by reference.

read_line returns a Result value.  

**Result** is an enumeration (a type with variants) and the purpose is to encode error-handling info. Result's variants are Ok (inside has the value) | Err (info on why it failed)  
methods:

- `.expect(...)` either returns the Ok output or it crashes the app with a msg.

`println!("something {var} {}", 2nd)`: fn to print out a string.  
Curly brackets with variable inside == prints the variable content; 
empty brackets == expects an expression per each empty brackets couple as arguments of the fn.

### comparisons & **match** expression

std::cmp::Ordering => enum with variants **Less | Greater | Equal** related to values comparison  
`.cmp(&some_value)`: compares two values, can be called on anything comparable; takes a ref parameter and returns one of the values of ::Ordering.

a match expression is made up of **arms** (a pattern to match against and the code to run if match)