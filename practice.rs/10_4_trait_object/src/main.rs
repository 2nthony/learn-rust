fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
    a5_2();
}

fn a1() {
    trait Bird {
        fn quack(&self) -> String;
    }

    struct Duck;
    impl Duck {
        fn swim(&self) {
            println!("look, the duck swimming");
        }
    }

    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("look, the swan flying");
        }
    }

    impl Bird for Duck {
        fn quack(&self) -> String {
            "duck".to_string()
        }
    }

    impl Bird for Swan {
        fn quack(&self) -> String {
            "swan".to_string()
        }
    }

    // implement hatch_a_bird
    fn hatch_a_bird(n: u8) -> Box<dyn Bird> {
        if n == 2 {
            Box::new(Duck {})
        } else {
            Box::new(Swan {})
        }
    }

    let duck = Duck {};
    duck.swim();

    let bird = hatch_a_bird(2);
    // bird.swim();
    assert_eq!(bird.quack(), "duck");

    let bird = hatch_a_bird(1);
    // bird.fly();
    assert_eq!(bird.quack(), "swan");

    println!("1 ok");
}

fn a2() {
    trait Bird {
        fn quack(&self);
    }

    struct Duck;
    impl Duck {
        fn fly() {
            println!("the duck is flying");
        }
    }

    struct Swan;
    impl Swan {
        fn fly() {
            println!("the swan is flying");
        }
    }

    impl Bird for Duck {
        fn quack(&self) {
            println!("{}", "duck");
        }
    }

    impl Bird for Swan {
        fn quack(&self) {
            println!("{}", "swan");
        }
    }

    let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck {}), Box::new(Swan {})];
    // below is docs answer
    // let birds: [Box<dyn Bird>; 2] = [Box::new(Duck {}), Box::new(Swan {})];

    for bird in birds {
        bird.quack();
    }
}

fn a3() {
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    fn draw_with_box(x: Box<dyn Draw>) {
        x.draw();
    }

    fn draw_with_ref(x: &dyn Draw) {
        x.draw();
    }

    let x = 1.1f64;
    let y = 8u8;

    draw_with_box(Box::new(x));
    draw_with_ref(&y);

    println!("3 ok")
}

fn a4() {
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String {
            format!("u8: {}", *self)
        }
    }

    impl Foo for String {
        fn method(&self) -> String {
            format!("string: {}", *self)
        }
    }

    fn static_dispatch<T: Foo>(x: T) {
        x.method();
    }
    fn dynamic_dispatch(y: &dyn Foo) {
        y.method();
    }

    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("4 ok");
}

/**
 * level 4
 */
fn a5() {
    trait MyTrait {
        fn f(&self) -> Self;
    }

    impl MyTrait for u32 {
        fn f(&self) -> Self {
            42
        }
    }

    impl MyTrait for String {
        fn f(&self) -> Self {
            self.clone()
        }
    }

    fn my_function(x: impl MyTrait) -> impl MyTrait {
        x.f()
    }
    my_function(13_u32);
    my_function(String::from("abc"));

    println!("5 ok");
}

fn a5_2() {
    trait MyTrait {
        fn f(&self) -> Box<dyn MyTrait>;
    }

    impl MyTrait for u32 {
        fn f(&self) -> Box<dyn MyTrait> {
            Box::new(42)
        }
    }

    impl MyTrait for String {
        fn f(&self) -> Box<dyn MyTrait> {
            Box::new(self.clone())
        }
    }

    fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
        x.f()
    }

    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("5-2 ok");
}
