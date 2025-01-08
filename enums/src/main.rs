enum IpAddrVersion {
    V4,
    V6,
}
// This is a bit overly verbose. We can be more concise here

struct IpAddr {
    version: IpAddrVersion,
    address: String,
}
// This allows us to define both the type, V4 or V6, and the string

enum IpAddress {
    V4(String),
    V6(String),
}
// You can define enums to hold different values per variant
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor { r: i32, g: i32, b: i32 },
}
// And even add functions!
impl Message {
    fn call(&self) {
        // do something here
    }
}
fn build_network(address: IpAddress) -> String {
    match address {
        IpAddress::V4(string) => {
            println!("Adding V4 IP Address {string}");
            string
        }
        IpAddress::V6(string) => {
            println!("Adding V6 IP Address {string}");
            string
        }
    }
}
#[derive(Debug)]
enum UsState {
    Alaska,
    California,
    Colorado,
    Washington,
    Oregon, // We don't care about the rest :p
}
// Building an enum for coin matching
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // capturing those state quarters, you know the ones
}

fn get_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("Got a Some variant with a value of {i}");
            Some(i + 1)
        }
    }
}

fn add_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn catch_dice() {
    let dice_roll = 4;
    // Match patterns are evaluated in order, so be sure to put your catch-all pattern last!
    match dice_roll {
        3 => add_hat(),
        6 => remove_hat(),
        other => move_piece(other), // this catches any value for dice_roll that isn't the two conditions listed above. You can use this approach if you want to do something with the value.
    }
}

fn catch_dice2() {
    let dice_roll = 6;
    match dice_roll {
        3 => add_hat(),
        6 => remove_hat(),
        _ => reroll(), // If we don't want to use the value, we assign to _
                       // _ => ()  <- you can also do this if you don't want to do anything
    }
}

fn add_hat() {
    println!("You have a hat!")
}
fn remove_hat() {
    println!("NO hat for you!")
}
fn move_piece(n: i32) {
    println!("Move your piece forward {n} spaces")
}
fn reroll() {
    println!("Roll dice again!")
}

fn main() {
    let coin = Coin::Quarter(UsState::California);
    let cents = get_cents(coin);
    println!("Got {cents}");
    let address = IpAddress::V4(String::from("127.0.0.1"));
    let val = build_network(address);
    println!("Navigate browser to {val}");
    add_one();
    catch_dice();
    catch_dice2();
}
