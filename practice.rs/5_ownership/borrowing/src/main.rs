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
}

fn a1() {
    let x = 5;
    let p = &x;

    println!("1 x mem address {:p}", p);
}

fn a2() {
    let x = 5;
    let y = &x;

    assert_eq!(&5, y);

    println!("2 success");
}

fn a3() {
    let mut s = String::from("hello, ");
    borrow_str(&s);
    println!("3 success");

    fn borrow_str(s: &String) {}
}

fn a4() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("4 {}", s);

    fn push_str(s: &mut String) {
        s.push_str("world");
    }
}

fn a5() {
    let mut s = String::from("hello, ");

    // let mut p = s;
    let p = &mut s;

    p.push_str("world");

    println!("5 {}", p);
}

fn a6() {
    let c = '中';

    let r1 = &c;
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    assert_eq!(get_addr(r1), get_addr(r2));

    println!("6 success");

    fn get_addr(r: &char) -> String {
        format!("{:p}", r)
    }
}

fn a7() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("7 {} {}", r1, r2);
}

fn a8() {
    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("8 success");

    fn borrow_object(s: &mut String) {}
}

fn a9() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("9 {}", s);

    fn borrow_object(s: &String) {}
}

fn a10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");

    let r2 = &mut s;
    r2.push_str("!!");

    // println!("10 {} {}", &r1, &r2);
}

fn a11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    r1.push_str("world");

    // println!("11 {} {}", r1, r2);
}
