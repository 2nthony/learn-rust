struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

pub fn run() {
    println!("-------------------");

    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    // empty;
    // null;
}
