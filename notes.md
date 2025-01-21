# Learning Rust

These are my notes as I read/work through [The Rust Programming Language][def1].

> Skipped: Chapters 1 -3, basic programming concepts but they are represented in the existing projects in this directory

[bookmark](https://doc.rust-lang.org/book/ch04-03-slices.html)

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

## References & Borrowing - project `/references/`
A reference is like a pointer that just provides the address to the data. But unlike a pointer, a reference is guarnateed to to point to a value of a specific type.  This allows you to provide a variable to a function and still access it without having to pass it back to the calling function.

Fundamentally, references allow your code to refer to a value without taking ownership of it.  Function signatures indicate they expect a reference by including the `&` character in their type declaration, e.g.
```rust
fn string_length(s: &String) -> usize
```
Variables are prefaced with the `&` to pass the reference to a function, e.g.
```rust
let len = string_length(&s1);
```

When a function accepts a reference to value it is said to have "borrowed" it.  One thing to note about borrowing is that you cannot modify a borrowed value unless you explicitly declare the reference to be mutable (just like variables). In those cases, the `&` is prefixed to the `mut` modifier.

Mutable references carry their own gotchas.  To prevent data races, you cannot declare multiple references to the same variable in the same scope if one of them is mutable.  This prevents situations where two pointers point to the same data, one is used to mutate the data, and there is not way to synch the access.

You can modify the scope of each pointer to enable the use of multiple references in your code.  Just so long as no data is being both mutated and referred to in the same scope from multiple pointers.  Even using the pointers earlier in your code and not referring to them later is sufficient (see `fn mut_ref()` in `/references/src/main.rs`).

### Dangling References
In many languages with pointers, it is possible to create a dangling pointer, a pointer that references a memory location where the data has been freed.  Rust prevents this at compile time and the error message points out you are passing a borrowed value with no value to be borrowed fom.  Helpful!

## The Slice Type
> project `slices`

Slices let you refer to a sequence of elements in a collection rather than the entire collection.  They are a kind of reference and therefore do not have ownership.

#### Problem:
Write a function that takes in a string of words separated by spaces and returns the first word.  If we do not find a space in the string, the whole string should be returned.

Rust has a mechanism to slice strings by referring to the index positions using the following syntax `let slice = &string[0..N];`

The `[..]` is Rust's range syntax.  If you do not provide the starting index number, Rust will assume you mean `0`. If you do not provide the ending index number, Rust will assume you mean the end of the sequence.

Therefore
```rust
let s = String::from("hello world");
// These two are equivalent
let slice = &s[0..5];
let slice = &s[..5];

let len = s.len();
// And so are these
let slice = &s[3..len];
let slice = &s[3..];
```

## Structs - Structuring Relational Data [Ch 5][def5]
> Project: `.structs/`
Structs (or Structures) are a custom data type that lets you package and name multiple related values to form a meaningful group.  For many OO languages, this is similar to the data attributes of an object.

Struct names should define the signifigance of the collection of data. E.g.

```rust
// Defines a user of our Rust application
struct User {
    active: bool,
    email: String,
    username: String,
    last_logged_in: Instant
}
```

Once you have created a Struct, you use it by creating an instance.  Instantiating structs invovles providing concrete values for the attributes in `{key: value}` pairs.

To access specific values from a struct, you use `.` notation like so:
```rust
user1 = User {
    active: true,
    email: String::from("judith.wombat@bufo.io"),
    username: String::from("iamusername"),
    last_logged_in: Instant::now()
}
println!("{user1.email}")
```
You can also update instances the same way (if you have declared the instance mutable with the `mut` keyword).

Similar to Javascript object declaration, Rust has a field init shorthand syntax for concisely declaring struct instances where your variable names match the field names of a struct.  E.g.
```rust
fn build_user(email:String, username:String) -> User {
    return User{
        active: true,
        username,
        email,
        last_logged_in: Instant::now()
    };
}
```
When you need to create a new struct that shares many of the same values, you can use the struct update syntax to copy values from an existing struct.

```rust
let user1 = build_user();
let user2 = User {
    email: String::from("new.example@bufo.io"),
    ..user1 // this has to come last
}
```
> **Note** The above approach will Move data _from_ user1 to user2. This will invalidate user1.  If we provided both email and username then user1 would still be valid since the other types implement the Copy trait.

### [Tuple Structs][def6]
Rust also allow you to define structs that appear similar to Tuples and do not name the individual fields.  These are useful when you want to provide the labeling of naming a struct but you don't care about the names of the particular fields.



```rust
struct Color(i32, i32, i32); // although, personally I would label these as R,G,B respectively
struct Point(i32, i32, i32); // again, I would label X,Y,Z

let black = Color(0,0,0);
let origin = Point(0,0,0);
```
In the snippet above both `black` and `origin` are essentially tuples with the same values.  However, using a tuple struct with a defined name helps us comminicate what each data type represents.

### [Example Program using Structs][def7]
Task: Write a program that calculates the area of a rectangle using first single variables and then refactor to use `struct`s.

> project `ch5_example/`

The code in the above project is hihgly annotated.  However, the overall lesson here is, beyond keeping related data components together, strucs help us improve our code by making the meaning and intended use of our code clear.

Another useful tip is the `dbg1` macro.  This is Rust's native debugging tool.  In addition to printing out details of varaiables and expressions, it writes to `stderr` and includes the filename and line number where it is used.

In summary, structs allow you to create custom types that encapsulate data and functions that have specific meaning to the domain of your program.  This can make your code better organized and easier to understand.

## Enums and Pattern Matching - [Ch6][def8]

> Examples in enums/src/main.rs

Enumerations (or enums) are a useful tool for defining possible variabnts of a type.  They can encode meaning along with specific data, depending on how they are defined.  They provide useful logical control through the application of pattern matching.

### [Defining and Enum][def9]

Enums allow you to define a value as one of a set of possible values. For example, let us say we are working on building a networking application.  For this application we need to distinguish between the two common types of IP addresses.  Enums allow us to group the different types, version 4 and version 6, within a single IP address type.  This allows us to treat both as IP address types in our code but also differentiate between versions where that is applicable.

We could express this in the following definition:
```rust
enum IpAddrVersion {
    V4,
    V6
}
```
We would then define specific instances of each variant like so
```rust
let four = IpAddrVersion::V4;
let six = IpAddrVersion::V6;
```
The `IpAddrVersion` namespace holds both variants and are accessed using the double colon syntax.  This allows our code to be explicit in both the type, `IpAddrVersion`, and the value, `V4` or `V6`, when we declace an instance.

We can then use either variant in a function that uses this type, e.g.
```rust
fn route(ip_version: IpAddrVersion){
    ...
}
```

You can _even_ allow different variants of the same enum type to store different information.  Let's look at another example

```rust
enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor{r: i32, g: i32, b: i32}
}
```
In this case we are defining a Message type, potentially for messages one part of our code could send to another, and each variant of our message enum has different data associated with it.

`Quit` sends no additional data, which makes sense if all we need to do is exit the current process.  `Move` seems to imply changing positions on a 2D surface. `Write` encodes a String type, and `ChangColor` takes in values to define a color using RGB notation.

You _could_ define all the same information using the `struct` keyword but then each type of `Message` would need to be it's own type and you could not specify a `Message` type.  In fact, the standard library uses an enum for IP Addresses that makes use of `struct` to define the data associated with each enum variant:
```rust
struct Ipv4Addr {
    //...
}

struct Ipv6Addr {
    // ...
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
This allows us to build different data requirements for each variant while still grouping the data into one type, `IpAddr`.

One more similarity to `struct` that the `enum` type has is the ability to define methods using the `impl` keyword.  Below is an example using the `Message` enum we defined earlier:
```rust
impl Message {
    fn call(&self){
        // method body defined here
    }
}

let m = Message::Write(String::from("I once knew a wombat. His name was Gus"));
m.call();
```

#### Option Enum and advantage over Null values

The `Option` type encodes a common scenario where a value could be something or nothing. This can be useful in adding flexibility to your code.

For example, if you request the first item in a non-empty list, you would receive a value.  If you request the first item in an empty list, you get nothing.  The compiler can check if you have handled all cases.  Rust does not have a `null` feature to capture the absence of a value.  Null values are common in programming languages but they often cause errors when attempting to access a non-null value from a null variable.

But the concept of a null value is still useful so Rust has an enum that can encode the concept of a value be present or absent, the `Option`.  It is definte in the standard library as follows:
```rust
enum Option<T> {
    None,
    Some(T),
}
```
This is such a common and useful feature that it is included in scope automatically, along with both variants.  You can use both variants without the `Option` namespace.  Here are some examples:
```rust
let some_number = Some(5); // defined as Option<i32>
let some_char = Some('e'); // defined as Option<char>

let absent_number: Option<i32> = None; // defined as Option<i32>
```

The types for our first two variables can be infered from the values assigned.  However, for `absent_number` we must define the type explicitly since we are not provided a value.

So how is this any better than Null?  The Rust compiler will not allow us to use `Option<T>` as if it were a definite value.  The following code won't run
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y
```
Rust does not know how to add `i8` and `Option<i8>` types.  So the Rust compiler is looking for us to already have handled the potential of `y` being `None` before it will allow you to use it.  In general, your code should explicitly handle situations where you have both the `Some<i8>` and the `None` values.  The `Option<T>` type has a large number of methods that allow you to handle this which you can find in the [documentation][def10]

### [Match control flow construct][def11]
The `match` flow construct is used to compare a value against a series of patterns and execute code based on which pattern is matched.  Patterns can be simple or complex and good Rust pattern matching is expressive in the patterns matched.  This pattern is powerful also because the compiler checks that all possible cases are handled.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Match flow controls consist of the `match` keyword and an expression followed by a set of braces.  Inside the braces, we define the different patterns and the expressions that should be executed if the pattern is matched.  This combination of pattern & expression is referred to as an "arm".  The value returned for the expression is what is returned when a pattern is matched.

If the expression is short, like just returning a value, we do not necessarily use braces in the expression definition.  However, if we want to run multiple lines of code, then we will need to employ braces.

Remember that we can store data inside our enum variants.  Arms can bind to this as well.  This is also how we can extract data from enum variants as well.

#### Matching Options with Option<T>
Using `match` with the `Option<T>` enum is a useful approach for handling scenarios where a value may or may not exist.

The utility of this approach is that matches _must_ be exhaustive.  You must handle the condition where an optional value is `None` or your Rust code will not compile.  This is true for all enum variants when using a `match` flow but especially when we are using an `Option<T>`, this ensures we handle a condition where we don't have a value.

In some cases, you are really only concered with a subset of your enum variants for specialized behavior and want to apply a more generic pattern for other cases.  You can use a catchall pattern in that case.

Rust offers multiple options here.  You can use `other` as your catch-all pattern and that will catch any values that don't match earlier patterns while passing the value to the expression you set for that arm.  If you do not plan on using that value, you can specify `_`.  This tells the Rust compiler you will not be using this value and it will not bind the value.  You can also provide an empty Tuple `()` as your expression to do nothing when that pattern is matched.

```rust
let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
// or
match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
// or
match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

```


### [Concise Control with if let][def12]

While the `match` flow control ensures you handle all cases, the `if let` syntax allows you to handle values that match a single pattern and doing something different with the rest.

Let's take an example where you want to only perform an action if a variable matches a defined `config_max`.  First we will use the `match` synatx
```rust
let config_max = Som3(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
```
That gets the job done but it's a lot of code for what is a pretty basic flow pattern.  Let's try this with `if let`
```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The max is configured to be {max}")
}
```
This more concise pattern _does_ come at a cost though.  We lose the exhaustive checking that the `match` pattern provides.  Whether to implement the `match` or `if let` pattern depends on whether the gain in more concise code is worth the trade off for less checking.

We can include an `else` branch in the `if let` pattern.  This is essentially the same as the `_` branch in the `match` pattern.  For simple cases there is not much difference between `match` and `if let else` but this will ultimately be up to you and what representation best expresses the intent of your code.  Remember that all these different approaches are intended to allow you to write code that is clear, easy to read, and expresses what it's intent is.

## Managing Projects w/ Pacakges, Crates, and Modules - [Ch7][def13]

As your programs gow, managing your code and keeping it organized become increasingly important.  Grouping related functionalities and keeping distinct features/services separate will help clarify your code and make it plain how different features work.

As your projects grow you organize Rust code by splitting into multiple modles and then multipel fies.  A single package can contain multiple binary crates.  YOu can extract larger projects into multiple crates that become external dependencies.  

The focus of this chapter is how to implement these techniques to encapsulate code logic and implement high level code reuse.  We will discuss how to implement a public interface that encapsulates the details of the implementation.  This allows you to change how your code implements a given function/service without breaking projects that depend on it, as long as the function call and end result remain the same.

This will also deepen our discussion of scope as what objects are in/out of scope plays a larger role when your code logic is spread across multiple different organizational groups.

Let's define some terms we will use

- **Package:** A Cargo feature that lets you build, test, and share Crates
- **Crate"** A tree of modules that produces a library or executable
- **Module** and **use**: Let you control the organization, scope and privacy of code paths
- **Path:** A way of naming an item


### [Packages and Crates][def14]
A `crate` is the smallest amount of code that the Rust compiler considers at a time.  Even when you pass a single file to the Rust runtime using `rustc`, Rust considers this a crate. Crates can contain modules.

A crate comes in two forms: binary or library crate. **Binary crates** are programs you can compile into an executable that you run (e.g. command line application, web server, etc). Each must have a function called `main` that defines what happens when the executable runs.

**Library crates** don't have a `main` function and they don't compile to an executable. Instead they define functionality intended to be shared in multiple projects (e.g. the `rand` functionality we used earlier).  In Rust, "crate" often refers to a library crate. The _crate root_ is a source file that the Rust compiler starts from and makes up the root module of your crate.

A **package** is a bundle of one or more crates that provides a set of functionality.  A package contains `Cargo.toml` file that defines how to build the crates.  The Cargo command line tool is a package that contains a binary crate and a library crate.

When you use `cargo new project`, Cargo creates a `Cargo.toml` file and a `src/main.rs` file.  This convention means `main.rs` is the crate root of the binary crate for this package. If Cargo finds an `src/lib.rs` that is the root for the library crate of the same name.  A package can contain many binary crates in the `src/bin` directory.

### [Modules - Controlling Scope & Privacy][def15]

Before we dig in, here's a quick breakdown of how modules, paths, and the `use` & `pub` keywords work with the Rust compiler. It also covers how most devs organize their Rust code.

* Start crate root: When compiling, the compiler first looks in the crate root (`src/lib.rs` or `src/main.rs`) for code to compile.
* Declaring modules: In the root file, you can declare new modules. The compiler then looks for module code (e.g. for the `neato` module)
    * Inline, whithin curly brackets folloing the `mod neato` declaration
    * In the file `src/neato.rs`
    * In the file `src/neato/mod.rs`
* Declaring submodules: In any file _other thant he root_, you can declare submodules. E.g. you might declare `mod nifty` in `src/neato.rs`. The complier will look for submodules code within the directory named for the parent module in the following places:
    * Inline following `mod nifty`
    * In the file `src/neato/nifty.rs`
    * In the file `src/neato/nitfy/mod.rs`
* Path to code in modules: Once amodule is part of your crate, you can refer to code in that module from anywhwere in the same crate, following privacy rules. E.g. you could access the `Spiffy` type in the `nifty`    module at `crate::neato::nifty::Spiffy`.
* Private vs Public: Code within a module is private from the parent by default. To make a module public, declare it with `pub mod`.  To make items within the module public, prefix their declaration with the `pub` keyword.
* `use` keyword: Within a scope, the `use` keyword creates a shortcut to items to reduce repetition of long paths. Rather than typing `crate::neato::nifty::Spiffy` every time, you can add `use crate::neato::nifty::Spiffy` and then just refer to `Spiffy` later.

We use modules to group related definitions together and name the relation of these definitions.  For example, the code in our `restaurant` library crate demnstrates the grouping of different funtions within their specific domain inside a resaurant. We can see the _module tree_ of this crate below

```sh
crate
  |_front_of_house
    |__hosting
    |  |_ add_to_waitlist
    |  |_ seat_at_table
    |__serving
       |_ take_order
       |_ sere_order
       |_ take_payment
```
This organizations shows how modules are nested and what modules are siblings of each other.  There is implicit root module name `crate`.


---
[def1]: https://doc.rust-lang.org/book
[def2]: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.htmlcar
[def3]: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope
[def4]: https://github.com/supermanzer/todo-cla
[def5]: https://doc.rust-lang.org/book/ch05-00-structs.html
[def6]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
[def7]: https://doc.rust-lang.org/book/ch05-02-example-structs.html
[def8]: https://doc.rust-lang.org/book/ch06-00-enums.html
[def9]: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
[def10]: https://doc.rust-lang.org/std/option/enum.Option.html
[def11]: https://doc.rust-lang.org/book/ch06-02-match.html
[def12]: https://doc.rust-lang.org/book/ch06-03-if-let.html
[def13]: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[def14]: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
[def15]: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html