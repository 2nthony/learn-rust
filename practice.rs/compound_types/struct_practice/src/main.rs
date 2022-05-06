fn main() {
    a1();
    a2();
    a3();
    a4();
    a6();
    a7();
    a8();
}

fn a1() {
    struct Person {
        name: String,
        age: u8,
        hobby: String,
    }

    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding rust"),
    };

    println!("1 success");
}

fn a2() {
    struct Unit;
    trait SomeTrait {}
    impl SomeTrait for Unit {}

    let u = Unit;
    do_something_with_unit(u);

    println!("2 success");

    fn do_something_with_unit(u: Unit) {}
}

fn a3() {
    struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32); // misleading struct

    let v = Color(0, 127, 255);
    check_color(v);

    println!("3 success");

    fn check_color(p: Color) {
        let Color(x, _, z) = p;
        assert_eq!(x, 0);
        assert_eq!(p.1, 127);
        assert_eq!(z, 255);
    }
}

fn a4() {
    // 填空并修复错误，不要增加或移除代码行
    struct Person {
        name: String,
        age: u8,
    }

    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18?
    p.age = 30;

    // 填空
    p.name = String::from("sunfei");

    println!("4 success");
}

// ignore 5

fn a6() {
    // 填空，让代码工作
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("6 {:?}", u2);

    fn set_email(u: User) -> User {
        User {
            email: String::from("contact@im.dev"),
            ..u
        }
    }
}

fn a7() {
    // 填空，让代码工作
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
        height: 50,
    };

    dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr

    println!("7 {:?}", rect1); // 打印 debug 信息到标准输出 stdout
}

fn a8() {
    // 修复错误
    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }

    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    // let _name = f.name;
    let File {
        name: _name,
        ref data,
    } = f;

    // 只能修改这一行
    println!("8 {}, {}", _name, f.data);
}
