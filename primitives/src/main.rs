use std::mem;

fn main() {
    basic();
    literals_and_operators();
    tuples();
    arrays_and_slices();
}

fn basic() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    // ignore rest, learnt
}

fn literals_and_operators() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("true AND false is {}", true && false);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("1 million is written as {}", 1_000_000u32);

    // ignore rest
}

fn tuples() {
    println!("-----------------Tuples-------------------");
    m();

    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;

        (boolean, integer)
    }

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    fn m() {
        let long_tuple = (
            1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
        );

        println!("long tuple first value: {}", long_tuple.0);
        println!("long tuple second value: {}", long_tuple.1);

        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
        println!("tuple of tuples: {:?}", tuple_of_tuples);

        // But long Tuples cannot be printed
        // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
        // println!("too long tuple: {:?}", too_long_tuple);
        // TODO ^ Uncomment the above 2 lines to see the compiler error

        let pair = (1, true);
        println!("pair is {:?}", pair);
        println!("the reversed pair is {:?}", reverse(pair));

        println!("one element tuple: {:?}", (5u32,));
        println!("just an integer: {:?}", (5u32));

        let tuple = (1, "hello", 4.5, true);

        let (a, b, c, d) = tuple;
        println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{:?}", matrix);
    }
}

fn arrays_and_slices() {
    println!("-----------------Array and Slices-------------------");

    fn analyze_slice(slice: &[i32]) {
        println!("first el of the slice: {}", slice[0]);
        println!("the slice has {} els", slice.len());
    }

    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // create an array, length 500, fill 0
    let ys: [i32; 500] = [0; 500];

    println!("First el of the 'xs' array: {}", xs[0]);
    println!("Second el of the 'xs' array: {}", xs[1]);
    println!("number of els in 'xs' array: {}", xs.len());

    println!("First el of the 'ys' array: {}", ys[0]);
    println!("number of els in 'ys' array: {}", ys.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // self run
    let less_bytes_array: [i8; 5] = [1, 2, 3, 4, 5];
    println!(
        "less_bytes_array {} bytes",
        mem::size_of_val(&less_bytes_array)
    );

    println!("\nborrow the whole array as a slice");
    analyze_slice(&xs);

    println!("\nborrow a section of the array as a slice");
    analyze_slice(&ys[0..4]);

    // error: out of bound
    // println!("{}", xs[5]);
}
