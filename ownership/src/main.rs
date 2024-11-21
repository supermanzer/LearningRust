// Functions for demonstrating ownership in Rust

fn string_class() {
    let mut s = String::from("hello"); // creating a String type from a string literal

    s.push_str(", world!"); // This means it's stored on the heap, not the stack

    println!("{s}"); // But the pointer IS on the stack
}

fn variables() {
    let x = 5; // <- binds the value 5 to x
    let y = x; // <- Copy x's value to y
               // Both of these values are pushed to the stack since integers are kown fixed sizes, so the issue discussed below doesn't apply
    println!("X: {x}, Y: {y}");

    // But for Strings, things are different.
    // Strings have 3 parts, a pointer, length, and capacity.
    // The pointer maps to the location on the heap where the data is stored
    let s1 = String::from("hello"); // Both s1 and s2 have the same pointer
    let s2 = s1;
    println!("{s2}");
    // println!("{s1}"); // <- Rust compiler flags this as bad since we've already borrowed the value
    let s3 = String::from("Hi there");
    let s4 = s3.clone(); // <- Performs deep copy, duplicating heap data
    println!("This is my own heap copy: {s4}");
}

fn main() {
    string_class();
    variables();
}
