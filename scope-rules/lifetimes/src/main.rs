use std::fmt::Debug;

fn main() {
    explicit_annotation();
    functions();
    methods();
    structs();
    traits();
    bounds();
    coercion();
    statics();
    ellisons();
}

fn explicit_annotation() {
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {} and y is {}", x, y);
    }

    fn failed_borrow<'a>() {
        let _x = 12;
        // let y: &'a i32 = &_x;
    }

    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow();
}

fn functions() {
    fn print_one<'a>(x: &'a i32) {
        println!("print_one: x is {}", x);
    }

    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }

    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("print_multi: x is {}, y is {}", x, y);
    }

    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }

    // fn invalid_output<'a>() -> &'a String {
    //     &String::from("foo")
    // }

    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

fn methods() {
    struct Owner(i32);

    impl Owner {
        fn add_one<'a>(&'a mut self) {
            self.0 += 1;
        }

        fn print<'a>(&'a self) {
            println!("print: {}", self.0);
        }
    }

    let mut owner = Owner(22);
    owner.add_one();
    owner.print();
}

fn structs() {
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    };

    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is NOT borrowed in {:?}", number);
}

fn traits() {
    #[derive(Debug)]
    struct Borrowed<'a> {
        x: &'a i32,
    }

    impl<'a> Default for Borrowed<'a> {
        fn default() -> Self {
            Self { x: &10 }
        }
    }

    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}

fn bounds() {
    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);

    fn print<T>(t: T)
    where
        T: Debug,
    {
        println!("print t is {:?}", t);
    }

    fn print_ref<'a, T>(t: &'a T)
    where
        T: Debug + 'a,
    {
        println!("print_ref t is {:?}", t);
    }

    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}

fn coercion() {
    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }

    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }

    let first = 2;
    {
        let second = 3;
        println!("the product is {}", multiply(&first, &second));
        println!("{}, is the first", choose_first(&first, &second));
    }
}

fn statics() {
    static NUM: i32 = 18;

    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

    {
        let static_string = "I am in read-only-memory";
        println!("static_string {}", static_string);
    }

    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static {}", coerced_static);
    }

    println!("NUM {} staus accessible", NUM);
}

fn ellisons() {
    fn elided_input(x: &i32) {
        println!("elided_input: {}", x);
    }

    fn annotated_input<'a>(x: &'a i32) {
        println!("annotated_input: {}", x);
    }

    fn elided_pass(x: &i32) -> &i32 {
        x
    }

    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
        x
    }

    let x = 3;
    elided_input(&x);
    annotated_input(&x);

    println!("elided_pass: {}", elided_pass(&x));
    println!("annotated_pass: {}", annotated_pass(&x));
}
