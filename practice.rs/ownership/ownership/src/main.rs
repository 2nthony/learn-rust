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
}

fn a1() {
    // let x = String::from("hello world");
    // let y = &x;
    // let ref y = x;
    // let y = x.clone();
    // -----------
    // let x = &String::from("hello world");
    // let y = x;
    // -----------
    // let x = "hello world";
    // let y = x;
    // -----------
    let x = 10;
    let y = x;

    println!("1 {}, {}", x, y);
}

fn a2() {
    let s1 = String::from("hello world");
    let s2 = take_ownership(s1);

    println!("2 {}", s2);

    fn take_ownership(s: String) -> String {
        s
    }
}

fn a3() {
    let s = give_ownership();
    println!("3 {}", s);

    fn give_ownership() -> String {
        let s = String::from("hello world");
        // let _s = s.clone().into_bytes();
        // let _s = s.as_bytes();

        s
    }
}

fn a4() {
    let s = String::from("hello world");

    print_str(s.clone());
    println!("4 {}", s);

    // print_str(&s);
    // println!("4 {}", &s);

    fn print_str(s: String) {
        println!("4 {}", s);
    }
}

fn a5() {
    let x = (1, 2, (), "hello".to_string());
    // let x = (1, 2, (), "hello");
    let y = &x;
    println!("5 {:?}, {:?}", x, y);
}

fn a6() {
    let s = String::from("hello, ");

    let mut s1 = s;

    s1.push_str("world!");

    println!("6 {}", s1);
}

fn a7() {
    let mut x = Box::new(5);

    // let mut y = Box::new(*x); // works
    let mut y = Box::new(3);

    *y = 4;

    assert_eq!(*x, 5);

    *x = 1;

    println!("7 {} {}", x, y);
}

fn a8() {
    let t = (String::from("Hello"), String::from("World"));
    let _s = &t.0;

    println!("8 {:?}", t);
}

fn a9() {
    let t = (String::from("Hello"), String::from("World"));
    let (s1, s2) = t.clone(); // from solutions
    let (ref s1, ref s2) = t;

    println!("9 {:?} {:?} {:?}", s1, s2, t);
}
