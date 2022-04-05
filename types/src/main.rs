fn main() {
    casting();
    literals();
    inference();
    aliasing();
}

fn casting() {
    // let decimal = 65.4321_f32; // A
    let decimal = 67.890_f32; // C

    let integer: u8 = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);

    // println!("1000 as u8 is: {}", 1000 as u8);
    println!("-1 as a u8 is: {}", -1i8 as u8);

    println!("1000 mod 256 is: {}", 1000 % 256);

    println!("128 is a i16 is: {}", 128 as i16);
    // println!("128 as a i8 is: {}", 128 as i8); // -128 ~ 127

    // println!("1000 as a u8 is: {}", 1000 as u8); // 0 ~ 255
    // println!("232 as a i8 is: {}", 232 as i8); // -128 ~ 127

    println!("300.0 is {}", 300.0_f32 as u8);
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    println!("nan as u8 is {}", f32::NAN as u8);
}

fn literals() {
    println!("-------------------");
    use std::mem;

    let x = 1u8;
    let xx: u8 = 1;
    let y = 2u32;
    let yy: u32 = 2;
    let z = 3f32;
    let zz: f32 = 3.0;
    println!("x {} {}, y {} {}, z {} {}", x, xx, y, yy, z, zz);

    println!("size of `x` in bytes: {}", mem::size_of_val(&x));
    println!("size of `xx` in bytes: {}", mem::size_of_val(&xx));
    println!("size of `y` in bytes: {}", mem::size_of_val(&y));
    println!("size of `yy` in bytes: {}", mem::size_of_val(&yy));
    println!("size of `z` in bytes: {}", mem::size_of_val(&z));
    println!("size of `zz` in bytes: {}", mem::size_of_val(&zz));

    let i = 1;
    let f = 1.0;
    println!("size of `i` in bytes: {}", mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", mem::size_of_val(&f));
}

fn inference() {
    let elem = 5u8;

    let mut vec = Vec::new();

    vec.push(elem);

    println!("{:?}", vec);
}

fn aliasing() {
    println!("----------------");

    type NanoSecond = u64;
    type Inch = u64;

    type u64_t = u64;

    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!(
        "{} nanoseonds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
