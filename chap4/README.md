## Ownership

It's a set of rules that govern how a Rust program manages memory.

A quick recap on stack and heap: both are part of memory available to use at runtime, structured in different ways.

The stack stores values in the order it gets them and remove the values in the opposite order (last in, first out - like a stack of plates). Adding data == pushing onto, removing data == popping off; all data on the stack needs to have a **fixed & known** size.

The heap is less organized, as it returns a certain amount of space to store data. The memory allocator finds this space and returns a **pointer** (address of the location) => allocating on the heap. The pointer is of fixed size, which makes it storeable on the stack, but to get the data the pointer must be traversed to the actual memory location it points to.

Pushing on the stack is faster than allocating because the location is always the same (on top). Accessing the heap is slower 'cause the retrieval pass through the pointer trasversal.

### Basic rules

- each value has an **owner**
- there can only be one owner at a time
- when the owner goes out of scope, the value is dropped

### Scopes

A variable is valid when it is in scope and remains valid until it goes out of scope.

```rust
{
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
}
// this scope is now over, and s is no longer valid
```

The simplest example goes with the string variable, which is an immutable type; the difficulties come with the String type, allocated on the heap and used to store an amount of text unknown at compile time.

### Memory & Allocation

To create a String, there's the method ::from

```rust
let s = String::from("example");
```

String is mutable, due to how it's managing memory. Where strings are immutable and of known-size at compile time, text produced at runtime must be allocated on the heap.

This memory is requested from the allocator at runtime and it must be returned when the activities are done on the runtime string.

The 2nd part is the difficult one: garbage collector languages reclaim memory automatically, non-GC requires the user to do the manual reclaim (which is difficult -> early == invalid var, late = lost memory, 2 times = bug ).

> In Rust the memory is automatically returned once the variable that owns it goes out of scope.

When a variable goes out of scope, a special fN is called at the closing curly bracket to return the memory: `drop`.

#### Move concept

```rust
// What happens with heap variables when assigning one to another?
let s1 = String::from("hello");
let s2 = s1;
```

String is made up of 3 parts, stored on the stack:

- pointer to the memory that holds the content (on the heap)
- length [bytes of memory of the current content]
- capacity [total bytes of memory received from the allocator]

When assigning s1 to s2, the String data parts are copied, not the content! The pointers will direct to the same content, as this one isn't copied on the heap.

What's the problem with the drop fN?

_Double Free error_: When the scope ends, both variables will try to free the same memory. **To prevent it, Rust invalidates s1 and consider it no longer valid**. If we try to use s1 after the assignation to s2, a compile err rises.

This invalidation is called **MOVE** (other languages have shallow copy, here it's a move because the first var is put out of usage).

With this design choice there's also an extra point, deep cloning is never done automatically - therefore automatic copying is inexpensive.

#### Clone concept

To deep copy there's a specific method, **clone()**.

#### Stack-only data and copy

With stack variables, the s2 = s1 assignment doesn't put s1 out of scope, because stack vars are not expensive to make (clone would be the same as a normal copy).

To control this behavior and copy a var instead of doing a move, Rust has the **trait Copy**. It's not possible to associate the Copy trait with the Drop trait.

Passing a variable to a function will move or copy, based on the type, the same as it happens with a assignation.

```rust
let s = String::from("hello");  // s comes into scope

takes_ownership(s);             // s's value moves into the function...
                                // ... and so is no longer valid here

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
```

### References & Borrowing

Having a variables move out of scope everytime it is passed to a function is tedious, because we need to return its value back if we want to continue using it later. A solution is to use references.

**Reference (&)** => a pointer to an address that can be followed to access the data stored at that address

The data is owned by another variable and we're sure that there's a valid value at the end of the reference (unlike what happens with pointers).

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // we create a reference and s1 isn't moving to the function

    // s1 is still valid at this point!

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

**borrowing** => action of creating a reference

In a fN with a reference, the variabile goes out of scope but it's not dropped, because there's no ownership on that variable.

References are immutable by default, because it's not possible to modify something we only have a reference to.

To make a reference mutable, we add **mut** to both the original variable and the function signature.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

**Only one mutable reference to a value can be created**! This prevents multiple mutable references and data races at compile time.

Data races happens when 1. 2 or more pointers access the same data at the same time 2. at least 1 pointer is being used to write data 3. no access synchronization mechanism is available.

There's a similar rule for borrowing immutable and mutable references, otherwise the user of the immutable reference could see the value change while using it:

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem 'cause the value won't be able to change!
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

Recap | The rules for references are:

> At any given time, you can have either one mutable reference or any number of immutable references.
> References must always be valid.

### Slice type

A Slice is a reference to a contiguous sequence of elements in a collection rather than the whole. As a reference, it doesn't have ownership.

```rust
// string composed of words, separated by spaces;
// return the 1st complete word or the entire string, if no space found
fn first_word(s: &String) // we borrow the value
 -> usize {
    let bytes = s.as_bytes(); // string as byte array

    // iter + enumerate => iterator applied + tuple returned with index and reference
    for (i, &item) in bytes.iter().enumerate() { 
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

/* THE PROBLEM */
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

The solution to the problem just ahead are string slices, references to a part of String.

```rust
let s = String::from("hello world");

let hello = &s[0..5]; // from 0 per 5 items
let world = &s[6..11]; // from 6 per 5 items
let slice = &s[..2]; // if starting from 0, first number can be excluded
let slice = &s[6..]; // if taking until the last, end number can be excluded
let slice = &s[..]; // if taking whole, we can simply use ..
```

Slices reference only a part of the collection, specified by using the range operators (start index, end index [1 more than the last desidered position of the slice, as range end is excluded]).

> Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index. ([6..11] => 6 with len 5)

```rust
/* THE SOLUTION: returning a string reference */
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // this word is a immutable ref on s!

    s.clear(); // error! it's requesting a mutable reference when a immutable ref exists!
    // s is already borrowed as immutable string (from the first_word fN, we know thanks to the signature)
    // as a fN can't return a reference, the only chance to do it returning reference to a reference parameter!

    println!("the first word is: {}", word);
    //                                ---- immutable borrow later used here
}
```
