fn main() {
    a2();
    a3();
    a4();
    a5();
}

fn a2() {
    // 填空
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg1 = Message::Move { x: 1, y: 2 }; // 使用x = 1, y = 2 来初始化
    let msg2 = Message::Write("hello, world!".to_string()); // 使用 "hello, world!" 来初始化

    println!("1 success");
}

fn a3() {
    // 仅填空并修复错误
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::Move { x: 2, y: 2 };

    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
        println!("2 success");
    } else {
        panic!("不要让这行代码运行！");
    }
}

fn a4() {
    // 填空，并修复错误
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // [_; 3]
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }

    fn show_message(msg: Message) {
        println!("4 {:?}", msg);
    }
}

fn a5() {
    // 填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("5 {:?}", n);
        return;
    }

    panic!("不要让这行代码运行！");

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
        // from the tips
        // x.map(|i| i + 1)
    }
}
