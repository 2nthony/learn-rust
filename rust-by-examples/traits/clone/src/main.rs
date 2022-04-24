#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let unit = Unit;
    let copied_unit = unit;

    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("copy(moved): {:?}", moved_pair);
    // println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    // println!("copy(moved): {:?}", moved_pair);

    println!("clone: {:?}", cloned_pair);
}
