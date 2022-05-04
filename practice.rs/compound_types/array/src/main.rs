fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
    a6();
}

// fn example(n: i32) {
//     let arr = [1; n];
// }

fn a1() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(a.len(), 5);

    println!("1 success");
}

fn a2() {
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    assert_eq!(std::mem::size_of_val(&arr0), 12);
    assert_eq!(std::mem::size_of_val(&arr), 12);
    println!("2 success");
}

fn a3() {
    let list: [i32; 100] = [1; 100];
    assert_eq!(list[0], 1);
    assert_eq!(list.len(), 100);

    println!("3 success");
}

fn a4() {
    let _arr = [1, 2, 3];
    println!("4 success");
}

fn a5() {
    let arr = ['a', 'b', 'c'];
    let first = arr[0];
    assert_eq!(first, 'a');

    println!("5 success");
}

fn a6() {
    let names = [String::from("Sunface"), "2nthony".to_string()];
    let name0 = names.get(0).unwrap();
    let name1 = &names[1];

    println!("6 {} {}", name0, name1);
}
