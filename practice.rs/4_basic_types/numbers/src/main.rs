fn main() {
    // a9();
    a11();
}

#[allow(dead_code)]
fn a9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}

fn a11() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9 / 3 == 3);

    assert!(24 % 5 == 4);

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
}
