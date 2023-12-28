mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
        fn seat_at_table(){}
    }
    mod serving {
        fn take_order(){}
        fn server_order(){}
        fn take_payment() {}
    }
}
fn deliver_order(){}
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),

            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order(){
    }
    
}
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please",meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

use std::collections::HashMap;

fn create_map() {
    let mut map = HashMap::new();
    map.insert(1,2);
}

use std::fmt;
use std::io::{self,Write};
/*
 * One way to use two result types in the same scope
fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
*/

use std::collections::*;
