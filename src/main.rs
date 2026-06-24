use crate::using_traits::{SocialPost, Summary};
use adder::add;

// mod enum_learning;
// mod guess_number;
//mod ownership_with_slices;
// mod using_structs;
// use enum_learning::Coin::{self, Quater};
// use using_structs::Rectangle;
mod using_traits;

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course  as you probably already know people"),
        reply: false,
        repost: false,
    };

    println!("new post {}", post.summarize());
}
