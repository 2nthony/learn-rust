fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
    a6();
}

fn a1() {
    let arr = [1, 2, 3];
    let _s1: &[i32] = &arr[0..2];
    let _s2: &str = "hello world" as &str;

    println!("1 success");
}

fn a2() {
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[0..2];

    println!("2 {:?}", slice);
    assert_eq!(std::mem::size_of_val(&arr), 12);
    assert_eq!(std::mem::size_of_val(&slice), 16);
    println!("2 success");
}

fn a3() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("3 success");
}

fn a4() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // 填空，不要再使用 0..2
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("4 success");
}

fn a5() {
    let s = "你好，世界";
    // 修改以下代码行，让代码工作起来
    let slice = &s[0..3];

    assert_eq!(slice, "你");

    println!("5 success");
}

fn a6() {
    let mut s = String::from("hello world");

    // 这里, &s 是 `&String` 类型，但是 `first_word` 函数需要的是 `&str` 类型。
    // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
    let word = first_word(&s);

    // move this line
    // s.clear(); // error!

    println!("6 the first word is: {}", word);

    s.clear();

    fn first_word(s: &str) -> &str {
        &s[..1]
    }
}
