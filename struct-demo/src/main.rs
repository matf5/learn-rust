mod hosting;

pub use crate::hosting::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
}
