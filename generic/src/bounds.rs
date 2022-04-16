use std::fmt::Debug;
use std::fmt::Display;

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

pub fn run() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _traingle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("area {}", area(&rectangle));
}

// testcase
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}
// impl Red for Turkey {}

fn red<T: Red>(_: &T) -> &'static str {
    "Red"
}

fn blue<T: Blue>(_: &T) -> &'static str {
    "Blue"
}

pub fn run_testcase() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));
}
