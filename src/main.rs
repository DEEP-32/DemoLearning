mod guess_number;
mod ownership_with_slices;
mod using_structs;

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

    println!("The area of the rectangle is {}", rect1.area())
}
