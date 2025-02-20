pub fn new_string() -> String {
    let mut s = String::new();
    s.push_str("Hello, ");
    s.push_str("world!");
    return s;
}

pub fn string_from_literal() -> String {
    let s: &str = "Hello from a string literal!";
    return s.to_string(); // any type that implements Display can be converted to a String
}

pub fn string_ownership() {
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("String: {}", s3);
}

pub fn string_concat() {
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = String::from(" wombat");
    let s = s1 + &s2 + &s3; // this kind of concatenation is inefficient
    println!("String: {}", s);
}

pub fn string_format() {
    let s1 = String::from("Hello");
    let s2 = String::from("world!");
    let s3 = String::from(" I am a wombat");
    let s = format!("{} {}{}", s1, s2, s3); // this is more efficient
    println!("String: {}", s);
}
