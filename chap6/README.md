## Enums and pattern matching

enums gives a way to restrict a value to a set of values.

```rust
enum IpAddrKind {
    V4,
    V6
}

// instances of the variants
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// data can be put directly into the enum variant,
// without the need to encapsulate it in a struct

// an enum variant can have different types and amounts of data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));

// on enum we can define methods as per struct, with impl
impl IpAddr {
    fn call(&self){
        // etc
    }
}
```

### Option enum

Rust doesn't have null type, while the concept itself (a value currently invalid or absent) is adopted with the Option type.

Option<T> is defined as

```rust
enum Option<T> {
    None,
    Some(T)
}
```

Option<T> requires to manage the chances of None and requires the cast of Some(T) to the actual T type, before doing operations.

Where the type isn't an Option, the value can be safely considered non-null by design.

### match control flow construct

It allows to compare a value against a series of patterns an execute code based on the pattern match.

Match is composed of a condition and arms (pattern + code to execute). Each arm is an expression and corresponds to the return value of the match.

With enum tuple variants, we can use the parameter of the tuple in the expression of the match arm. When match is hit, the binding is completed between arm parameter and the actual argument provided to the enum variant.

```rust
enum UsState { /*...*/ }
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

A match expression must cover all possibilities of the enum entry.

There are 2 ways to treat match with unmatched options:

- the catch-all patterns, whih covers all the unspecified values of the input condition. **The catch-all parameter must come last**. We use catch-all when the value hit should be used.
- the placeholder, called with _. This special syntax tells the compiler that the variable won't be used but that arm will be run when all other arms aren't hit.

### if let: concise control flow

```rust
// this situation tells us that code will be executed only if the value is a Some variant
// the placeholder arm is boilerplate...
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}

// ... but we can have a different way of writing it concisely:
let config_max = Some(3u8);
// pattern is Some(max), max binds to the value inside the Some
// we use max in the expression and associate the whole match short-cut to the input variable
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

if let takes a pattern and an expression, separate by an equal sign.

`if let @@pattern@@ = @inputvalue@@ { code... }`

if let lets define an else expression:

```rust
if let @@pattern@@ = @inputvalue@@ {
    code...
} else {
    code... 
}
```

which covers the catch-all/placeholder option.