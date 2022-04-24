#[allow(dead_code, unused_variables)]
fn foo<T>(arg: T) {}

struct A;

struct Single(A);

#[derive(Debug)]
struct SingleGen<T>(T);

pub fn run() {
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('b');

    println!("_char {:?}", _char);
}
