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
    // If you have mutable reference to a value, you can have no other reference to it
    let mut s = String::from("hello");

    change(&mut s);
    println!("{s}");
    // You cannot make two mutable references to the same value in the same scope
    // You CAN achieve this by restricting the scope though!

    {
        // start inner scope
        let r1 = &mut s;
        println!("{r1}")
    } // end inner scope

    // The same thing holds true for mixing immutable and mutable references
    let mut s2 = String::from("Hi there");
    let r3 = &s2;
    let r4 = &s2;
    println!("{}, {}", r3, r4); // this is okay because the references end at this line
    let r2 = &mut s2;
    println!("{}", r2); // if we tried to print them here, it would cause an error
}

fn dangling_pointer() -> &String {
    let s = String::from("Hello wombat");

    &s // <- return a pointer to the string
} // but s goes out of scope here, so this won't work.

fn main() {
    first_reference();
    mut_ref();
    let ref_to_nil = dangling_pointer();
}
