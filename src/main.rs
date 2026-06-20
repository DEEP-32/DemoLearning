mod enum_learning;
mod guess_number;
mod ownership_with_slices;
mod using_structs;
use enum_learning::Coin::{self, Quater};
use using_structs::Rectangle;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {}", rect1.area());

    let config_max = Some(3u8);

    let current_quater = Quater(enum_learning::UsState::Alaska);

    if let Coin::Quater(state) = current_quater {
        println!("State quater is from {state:?}!")
    } else {
    }
}
