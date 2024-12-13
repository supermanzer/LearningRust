use std::time::Instant;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    last_logged_in: Instant,
}

fn create_user(email: String, username: String) -> User {
    let user = User {
        active: true, // default to active
        username,
        email,
        last_logged_in: Instant::now(),
    };
    return user; // I prefer to have explicit return statements
}

fn get_user1() -> User {
    let username = String::from("judithwombat");
    let email = String::from("judith.wombat@bufo.io");
    return create_user(email, username);
}

fn main() {
    let user = get_user1();
    println!("Hello, user!\n{user:#?}");
}
