# Learning Rust

These are my notes as I read/work through [The Rust Programming Language][def1].

> Skipped: Chapters 1 -3, basic programming concepts but they are represented in the existing projects in this directory

## Project Ideas
These are ideas I have for software projects in Rust:
* Revise my [TODO application][def4] to use Rust and write a Git hook for it to run before any push (or commit).

## Ownership - [Ch 4][def2]
* project - `ownership/`

This is Rust's approach to handling memory management and differs from many programming languages.

Because Rust is a systems programming language, you need to be aware of whether your values are stored on the stack or the heap in your program. Both stack and heap are parts of memory available at runtime. `Stack` stores data in LIFO (last in, first out) and requires that all data stored have a fixed size.  Data with unkown sizes at compile time or sizes that might change are stored on the heap instead.

The heap isn't as organized as the stack. It finds a space in memory and returns a `pointer` to that location.  Heap allocation allows you to store the _pointer_ on the stack, since it's size is known.  Pushing to the stack is faster than heap allocation and accessing data on the stack is faster.

Learning to keep track of where your program allocates memory and minimizing the amount of back and forth between the heap and the stack will make your code more efficient and ensure you don't run out of memory.

### Ownership Rules
* Every value must have an owner
* There can only be one owner at a time
* When an owner goes out of scope, the value is dropped

Variables come into scope when declared and remain valid until they go out of scope (see `fn shadowing` in `variables` for examples).

When working with simple values like an integer, the following data is truly copied.

```rust
let x = 5;
let y = x;
```
_However_, more complex data like Strings contain a pointer (ptr), length (len), and capacity.  Length and capacity indicate the amount of memory and the memory allocated respectively.  The pointer maps to the location on the heap were the value is stored. These are the values on the stack, while the characters that the pointer maps to are stored on the heap.  When you copy variables like this, you copy the pointer, length, and capacity, not the value on the heap!

```rust
let s1 = String::from("hello");
let s2 = s1; // copied stack data (ptr, len, capacity)
```

### Impact of Scope
Rust frees memory when variables go out of scope.  This includes releasing data stored on the heap.  Since both `s1` and `s2` point to the _same_ heap data, it is cleared when either one goes out of scope!  **Also:** When both go out of scope, they could raise an error as you attempt to free the same data twice.  Rust solves this by _moving_ the value instead of copying it.  This means that, when you assign the value from `s1` to `s2`, you can no longer access that value from `s1`.  This prevents rust from making automatic deep copies so that you can be certain that any automatic copying will not hurt runtime performance

### Ownership and Functions
These same rules apply to when you pass values to functions.  Passing a variable to a function will either move or copy the value.

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s's value is moved to the function so it's no longer valid here

    let x = 5;  // Since the i32 type of x implements Copy, it's value is copied
    makes_copy(x); // and we can still reference it later
    println!("{x}"); // like this
}
fn takes_ownserhip(some_string: String){
    println!("{some_string}");
}
fn makes_copy(some_integer: i32) {
    println!("{some_integer"});
}
```
Returning values from functions can also impact when variables move in and out of scope.  See the code snippet [here][def3] for an example.  This means that, in order to pass a variable to a function and use that variable later, we would need to return it as part of that function.  This could get pretty tedious when writing our code.  Thankfully Rust also implements a mechanism for using a value without transferring ownership, called references.

## References

[bookmark](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)


[def1]: https://doc.rust-lang.org/book
[def2]: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.htmlcar
[def3]: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope
[def4]: https://github.com/supermanzer/todo-cla
