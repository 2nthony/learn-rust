mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() -> String {
    hosting::add_to_waitlist();
    back_of_house::cook_order();

    String::from("yummy yummy")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
