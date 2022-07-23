use hello_package::{eat_at_restaurant, hosting::seat_at_table};

mod front_of_house;

fn main() {
    assert_eq!(seat_at_table(), "sit down please");
    assert_eq!(eat_at_restaurant(), "yummy yummy");

    println!("ok")
}
