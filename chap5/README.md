## Structs (classes)

> A custom data type that lets you package together and name multiple related values that make up a meaningful group.

```rust
struct User {
    active: bool, // fields
    username: String,
    email: String,
    sign_in_count: u64,
}
// using the struct by creating an instance
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

To mutate fields after creation, the variable must be **mut**. There's no option to only specify some fields as mutable.

We can use field init shorthand to use a parameter with the same name of a field as its value during struct instantiation.

We can use __update syntax__ ({ fields..., ..otherVariable }) to create instances from fields of another, and only modifying some values. The update must come in the last position of the instance creation.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init
        email, // field init
        sign_in_count: 1,
    }
}
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // update syntax, similar to the spread syntax in JS
    };

    // here user1 won't be usable anymore, because we're **moving** data to user2.
    // exception: if we specified values for all fields except active and sign_in_count,
    // user1 would be usable because boolean and u64 implements the Copy Trait (btw, are on the stack)!

    // the update entry is usable only 
    // if we don't _ move_ (== copy their values to the new instance) fields
    // that does not implement Copy
}
```

There are __tuple structs__, which are tuples with the struct name definition but that don't define the fields names.

There are __unit structs__, that don't have fields and behaves similar to the unit tuples! They're useful to implement a trait on a type but where there's no data to store in the type itself.

```rust
struct Color(i32, i32, i32); // tuple structs
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct AlwaysEqual; // unit struct, without values

fn main() {
    let subject = AlwaysEqual;    
}
```

