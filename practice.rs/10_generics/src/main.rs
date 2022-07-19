fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
    a6();
    a7();
}

fn a1() {
    struct A;
    struct S(A);
    struct SGen<T>(T);

    fn reg_fn(_s: S) {}
    fn gen_spec_t(_s: SGen<A>) {}
    fn gen_sepc_i32(_s: SGen<i32>) {}
    fn generic<T>(_s: SGen<T>) {}

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_sepc_i32(SGen(8));
    generic::<char>(SGen('1'));
    generic(SGen('c'));
}

fn a2() {
    fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
}

fn a3() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{:?}", integer);
    println!("{:?}", float);
}

fn a4() {
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let p = Point {
        x: 5,
        y: "hello".to_string(),
    };

    println!("{:?}", p)
}

fn a5() {
    struct Val<T> {
        val: T,
    }

    impl<T> Val<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }

    let x = Val { val: 3.0 };
    let y = Val {
        val: "hello".to_string(),
    };

    println!("{} {}", x.value(), y.value())
}

fn a6() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point {
        x: "Hello",
        y: '中',
    };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
}

fn a7() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5.0, y: 10.0 };
    println!("{}", p.distance_from_origin())
}
