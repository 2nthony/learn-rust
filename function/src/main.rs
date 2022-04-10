fn main() {
    fizzbuzz();
    method();
    closures();
    closures_capturing();
    closures_as_input_parameters();
    closures_anonnymous();
    closures_input_function();
    closure_as_output_parameters();
    std_iterator_any();
    std_iterator_find();
    hof();
    diverging_function();
}

fn fizzbuzz() {
    fizzbuzz_to(69);

    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs == 0
    }

    fn fizzbuzz(n: u32) {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz")
        } else if is_divisible_by(n, 3) {
            println!("fizz")
        } else if is_divisible_by(n, 5) {
            println!("buzz")
        } else {
            println!("{}", n);
        }
    }

    fn fizzbuzz_to(n: u32) {
        for n in 1..=n {
            fizzbuzz(n);
        }
    }
}

fn method() {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }
    }

    #[derive(Debug)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        // sugar: &mut slef
        fn translate(self: &mut Self, x: f64, y: f64) {
            self.p1.x += x;
            self.p1.y += y;
            self.p2.x += x;
            self.p2.y += y;
        }
    }

    struct Pair(Box<i32>, Box<i32>);
    impl Pair {
        fn destroy(self) {
            let Pair(first, second) = self;
            println!("Destroying Pair({}, {})", first, second);
        }
    }

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    // not mutable for rectangle
    // rectangle.translate(1.0, 1.0);

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(2.0, 2.0);
    println!("square translated: {:?}", square);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

fn closures() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());
}

fn closures_capturing() {
    use std::mem;

    let color = String::from("green");
    let print = || println!("Color {}", color);
    print();

    let _reborrow_color = &color;
    print();

    let _color_moved = color;
    // print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count {}", count);
    };
    inc();
    // let _reborrow_count = &count; // error
    inc();
    // let _reborrow_count = &mut count;

    let movable = Box::new(3);
    let consume = || {
        println!("movable {:?}", movable);
        mem::drop(movable);
    };
    consume();
    // consume(); // dropped

    let haystack = vec![1, 2, 3];
    // let contains = move |needle| haystack.contains(needle);
    let contains = |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    println!("There are {} elements in vec", haystack.len());
}

fn closures_as_input_parameters() {
    use std::mem;

    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        println!("I said {}", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("now I can sleep");

        mem::drop(farewell);
    };

    apply(diary);

    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    let double = |x| 2 * x;
    println!("3 doubled {}", apply_to_3(double));
}

fn closures_anonnymous() {
    fn apply<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    let x = 7;
    let print = || println!("{}", x);
    apply(print);
}

fn closures_input_function() {
    fn call_me<F: Fn()>(f: F) {
        f()
    }

    fn function() {
        println!("i am a function");
    }

    let closure = || println!("i am a closure");

    call_me(closure);
    call_me(function);
}

fn closure_as_output_parameters() {
    println!("---------------");

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        move || println!("This is a {}", text)
    }

    let fn_plain = create_fn();
    fn_plain();

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("this is a {}", text)
    }

    let mut fn_mut = create_fnmut();
    fn_mut();

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        move || println!("this is a {}", text)
    }

    let fn_once = create_fnonce();
    fn_once();
}

fn std_iterator_any() {
    // pub trait Iterator {
    //     type Item;
    //
    //     fn any<F>(&mut self, f: F) -> bool
    //     where
    //         F: FnMut(Self::Item) -> bool,
    //     {
    //     }
    // }

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1 {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2 {}", vec2.iter().any(|&x| x == 2));
}

fn std_iterator_find() {
    // pub trait Iterator {
    //     type Item;

    //     fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    //     where
    //         P: FnMut(&Self::Item) -> bool,
    //     {
    //     }
    // }

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1 {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2 {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1 {:?}", array1.iter().find(|&&x| x == 2));
    println!(
        "Find 2 in array2 {:?}",
        array2.into_iter().find(|&x| x == 2)
    );
}

// high order function
fn hof() {
    println!("----------------");

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("find the sum of all the squared odd numbers under 1000");

    let upper = 1000;
    let mut acc = 0;

    // imperative
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }

    println!("imperative style {}", acc);

    // functional style
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);

    println!("functional style {}", sum_of_squared_odd_numbers)
}

fn diverging_function() {
    println!("------------------");

    fn foo() -> ! {
        panic!("this call never return");
    }

    fn some_fn() {
        ()
    }

    let a: () = some_fn();

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue,
            };

            acc += addition;
        }

        acc
    }

    println!(
        "Sum of odd numbers up to 9(excluding) {}",
        sum_odd_numbers(9)
    );
}
