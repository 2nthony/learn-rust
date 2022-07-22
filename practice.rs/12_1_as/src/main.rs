fn main() {
    a1();
    a2();
    a3();
}

#[allow(unused)]
fn a1() {
    let decimal = 97.123_f32;
    let integer: u8 = decimal as u8;

    let c1: char = decimal as u8 as char;
    let c2 = integer as char;

    assert_eq!(integer, 'a' as u8);

    println!("1 ok")
}

fn a2() {
    assert_eq!(u8::MAX, 255);

    #[allow(overflowing_literals)]
    let v = 1000 as u8;

    println!("2 ok")
}

#[allow(overflowing_literals)]
fn a3() {
    println!("{}", u16::MAX - 1);
    assert_eq!(1000 as u16, 1000);

    println!("{}", u8::MAX - 1);
    assert_eq!(1000 as u8, 232);
    println!("1000 mod 256 is {}", 1000 % 256);

    println!("{}", -1_i8 as u8);
    assert_eq!(-1_i8 as u8, 255);

    println!("{}", 300.1_f32 as u8);
    assert_eq!(300.1_f32 as u8, 255);

    assert_eq!(-100.1_f32 as u8, 0);

    println!("3 ok")
}
