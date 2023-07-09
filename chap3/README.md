<!-- variables, branches -->

## Common programming concepts

### Variables + Mutability

By default, variables are immutable (= when a value is bound to a name, the value can't be changed). Trying to update a variable value will result in a compile error.

To make a variable mutable, place the keyword **mut** after the let declaration.

Constants are always immutable. They are declared with **const** and the type value must be annotated.

They can be declared in any scope and they can be set only to constant expression, **not to result of values coming from a runtime computation**.

Constants are declared all uppercase with underscores between words.

A variable with the same name can be declared a 2nd time in the same scope with the same name (**shadowing**).

Shadowing is different from mutating the variable, because we can make transformations to the variable but we leave its value untouched after the transformation scope ends.

Another interesting thing is that the type of the shadowed variable can be changed, because shadowing creates a new variable behind the same name. When you use mut, this type change can't be done. The idea is that, instead of inventing names related to types, we reuse the same variable name for different operations.

It's legit to assign a first x to string, a 2nd x to int - after the scope ends, the x will be string.

```rust
let x = 5;

let mut z = 5;

const MY_VALUE: u32 = 5 * 40;

{
    let x = x * 2; // shadowing...
    println!("The value of x in the inner scope is: {x}"); // value: 10
}

{
    let x = "my_new_string_Value"; // x becomes a string during this scope...
}
```

### Data Types

Rust is a statically typed language == means that it must know the types of all variables at compile time.

There are 2 subsets:

- *Scalar types*, they represent a single value.
    - integers: number without a fractional component. In the type it's specified the size ({x}-bit) and if it is signed/unsigned (_i vs u_), a.k.a. if it can be negative.
    example: i32, i64, u8...
    default is i32
    - floating-points: number with decimal points. There's f32 and f64, with the latter the more precise one.
    - boolean: one byte in size
    - character type: specified with single quotes, 4 bytes in size, represents an Unicode Scalar Value.
- *Compound types*, they group multiple values in one variable.
    - tuple: declared by writing comma-separated values of different types inside parenthesis.
    example: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
    the values can be accessed or by variable deconstruction [`let (x,y) = my_tuple;`] or by using the value index [my_tuple.0]
    a tuple without values is a unit, with value and type written as **()**. fN that doesn't return implicitly return a unit tuple;
    - array: single chunk of memory of a known, fixed size that can be allocated on the stack; they have fixed length and all values must share the same type. Arrays are useful to store data on the stack instead of the heap and when the num of items of the collection is fixed.
    To specify the array type, you set the items type and the number of items: `let a: [i32; 5] = [1, 2, 3, 4, 5];`

> **what happens on an integer overflow?** The variable can't hold the provided value and the program panic.
>
> In --release mode, instead of panicking, the program performs a _two complement wrapping_ (the type wrap around on its maximum values, ex u8 => 255 max, 256 would cause a panic => type wraps and wraps 256 as value 0), which ends with the variable assigned to an unexpected value.
>
> This overflow can be handled in specific ways
>
> - wrap with wrapping_* methods
> - return None if overflow, using checked_* methods
> - return value + boolean that tells about happened overflow, with overflowing_* methods
> - saturate with saturate_* methods

> **what happens on invalid array element access?** The program will panic, to prevent it from accessing invalid memory.

### Functions

_main_: entry point of a rust program

_fn_: keyword to declare functions

A main difference in Rust is statements vs expressions:

- statements: instructions that perform actions and not return a value
- expressions: evaluate to a resultant value

**Expressions don't include ending semicolon**; if there's a semicolon, the expression will become a statement and won't return a value.

The return type of a function is declared after the arguments with an arrow, as  
`fn five() -> i32 { 5}`

### Control flow

**if, if-else**

**loop**:
    executing a block of code until an exit cause is hit
    **break** will stop the loop and exit it, **continue** will stop the current iteration and proceed to the next one immediately
    if there are multiple loops, we can tag them to be able to distinguish them when using the other flow statements. An example:
    `'my_loop': loop { let mut i = 5; loop { if i == 0 { break 'my_loop'; } i -= 1;  } }`

**while**

**for**:
    `let a = [1,2]; for element in a { println!("the value is {element}"); } `
    we can also use a Range argument in place of an array to use the for and iterate a certain amount of times:
    `for element in (1..2) { println!("the value is {element}"); } `