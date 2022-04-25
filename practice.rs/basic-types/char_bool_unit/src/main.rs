use std::mem::size_of_val;

fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
    a6();
}

fn a1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("success")
}

fn a2() {
    let c1 = '中';
    print_char(&c1);

    fn print_char(c: &char) {
        println!("{}", c);
    }
}

fn a3() {
    let _f: bool = false;
    let t = true;
    if t {
        println!("3 success");
    }
}

fn a4() {
    let f: bool = false;
    let t = true && false;
    assert_eq!(t, f);
    println!("4 success");
}

fn a5() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicity_ret_unit());

    println!("5 success");

    fn implicity_ret_unit() -> (i8, i8) {
        // println!("I will return a ()")
        (2, 3)
    }
}

fn a6() {
    let unit: () = ();
    assert_eq!(size_of_val(&unit), 0);

    println!("6 success");
}
