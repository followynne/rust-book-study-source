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

The simplest example goes with the string variable, which is an immutable type; the difficulties come with the String type, allocated on the heap and used to store an amount of text unknown at compile time.

To create a String, there's ::from

`let s = String::from("example");`

String is mutable, due to how it's managing memory.