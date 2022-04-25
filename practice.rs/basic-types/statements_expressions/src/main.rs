fn main() {
    a1();
    a2();
    a3();
}

fn a1() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
    // assert_eq!(v, ());
}

fn a2() {
    let v = {
        let x = 3;
        x
    };

    assert_eq!(v, 3);
}

fn a3() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
}
