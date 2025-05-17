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
This organizations shows how modules are nested and what modules are siblings of each other.  There is implicit root module name `crate`. It is also similar to a filesystem directory.  The primary use cases are similar. We need to organize our code and be able to find what we need.

### [Paths][def16]

Paths are how we tell the Rust compiler where to find something in our module tree.  Paths take two forms
* **Absolute Path:** The full path starting from the create root (or crate name for external crates).
* **Relative Path:** Path from the current module

In both cases we use double colons `::` to separate parts of the path. You can see multiple examples of this in the `restaurant` project.

### [Use Keyword][def17]

It would be quite tedious if we always needed to write out paths (absolute or relative) every time we wanted to interact with an item.  That is why Rust provides the keyword `use`. This allows us to reference the path once and then interact with the shorter name later on.

For example, using our `/restaurant` library code:
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // <- we can now access hosting directly

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist()
}
```
It's important to note that the `use` keyword only creates a symbolic link to the item in the particular scope in which it is specified.

E.g.
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // <- we can now access hosting directly

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist() // <- this won't work because hosting isn't in scope here
        super::hosting::add_to_waitlist() // <- this references parent scope where hosting IS in scope
    }
}
```
**Note** for _functions_, it is idomatic to import them by specifying the parent module (e.g. `use crate::front_of_house::hosting` instead of `crate::front_of_house::hosting::add_to_waitlist`).  This helps identify what functions are not defined locally.  _BUT_ for enums and structs and other kinds of items, you should specify the full path like so
```rust
use std::collections::HashMap

fn main() {
    let mut map = HashMap::new();
    map.insert(1,2);
}
```
The exception to this general rule is when you import two items with the same name. In that case, keeping the parent module helps diambiguate the two (also Rust won't allow two items with the same name).

_However_, you can rename an item brought into scope with the `use` keyword by specifying the new name with the `as` keyword. This is also similar to Python.

When you bring external modules into scope with the `use` keyword, they are private by default.  You can re-export them with `pub use`

### Separating Modules
All our examples so far define modules in a single file. But this can easily get out of hand as your modules get larger. Splitting modules into separate files can help keep your code organized and readable.

Our `restaurant` project has now been split up into separate modules to allow us to keep our files organized and sepearated our concerns.

## Collections
> Project `collections/`

Collections are common data structures that can store multiple values.  These are separate from built-in array and tuple types because these collections store data on the heap so the amount does not need to be known and can grow/shrink while the program runs.  Collections have different costs & capabilities and choosing the right one is a skill you will develop as your experience grows.

### Vectors
Vectors allow you to store multiple values in a single data structure and keep the values next to each other in memory space.  Vectors are typed, meaning they can only store values of the saem type.  They are useful for storing lists of items.

To modify a vector, like add elements to it, you need to define it as mutable using the `mut` keyword.  Then you can use the `.push()` function.

We can even store multiple types in a vector. This may sound like it breaks the rules but not if we declare an Enum that stores multiple types and a vector that stores that Enum as it's type. 🤪

### [Strings][def20]

Strings, while extremely common in all programming languages, often trip up new Rust programmers.  These are due to Rust's approach for exposing possible errors, the underlying complexity of Strings, and UTF-8.  Strings are included in Collections as they represent a collection of bytes.  Many operations that are common to Collections are also found in Strings.

**What is a String?** - Rust has a string type which is a string slice, `str`, and is often seen in it's borrowed form `&str`.  By contrast, the `String` type is a mutable, owned, UTF-8 encoded type.  Ensuring you know when you are referring to `String` vs `&str` is important to understanding your code.

**Concatenation** - Often you want to build strings from others.  You can do this using the `+` operator.  However, this will consume the first string but require a reference to the second string because that is the way the `add(self, s: &str) -> String` function is built.  You can see this below

```rust
let s1 = String::from("Hello ");
let s2 = String::from("world");
let s3 = s1 + &s2; // s1 is consumed since we passed it directly.  s2 is still available since we passed a reference. 
```
For concatenating multiple strings, using the `+` gets rather unweildy.  What we can do instead is use the `format!` macro.

**Indexing** - Unlike many other languages, you cannot simply access a character in a string using an index like `s[0]`.  Rust does not support String indexing.  Why not?  Because Rust stores Strings as vectors of bytes.  What's more, some characters require more bytes than others to store. Since the index will return a single byte, it may not return a valid Unicode scalar value. Rust avoids returning unexpected values by not supporting this approach.  This is also done to allow programs to choose how they interpret data, no matter the human language.

Because of this approach, it is not advisable to attempt to slice strings, since it's not clear what type would be returned.  You _can_ slice strings like so:
```rust
let hello = "Здравствуйте"; // each character here takes 2 bytes to store
let s = &hello[0..4]; // so s only holds the first 2 letters
```
Because of the number of bytes required to store these characters, if you were to use `s = &hello[0..3];`, the program would crash.  Be careful how you slice!

**Iterating Methods** - THe best way to operate on pieces of a string is to be explicit about whether you want characters or bytes.  You can use the `.chars()` method to explicitly tell Rust you want to extract Unicode scalar values from a string. Alternatively, you can retrieve individual bytes using the `bytes()` method.

**Beware!** Unicode values may be composed of more than 1 scalar value.

Strings a complicated!  If you want to manipulate strings in your programs, be intentional and think carefully. Rust exposes more complexity but it prevents you from having to handle non-ASCII character errors.  There are lots of tools built into Rust that will allow you to safely manipulate strings so be sure to check them out.

### [Hash Maps][def21]

The `HashMap<K, V>` stores mappings of keys to values using a hashing function, which determines how it places these in memory.  Many languages use this kind of data structure but with different names, e.g. object, hash table, dictionary, associative array, etc.

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert(String::from("a"), 10);
map.insert(String::from("b"), 20);
```
HashMaps, like vectors, store their data on the heap and, like vectors, their key and value types are homogenous.

We can access values in a HashMap using the `.get()` method.  The method returnes an `Option<&V>` type if there is a value for the key provided, or `None`.  You can hanlde the `Option` by calling the `copied` function to convert to `<Option<V>` and the `unwrap_or()` function will handle the `None` condition by returning whatever default you specify.
```rust
let val = map.get(&key).copied().unrwap_or(0); 
```
**Ownership** - For types that implement the `Copy` trait, values are copied into the HashMap.  For owned values, like `String`, the values are moved into the HashMap.

**Updating** - The number of keys, vals are growable but each key an only have one value.  If you want to change a value you need to choose how to handle the old value. You could replace, combine, or only add a new value if there isn't an old one.

the `insert` function will overwrite an existing value.  The `insert_or` function will only insert the provided value if there isn't a value currently present. To update based on the old value, it takes a bit more work.  All three are shown below.
```rust
let mut scores = HashMap::new();
scores.insert(String::from("a"),10);
scores.insert(String::from("a"), 20); // overwriting value

scores.entry(String::from("a")).or_insert(30); // nothing happens becaue "a" exists already
scores.entry(String::from("b")).or_insert(40); // adds k, v because they don't exist

let keys = "a b c d";
for key in keys.split_whitespace() {
    let val = map.entry(key).or_insert(10); // adding k,v if they don't exist
    *val += 10; // incrementing values
}
```
Collections provide a large amount of functionality that will be necessary in many programs.  The standard library API docs contain details of many of the methods these collections implement.  

## [Error Handling][def22]
Despite our best efforts, errors happen so it's important to know how to handle them.

Rust groups errors in two major categories: recoverable and uncrecoverable errors.  Recoverable errors, like a `file not found` error, are ones where we likely want to report the error and move on.  Unrecoverable errors are the result of bugs, for example attempting to access an array outside the index.  In these situations we immediately stop the program.

Most languages don't distinguish between these two and treat them the same using a system like Exceptions.  Instead Rust has a type `<Result T, E>` for recoverable errors and the `panic!` macro that stops execution for uncrecoverable errors.

### Unrecoverable errors

For these errors, Rust has the `panic!` macro.  You can trigger this in two ways.
1. By doing someting that will cause the program to panic (e.g. accessing arrays out of bounds)
1. Using the `panic!` macro

By setting an `env` variable, you can have Rust display the call stack when it encounters a `panic!` to make it easier to debug.  By default, when Rust hits a panic it will start unwinding the stack.  This invovles going back up the call stack and cleaning up the data.  This is a lot of work and Rust allows you to change this behavior by specifying in your `Cargo.toml` file.
```toml
[profile.release]
panic = 'about'
```
When Rust panics, it can return just the line in your code that threw the error but often it can be more helpful to see the full stack trace.  In thos situations, you can set `RUST_BACKTRACE` to `1`.  When reading the stack trace, start at the top and read until you get to files you have written.  There will be calls both before and after this that you do not control but your code is where the error came from so reviewing how your program triggered it starts there.

### Recoverable Errors
Many errors are not serious enough that we should stop the program.  Rust includes these in the `Result` enum which can have 2 variants: `Ok` and `Err`.  
```rust
enum Result<T, E>{
    Ok(T), // If the result is okay, return the expected type
    Err(E) // If this caused an error, return the error
}
```
When perfoming an operation that could return either type, you will want to use a `match` statement to handle both cases. E.g.
```rust
let file_result = File::open("path/to/file");

let get_file = match file_result {
    Ok(file) => file,
    Err(error) => panic!("Problem opening file: {error:?}")
}
```
#### Matching different errors
Rust allows you to match on specific types of errors so you can handle those appropriately in your code. This can help you identify and handle those errors that are not severe enough to require exiting your code as well as more serious errors.  Let's take a look at one such example.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt"); // Returns a Result<T, E> enum

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {  // Here we handle different kinds of errors
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Progme creating file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}")
            }
        }
    }
}
```
Both `File::open()` and `File::create()` return `Result<T, E>`. For `File::open()`, we return an `Err` variant of `io::Error`.  This is a struct that contains methods like `kind()` which we can use to check against the enum `io::ErrorKind` to match different types of errors that can result from `io` operations. In the above code, we match of the `NotFound` error type which indicates the file does not exist. In that case we attempt to create the file. We include a nested match statement because file creation could also fail so we raise an error in that case using the `panic!` macro.

The other arm of the outer match identifies if we have some other problem opening the file (e.g. a permissions issue).

In the above code we are doing quite a bit of `match`ing.  The `match` statement is a primitive and you _can_ use closures as a way to be more concise when you need to handle multiple `Result<T,E>` values.

Here's the same operation using closures
```rust
fn main() {
    let greeting_file = File::open("hello.txt").unrwap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {e:?}");
            })
        } else {
            panic!("Problem opening file: {error:?}")
        }
    });
}
```
This does add more operations per line (something I generally try to avoid) but, on the flip side, it _does_ wrap up a lof of our error handling and simplify returning a value to `greeting_file`.

Which style you choose can come down to which is most readable and communicates intent.  The `Result<T,E>` type has many helper methods to reduce the need to use `match`.  For example, `unrwap` is a shortcut that will return a value if `Result` is an `Ok` variant, or call `panic!` if not.

You can even define a custom `panic!` message by using the `expect` function like so:
```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("File hello.txt should be included in this project");
}
```
This approach can be very helpful when your project is going to be used by someone else. Throwing errors that indicate what the user needs to fix make everyone's life easier.  This also makes the assumptions of your code more clear.

### [Propagating Errors][def23]
In some cases where your code calls some function that might fail, you do not want to handle it where the error occurs.  Maybe that isn't the right place to make a decision about what to do next.  In that case you can _propagate_ the error up to the stack to another layer in your code where you might want to decide what to do next.

The `errors` project contains a few different ways to implement this kind of error propagation.  Essentially, what we are trying to do in our basic functions is perform some operation. If we are successful, we return the value. If we are not, we return the errors so the calling function can take apporpriate action.  Our functions do not have sufficient context to know how to handle these errors and we should defer to the calling function.

One nifty thing the `?` operator does when returning errors is coerce the error type into whatever type we defined in the `Result<>` return object.  If we implement a custom error, we could customize the `from` function to allow us to add any program-specific context when returning the error.  This functionality is lost with the most concise version of the function.

The `?` operator can only be used within a function whose return types are compatible. For instance if we attempted
```rust
fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```
this would nto work because the return value for the `main()` function doesn't match what the `?` operator will return.  If you have need of error handling in a function whose return type does not support the `?` error, you can either change the return type or use a `match` statement to handle the error within your function.

### [To Panic or Not][def24]

When you call `panic!` there is no way for your code to recover from the error.  When you choose to either call `panic!` or return `Result`, you are making a decision for your code about whether the error is something that should stop code execution or not.


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
[def16]: https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[def17]: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
[def18]: https://doc.rust-lang.org/book/ch12-00-an-io-project.html
[def19]: https://github.com/supermanzer/photo-info
[def20]: https://doc.rust-lang.org/book/ch08-02-strings.html
[def21]: https://doc.rust-lang.org/book/ch08-03-hash-maps.html
[def22]: https://doc.rust-lang.org/book/ch09-00-error-handling.html
[def23]: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors
[def24]: https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html