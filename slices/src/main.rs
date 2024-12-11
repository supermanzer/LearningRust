// Finding the first word in a string
// https://doc.rust-lang.org/book/ch04-03-slices.html

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // return slice of string
        }
    }
    &s[..] // return entire string
}

// Examine slices with other collections
fn other_slices() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3])
}

fn main() {
    let string = String::from("Wily wombats weasel walnuts while winking");
    let index = first_word(&string);
    println!("{index}");

    other_slices();
}
