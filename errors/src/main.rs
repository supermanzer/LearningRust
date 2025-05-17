use std::fs::{self, File};
use std::io;
use std::io::{ErrorKind, Read};

fn unrecoverable_errors() {
    // panic!("Crash and burn")
    let v = vec![2, 3, 4, 5];

    v[20];
}

fn recoverable_errors() {
    let file_name = String::from("hello.txt");
    let greeting_result_file = File::open(&file_name);
    // let greating_file = match greeting_result_file {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening this file: {error:?}"),
    // };
    let greating_file = match greeting_result_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening file: {other_error:?}")
            }
        },
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    // Load file contianing username
    let username_file_result = File::open("hello.txt");
    // If we are successful, set open file to username_file.  If not, return Error
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    // Create placeholder string for username
    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut1() -> Result<String, io::Error> {
    // This function uses the ? operator to shortcut returning errors

    let mut username_file = File::open("hello.txt")?; // ? automatically returns the error to the calling function, if one is thrown
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // Same thing here but we need to explicitly handle the success case below
    Ok(username)
}

fn read_username_from_file_shortcut2() -> Result<String, io::Error> {
    // We can make this even shorter by chaining these methods
    let mut username = String::new();
    // This line performs both operations and will return early if errors are encountered with either
    File::open("hello.txt")?.read_to_string(&mut username)?;
    // If both operations are successful, return the username
    Ok(username)
}

fn read_username_from_file_shortcut3() -> Result<String, io::Error> {
    // The most concise version.  Reading strings from files is so common that there is a handy shortcut for all of these operations.
    //The one downside is, with the previous approaches we had the opportunity to maniuplate the Error object before returning it.
    fs::read_to_string("hello.txt")
}

fn main() {
    // unrecoverable_errors()
    // recoverable_errors()
    let result = read_username_from_file_shortcut2();
    match result {
        Ok(username) => println!("{username}"),
        Err(e) => println!("{e}"),
    }
}
