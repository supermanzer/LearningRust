// Function extraction
fn largest_num(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

fn main() {
    // Initial approach
    // let number_list = vec![35, 54, 12, 78, 56, 98];

    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {largest}");

    // // Doing all of this again for a second list is tedious
    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {largest}");

    // Abstracted approach
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_num(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_num(&number_list);
    println!("The largest number is {result}");
}
