mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        //fn seat_at_table() {}
    }

    // mod serving {
    //     fn take_order() {}

    //     fn serve_order() {}

    //     fn take_payment() {}
    // }
}

// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();

//     front_of_house::hosting::add_to_waitlist();
// }

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // a public struct can have private filed
    pub struct Breakfast {
        // this is public field
        pub toast: String,
        // this is private filed
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"), 
            }
        }
    }

    // All enum annotated with pub in every case is a good choice based on rust doc
    // a public enum have all public enum keyword
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//     // private field can't access directly
//     //meal.seasonal_fruit = String::from("blueberries");
// }
pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// use can be in this scope, but cannot in another scope
use crate::front_of_house::hosting;

// pub use can be use in more scopes
// pub use crate::front_of_house::hosting;

pub fn eat() {
    hosting::add_to_waitlist();
}

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// use std::io::Result as IoResult;
// fn function2() -> IoResult<()> {
//     // --snip
// }

