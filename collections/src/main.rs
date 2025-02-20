mod strings;
mod vectors;

use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // <- enable backtraces
    let v = vectors::get_vector();
    let val = vectors::read_vector(1);
    vectors::bad_read();
    vectors::vector_iterator(true);
    println!(
        "Hello, world!, I have a vector: {:?} and a value: {:?}",
        v, val
    );
    vectors::spreadsheet();

    let s = strings::new_string();
    println!("String: {}", s);

    let s = strings::string_from_literal();
    println!("String: {}", s);

    strings::string_concat();
    strings::string_format();
}
