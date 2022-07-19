use std::ops::{Add, Mul, Sub};

fn main() {
    // example();
    a1();
    a2();
    a3();
    a4();
    a5();
    a6();
    a7();
    a8();
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

fn a5() {
    trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    struct Post {
        title: String,
        author: String,
        content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("The author of post {} is {}", self.title, self.author)
        }
    }

    #[derive(Debug)]
    struct Weibo {
        username: String,
        content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} published a weibo {}", self.username, self.content)
        }
    }

    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome".to_string(),
    };

    let weibo = Weibo {
        username: "Sunface".to_string(),
        content: "Weibo seem to be worse that Twitter".to_string(),
    };

    fn summary(item: &impl Summary) {
        item.summarize();
    }

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}

fn a6() {
    struct Sheep {}
    struct Cow {}

    trait Animal {
        fn noise(&self) -> String;
    }

    impl Animal for Sheep {
        fn noise(&self) -> String {
            "baah".to_string()
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> String {
            "moo".to_string()
        }
    }

    // 无意义答案
    // fn random_animal(random_number: f64) -> impl Animal {
    //     if random_number < 0.5 {
    //         Sheep {}
    //     } else {
    //         Sheep {}
    //     }
    // }

    // 但 Box<dyn> 是下一章的知识
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    let random_number = 0.23;
    let animal = random_animal(random_number);

    println!(
        "you've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}

fn a7() {
    fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
        x + y
    }

    assert_eq!(sum(1, 2), 3);
}

fn a8() {
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("the largest member is x = {:?}", self.x);
            } else {
                println!("the largest member is y = {:?}", self.y);
            }
        }
    }

    #[derive(PartialEq, PartialOrd, Debug)]
    struct Unit(i32);

    let pair = Pair {
        x: Unit(1),
        y: Unit(3),
    };

    pair.cmp_display();
}
