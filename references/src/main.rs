fn calculate_length(s: &String) -> usize {
    // The & in the type declaration indicates we will pass a reference
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn first_reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Passing reference using &

    println!("The length of {s1} is {len}");
}

fn mut_ref() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");
}

fn main() {
    first_reference();
    mut_ref();
}
