fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
    a6();
    a7();
    a8();
    a9();
    a10();
    a11();
    a12();
    a13();
    a14();
    a15();
}

fn a1() {
    enum Direction {
        East,
        West,
        North,
        South,
    }

    let dire = Direction::West;

    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        Direction::West => {
            println!("West")
        }
    }
}

fn a2() {
    let boolean = true;

    let binary = match boolean {
        true => 1,
        false => 0,
    };

    println!("{:?}", assert_eq!(binary, 1));
}

fn a3() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    fn show_message(msg: Message) {
        match msg {
            Message::Move { x, y } => {
                assert_eq!(x, 1);
                assert_eq!(y, 3);
            }
            Message::ChangeColor(_, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            _ => println!("no data in these variants"),
        }
    }
}

fn a4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }
    println!("pass")
}

fn a5() {
    enum MyEnum {
        Foo,
        Bar,
    }

    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }

    // 2
    println!("{}", count)
}

fn a6() {
    let o = Some(7);

    if let Some(i) = o {
        println!("pass {}", i)
    }
}

fn a7() {
    enum Foo {
        Bar(u8),
    }

    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i)
    }
}

fn a8() {
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Qux(10);

    match a {
        Foo::Bar => {
            println!("Match foo::bar")
        }
        Foo::Baz => {
            println!("match foo::baz")
        }
        _ => {
            println!("match others")
        }
    }
}

fn a9() {
    let age = Some(30);

    if let Some(age) = age {
        assert_eq!(age, 30);
    }

    match age {
        Some(age) => println!("age is a new var, the value is {}", age),
        _ => {}
    }
}

fn a10() {
    match_number(2);

    fn match_number(n: i32) {
        match n {
            1 => println!("1"),
            2 | 3 | 4 | 5 => {
                println!("2 ~ 5")
            }
            6..=10 => {
                println!("6 ~ 10")
            }
            _ => {
                println!("11+")
            }
        }
    }
}

fn a11() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 20 };

    match p {
        Point { x, y: 0 } => {
            println!("x: {}", x)
        }
        Point {
            x: 0..=5,
            y: y @ (10 | 20 | 30),
        } => {
            println!("y: {}", y)
        }
        Point { x, y } => {
            println!("{} {}", x, y)
        }
    }
}

fn a12() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 11 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("id in range [3, 7]: {}", id)
        }
        Message::Hello {
            id: newid @ (10 | 11 | 12),
        } => {
            println!("id in range [10, 12]: {}", newid)
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

fn a13() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => {
            println!("{}", x);
            assert!(x < split)
        }
        Some(x) => {
            println!("{}", x);
            assert!(x >= split)
        }
        None => (),
    }
}

fn a14() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
            println!("{}, {}", first, last);
        }
    }
}

fn a15() {
    let mut v = String::from("Hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world"),
    }
}
