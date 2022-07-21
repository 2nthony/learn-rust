use std::{fmt::Display, ops::Sub};

fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
}

fn example() {
    pub trait CacheableItem: Clone {
        type Address: AsRef<[u8]>;
        fn is_null(&self) -> bool;
    }
}

fn e2() {
    struct Counter;
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            self.next()
        }
    }
}

fn a1() {
    struct Container(i32, i32);

    trait Contains {
        type A;
        type B;

        fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        type A = i32;
        type B = i32;

        fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        fn first(&self) -> Self::A {
            self.0
        }

        fn last(&self) -> Self::B {
            self.1
        }
    }

    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {} {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("number is {} {}", container.first(), container.last());
    println!("the difference is {}", difference(&container));
    println!("ok");
}

fn a2() {
    #[derive(PartialEq, Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T: Sub<Output = T>> Sub<Point<T>> for Point<T> {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
        Point { x: 1, y: 3 }
    );
    println!("2-2 ok")
}

fn a3() {
    trait Pilot {
        fn fly(&self) -> String;
    }
    trait Wizard {
        fn fly(&self) -> String;
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) -> String {
            "This is your captain speaking".to_string()
        }
    }

    impl Wizard for Human {
        fn fly(&self) -> String {
            "Up!".to_string()
        }
    }

    impl Human {
        fn fly(&self) -> String {
            "waving arms furiously".to_string()
        }
    }

    let person = Human;

    assert_eq!(Pilot::fly(&person), "This is your captain speaking");
    assert_eq!(person.fly(), "waving arms furiously");

    println!("3 ok");
}

fn a4() {
    trait Person {
        fn name(&self) -> String;
    }

    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn fav_language(&self) -> String;
    }

    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    fn comp_sci_student_gretting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name {} and I attend {}. My faviourite language is {}. My git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }

    struct CSStudent {
        name: String,
        university: String,
        fav_language: String,
        git_username: String,
    }

    // TODO impl
    impl Person for CSStudent {
        fn name(&self) -> String {
            self.name.clone()
        }
    }
    impl Student for CSStudent {
        fn university(&self) -> String {
            self.university.clone()
        }
    }
    impl Programmer for CSStudent {
        fn fav_language(&self) -> String {
            self.fav_language.clone()
        }
    }
    impl CompSciStudent for CSStudent {
        fn git_username(&self) -> String {
            self.git_username.clone()
        }
    }

    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string(),
    };

    println!("{}", comp_sci_student_gretting(&student));
}

fn a5() {
    struct Pretty(String);

    impl Display for Pretty {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "\"{}\"", self.0.clone() + ", world")
        }
    }

    let w = Pretty("hello".to_string());
    println!("w = {}", w)
}
