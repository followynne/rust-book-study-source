<!-- variables -->

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