#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::fmt::Debug;

fn main() {
    a1();
    a2();
    a3();
}

fn a1() {
    #[derive(Debug)]
    struct Array<T, const N: usize> {
        data: [T; N],
    }

    let arrays = [Array { data: [1, 2, 3] }, Array { data: [2; 3] }];

    println!("{:?}", arrays);
}

fn a2() {
    fn print_array<T: Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr)
    }

    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}

fn a3() {
    fn check_size<T>(val: T)
    where
        Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
    {
        //...
    }

    // fix the errors in main
    fn main() {
        check_size([0u8; 767]);
        check_size([0i32; 191]);
        check_size(["hello你好"; 47]); // &str is a string reference, containing a pointer and string length in it, so it takes two word long, in x86-64, 1 word = 8 bytes
        check_size([(); 31].map(|_| "hello你好".to_string())); // String is a smart pointer struct, it has three fields: pointer, length and capacity, each takes 8 bytes
        check_size(['中'; 191]); // A char takes 4 bytes in Rust
    }

    pub enum Assert<const CHECK: bool> {}

    pub trait IsTrue {}

    impl IsTrue for Assert<true> {}
}
