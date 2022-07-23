use core::fmt;
use std::{num::ParseIntError, str::FromStr};

fn main() {
    a1();
    a2();
    a3();
}

fn a1() {
    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "The point is ({}, {})", self.x, self.y)
        }
    }

    let origin = Point { x: 0, y: 0 };

    assert_eq!(origin.to_string(), "The point is (0, 0)");
    println!("{}", origin);
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("1 ok")
}

fn a2() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("2 ok")
}

fn a3() {
    #[derive(Debug, PartialEq, Eq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl FromStr for Point {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let coords: Vec<&str> = s
                .trim_matches(|p| p == '(' || p == ')')
                .split(',')
                .collect();

            let x_fromstr = coords[0].parse::<i32>()?;
            let y_fromstr = coords[1].parse::<i32>()?;

            Ok(Point {
                x: x_fromstr,
                y: y_fromstr,
            })
        }
    }

    let p = Point::from_str("(3,4)");
    let p = "(3,4)".parse::<Point>();
    assert_eq!(p.unwrap(), Point { x: 3, y: 4 });

    println!("3 ok")
}
