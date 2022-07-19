fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
    a6();
}

#[allow(unused)]
fn example() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle { x, y, radius }
        }

        fn area(&self) -> f64 {
            self.x * self.y
        }
    }
}

fn a1() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect1.area());
}

fn a2() {
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn show_state(&self) {
            println!("the state is {}", self.color)
        }
    }

    let light = TrafficLight {
        color: "red".to_owned(),
    };

    light.show_state();

    println!("{:?}", light)
}

fn a3() {
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn show_state(self: &Self) {
            println!("the state is {}", self.color)
        }

        pub fn change_state(&mut self) {
            self.color = "green".to_string()
        }
    }

    let _light = TrafficLight {
        color: "red".to_owned(),
    };
}

fn a4() {
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn new() -> Self {
            Self {
                color: "red".to_string(),
            }
        }

        pub fn get_state(&self) -> &str {
            &self.color
        }
    }

    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
}

fn a5() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_bold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}

fn a6() {
    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLightColor {
        fn color(&self) -> &str {
            match self {
                Self::Yellow => "yellow",
                Self::Red => "red",
                Self::Green => "green",
            }
        }
    }

    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?} {}", c, c.color())
}
