use std::convert::{From, TryFrom};
use std::string::ToString;

fn main() {
    from_example();
    into_example();
    try_from_and_try_into_example();
    tostring_and_fromstr();
}

fn from_example() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30);
    println!("My number is {:?}", num);
}

fn into_example() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

fn try_from_and_try_into_example() {
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

fn tostring_and_fromstr() {
    struct Circle {
        radius: i32,
    }

    impl ToString for Circle {
        fn to_string(&self) -> String {
            format!("Circle of radius {:?}", self.radius)
        }
    }

    let circle = Circle { radius: 5 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    println!("parsed: {} and turbo_parsed: {}", parsed, turbo_parsed);
    println!("Sum: {:?}", parsed + turbo_parsed);
}
