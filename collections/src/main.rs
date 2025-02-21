mod maps;
mod problems;
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

    let m = maps::create_map();
    println!("Map: {:?}", m);
    let val = maps::access_map(&m, "a");
    println!("Value: {}", val);
    maps::interate_map(&m);

    maps::update_map();

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 4, 4, 6, 6, 6, 6];
    let (mean, median, mode) = problems::mean_median_mode(numbers);
    println!("Mean: {}, Median: {}, Mode: {}", mean, median, mode);

    let s = "Hello from a string literal!";
    let pig_latin = problems::to_pig_latin(s);
    println!("Pig Latin: {}", pig_latin);
}
