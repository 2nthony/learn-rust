fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
    a6();
}

fn a1() {
    let _t0: (u8, i16) = (0, -1);
    // 元组的成员还可以是一个元组
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // 填空让代码工作
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("1 success")
}

fn a2() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("2 success")
}

fn a3() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("3 too long tuple: {:?}", too_long_tuple);
}

fn a4() {
    let tup = (1, 6.4, "hello");

    // 填空
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("4 success")
}

fn a5() {
    let (x, y, z);

    // 填空
    (y, z, x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("5 success")
}

fn a6() {
    // 填空，需要稍微计算下
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("6 success");

    fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
        (nums.0 + nums.1, nums.0 * nums.1)
    }
}
