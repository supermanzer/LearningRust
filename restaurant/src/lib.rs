fn deliver_order() {}

mod front_of_house;

mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // <- private field
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        pub fn winter(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("oranges"),
            }
        }
        pub fn autumn(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("persimmons"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // <- super keyword to access parent module, same as Python
    }

    fn cook_order() {}
}

// Using the 'use' keyword to bring a path into scope
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();

    // Order breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change the toast
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
