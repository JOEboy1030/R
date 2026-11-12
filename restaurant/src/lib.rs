mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    println!("eat_at_restaurant　が　呼び出されました");
    hosting::add_to_waitlist();
}