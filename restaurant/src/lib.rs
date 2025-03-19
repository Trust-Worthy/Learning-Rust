

mod front_of_house {
    
   pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {

        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order(){}

mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,

    }

    impl Breakfast {
        
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches")
            }
        }
    }



    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order(){}
}



mod customer {

    use super::back_of_house::{Breakfast,Appetizer};
    use crate::front_of_house::hosting;
    
    pub fn eat_at_restaurant() {
    
        // absolute path
        crate::front_of_house::hosting::add_to_waitlist();
    
        // relative path 
        hosting::add_to_waitlist();
    
        let mut meal = Breakfast::summer("rye");
    
        meal.toast = String::from("Wheat"); // customer changing their food option
        // beacuse toast field is public I can write and read this field
    
        let order1 = Appetizer::Salad;
        let order2 = Appetizer::Soup;
        
        hosting::add_to_waitlist();
    
    
    }
}


// the weird case where two items with the same name are brought into the same scope.
// can use the as keyword or just use fmt::Result and io::Result respectively
use std::fmt::Result as fmtResult;
use std::io::Result as IoResult;




