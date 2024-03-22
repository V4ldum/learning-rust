pub use crate::front_of_the_house::hosting;

mod front_of_the_house;
mod back_of_the_house;

fn deliver_order() {}

pub fn eat_at_the_restaurant() {
    let order1 = back_of_the_house::Appetizer::Soup;
    let order2 = back_of_the_house::Appetizer::Salad;

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_the_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}