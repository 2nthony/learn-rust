use std::marker::PhantomData;
use std::ops::Add;

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);
#[derive(PartialEq)]
struct PhantomSturct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

pub fn run() {
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomSturct<char, f32> = PhantomSturct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomSturct<char, f64> = PhantomSturct {
        first: 'Q',
        phantom: PhantomData,
    };

    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);
    // println!("_struct 1 == _struct2 yields: {}", _struct1 == _struct2);
}

// pub trait Add<RHS = Self> {
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }

#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

pub fn run_testcase() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one foot = {}", two_feet.0);
    println!("one meter + one meter = {}", two_meters.0);

    // let one_feter = one_foot + one_meter;
}
