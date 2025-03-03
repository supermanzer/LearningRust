use std::fs::File;
use std::io::ErrorKind;

fn unrecoverable_errors() {
    // panic!("Crash and burn")
    let v = vec![2, 3, 4, 5];

    v[20];
}

fn recoverable_errors() {
    let file_name = String::from("hello.txt");
    let greeting_result_file = File::open(file_name);
    // let greating_file = match greeting_result_file {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening this file: {error:?}"),
    // };
    let greating_file = match greeting_result_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match() File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening file: {other_error:?}")
            }
        },
    };
}

fn main() {
    // unrecoverable_errors()
    recoverable_errors()
}
