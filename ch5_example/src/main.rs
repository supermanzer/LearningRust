// Task: Create a program that calculates the area of a rectangle

// A function to calculate the area of a rectangle using signle variables
// The signature of this function doesn't really give us much information
// about what it is we are doing
fn area_single(width: u32, height: u32) -> u32 {
    let area = width * height;
    return area;
}

// refactoring to use a Tuple Struct
// this isn't actually much better.  The tuple
// doesn't name the elements so we don't know
// which is height and which is width
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    let area = dimensions.0 * dimensions.1;
    return area;
}

// Let's add some proper structure!
// Now it's a bit more clear what each of these u32 numbers pertain to
// However, we still can't print this struct because Rust doesn't
// "guess" how this should be represented.

// We CAN opt into displaying debugging info like so
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// This function only works on our rectangle struct so it would make more sense as a method of that struct!
fn area_struct(rectangle: &Rectangle) -> u32 {
    let area = rectangle.width * rectangle.height;
    return area;
}
// Adding a method to our Rectangle struct - not sure how I like this syntax
// I guess this could allow for more readable definitions for large structs with lots of methods.  I wonder if you could even split the data definition and methods in separate files?
impl Rectangle {
    fn area(&self) -> u32 {
        let area = self.width * self.height;
        return area;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        let result = self.width > other.width && self.height > other.height;
        return result;
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of a rectanlge is {}",
        area_single(width1, height1)
    );
    println!(
        "The area of a tuple rectangle is {}",
        area_tuple((width1, height1))
    );
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // println!("Rectangle info: {rect1:#?}");
    dbg!(&rect1); // using native debugging macro - this includes more detail
    println!("The area of a struct rectangle is {}", area_struct(&rect1));
    println!("The area of a method rectangle is {}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
