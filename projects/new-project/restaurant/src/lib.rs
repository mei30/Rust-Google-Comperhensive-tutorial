mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute
    hosting::add_to_wait_list();

    // Relative
    hosting::add_to_wait_list();
}

mod serving {
    fn take_order() {}

    fn save_order() {}

    fn take_payment() {}

    fn eat_at_restaurant() {
        let mut order1 = back_of_house::Appetizer::Soup;
        let mut order2 = back_of_house::Appetizer::Salad;

        let mut meal = back_of_house::BreakFast::summer("Rye");

        meal.toast = String::from("Wheat");

        println!("I'd like {} toast please", meal.toast);
    }

    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }

        fn fix_incorrect_order() {
            super::save_order();
        }
    pub struct BreakFast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl BreakFast {
        pub fn summer(toast: &str) -> BreakFast {
            BreakFast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}
}
