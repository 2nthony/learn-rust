use std::fmt::Display;

fn main() {
    a1();
    a3();
    a4();
    a5();
    a6();
    a7();
    a8();
    a9();
    a10();
}

#[allow(unused)]
fn e1() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("aaa");
    let string2 = "xyz";

    longest(string1.as_str(), string2);

    struct Point<T> {
        x: T,
        y: T,
    }

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    impl<'a: 'b, 'b> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
        fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
            println!("{}", announcement);
            self.part
        }
    }

    fn longest_with_an_announcement<'a, T: Display>(
        x: &'a str,
        y: &'a str,
        announcement: T,
    ) -> &'a str {
        println!("Announcement {}", announcement);

        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn a1() {
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1 {}", borrow1);
    }

    {
        let borrow2 = &i;
        println!("borrow2 {}", borrow2);
    }

    println!("1 ok")
}

fn a3() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    println!("3 ok")
}

fn a4() {
    fn invalid_output() -> String {
        String::from("foo")
    }

    fn invalid_output_2() -> &'static str {
        "foo"
    }

    fn invalid_output_3<'a>(s: &'a String) -> &'a String {
        s
    }

    println!("4 ok")
}

fn a5() {
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x {x}, y {y}")
    }

    fn failed_borrow<'a>() {
        let _x: i32 = 12;
        let y: &i32 = &_x;
    }

    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow();

    println!("5 ok")
}

fn a6() {
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x {single:?}");
    println!("x and y {double:?}");
    println!("x {reference:?}");
    println!("y is not borrowed {number:?}");

    println!("6 ok")
}

fn a7() {
    #[derive(Debug)]
    struct NoCopyType {}

    #[derive(Debug)]
    struct Example<'a, 'b> {
        a: &'a u32,
        b: &'b NoCopyType,
    }

    let var_a = 35;
    let example: Example;

    // {
    let var_b = NoCopyType {};
    example = Example {
        a: &var_a,
        b: &var_b,
    };
    // }

    println!("7 ok {example:?}");
}

fn a8() {
    #[derive(Debug)]
    struct NoCopyType {}

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Example<'a, 'b> {
        a: &'a u32,
        b: &'b NoCopyType,
    }

    fn fix_me<'a, 'b>(foo: &Example<'a, 'b>) -> &'b NoCopyType {
        foo.b
    }

    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };

    fix_me(&example);

    println!("8 ok")
}

fn a9() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&'a self) -> i32 {
            3
        }
    }

    println!("9 ok")
}

fn a10() {
    fn input(x: &i32) {
        println!("annotated {}", x)
    }

    fn pass(x: &i32) -> &i32 {
        x
    }

    fn longest<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
        x
    }

    struct Owner(i32);
    impl Owner {
        fn add_one(&mut self) {
            self.0 += 1;
        }

        fn print(&self) {
            println!("{}", self.0);
        }
    }

    struct Person<'a> {
        age: u8,
        name: &'a str,
    }

    enum Eigher<'a> {
        Num(i32),
        Ref(&'a i32),
    }
}
