use std::ops::{Add, Mul, Sub};

fn main() {
    // example();
    a1();
    a2();
    a3();
    a4();
}

#[allow(unused)]
fn example() {
    struct Sheep {
        naked: bool,
        name: String,
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                println!("already naked {}", self.name())
            } else {
                self.naked = true
            }
        }
    }

    trait Animal {
        fn new(name: String) -> Self;
        fn name(&self) -> String;
        fn noise(&self) -> String;
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise())
        }
    }

    impl Animal for Sheep {
        fn new(name: String) -> Self {
            Sheep { name, naked: false }
        }

        fn name(&self) -> String {
            self.name.clone()
        }

        fn noise(&self) -> String {
            if self.naked {
                "baaaah?".to_string()
            } else {
                "baaaah!".to_string()
            }
        }

        fn talk(&self) {
            println!("{} pauses briefly {}", self.name, self.noise())
        }
    }

    let mut dolly: Sheep = Animal::new("Dolly".to_string());

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

fn a1() {
    trait Hello {
        fn say_hi(&self) -> String {
            String::from("hi")
        }

        fn say_something(&self) -> String;
    }

    struct Student {}
    impl Hello for Student {
        fn say_something(&self) -> String {
            String::from("I'm a good student")
        }
    }

    struct Teacher {}
    impl Hello for Teacher {
        fn say_hi(&self) -> String {
            String::from("Hi, I'm your new teacher")
        }
        fn say_something(&self) -> String {
            String::from("I'm not a bad teacher")
        }
    }

    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("ok")
}

fn a2() {
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    #[derive(Debug)]
    struct Inches(i32);
    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Seconds(i32);

    let _one_second = Seconds(1);

    println!("one second looks like {:?}", _one_second);

    let _this_is_true = _one_second == _one_second;
    let _this_is_true = _one_second > _one_second;

    let foot = Inches(12);
    println!("one foot equals {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("one foot is {} that one meter", cmp);
}

fn a3() {
    fn multipy<T: Mul<Output = T>>(a: T, b: T) -> T {
        a * b
    }

    assert_eq!(6, multipy(2u8, 3u8));
    assert_eq!(5.0, multipy(1.0, 5.0));

    println!("ok")
}

fn a4() {
    struct Foo;
    struct Bar;

    #[derive(PartialEq, Debug)]
    struct FooBar;
    #[derive(PartialEq, Debug)]
    struct BarFoo;

    impl Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> Self::Output {
            FooBar
        }
    }
    // plus
    impl Add<Foo> for Bar {
        type Output = BarFoo;

        fn add(self, _rhs: Foo) -> Self::Output {
            BarFoo
        }
    }

    impl Sub<Bar> for Foo {
        type Output = BarFoo;

        fn sub(self, _rhs: Bar) -> Self::Output {
            BarFoo
        }
    }

    // main
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Bar + Foo, BarFoo);
    assert_eq!(Foo - Bar, BarFoo);

    println!("ok")
}
