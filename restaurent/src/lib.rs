// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

//module privacy

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         // fn seat_at_table() {}
//     }

//     // mod serving{
//     //     fn take_order() {}

//     //     fn serve_order() {}

//     //     fn take_payment() {}
//     // }

//     pub fn eat_at_restaurant() {
//         //Absolute path
//         crate::front_of_house::hosting::add_to_waitlist();

//         //Relative path
//         front_of_house::hosting::add_to_waitlist();
//     }
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order(); //super allows us to reference parent module
//     }

//     fn cook_order() {}
// }


//privacy module using struct

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast{
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal: Breakfast = back_of_house::Breakfast::summer("Rye");

//     //does not work
//     // let meal2: Breakfast = back_of_house::Breakfast{
//     //     toast: String::from("Wheat"),
//     //     seasonal_fruit: String::from("peaches")
//     // };

//     meal.toast = String::from("Wheat"); // toast field is private
// }

mod back_of_house{
    pub enum Appetizer {
        Soup, 
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1: Appetizer = back_of_house::Appetizer::Soup;
    let order2: Appetizer = back_of_house::Appetizer::Salad;

}