enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn get_vector() -> Vec<i32> {
    let _v: Vec<i32> = Vec::new(); // <- not mutable and no values, not really useful
    let mut v1: Vec<i32> = Vec::new(); // nowe we can push values, getting better.
    v1.push(9);
    v1.push(8);
    v1.push(7);

    let mut v2 = vec![1, 2, 3]; // <- Rust can infer the type of the vector if we provide initial values. This skips a lot of boilerplate shown above

    v2.push(4);
    return v2;
}

pub fn read_vector(index: usize) -> i32 {
    let v = get_vector();
    println!("Vector: {:?}", v);
    let val: &i32 = &v[index]; // Notice the difference in return types between the index and get() methods
    print!("Value at index {} is: {}\n", index, val);
    let val: Option<&i32> = v.get(index);
    print!("Value using get() at index {} is: {:?}\n", index, val);
    match val {
        Some(&value) => value,
        None => 0, // or handle the error as needed
    }
}

pub fn bad_read() {
    // let mut v = get_vector();
    let v = get_vector();
    let first: &i32 = &v[1];
    // v.push(8);
    println!("First value is: {}", first);
}

pub fn vector_iterator(is_mutable: bool) {
    if is_mutable {
        let mut v = get_vector();
        for i in &mut v {
            *i *= 50; // <- since we are borrowing here, we actually change the value in the vector
        }
        // iterate again to show the changes
        for i in &v {
            println!("{}", i);
        }
    } else {
        let v = get_vector();
        for i in &v {
            println!("{}", i);
        }
    }
}

pub fn spreadsheet() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
        }
    }
}
