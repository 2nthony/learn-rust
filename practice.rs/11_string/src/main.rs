use std::mem;

fn main() {
    a1();
    a2();
    a4();
    a5();
    a6();
    a7();
}

fn a1() {
    fn move_ownership(s: String) {
        println!("ownership of `{}` is moved here", s);
    }

    let mut s: String = String::from("hello, ");
    s.push_str("world");
    // s.push_str("!");
    s.push('!');

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");

    println!("1 ok");
}

fn a2() {
    let mut s = String::from("hello, world");

    let slice1: &str = &s;
    let slice1: &str = &s[0..];
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");

    let slice3: &mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("2 ok")
}

fn a4() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1];
    assert_eq!(slice1, "h");
    let slice2 = &s[7..10];
    assert_eq!(slice2, "世");

    // NOTE: also works with `s.chars_indices()`
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世');
        }
    }

    println!("4 ok")
}

fn a5() {
    let mut s = String::new();
    s.push_str("hello");

    let v = vec![104, 101, 108, 108, 111];

    let s1 = String::from_utf8(v).unwrap();

    assert_eq!(s, s1);

    println!("5 ok")
}

fn a6() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("6 ok")
}

fn a7() {
    let story = String::from("Rust By Practice");

    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("7 ok")
}
