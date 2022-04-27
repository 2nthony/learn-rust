fn main() {
    a1();
    a2();
    // a3();
    // a4();
    a5();
}

fn a1() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
}

fn a2() {
    print();

    fn print() -> () {
        println!("Hello, world!");
    }
}

fn a3() {
    never_return();

    fn never_return() -> ! {
        panic!("I return nothing")
    }
}

fn a4() {
    println!("4");
    get_option(1);

    fn get_option(tp: u8) -> Option<i32> {
        match tp {
            1 => {}
            _ => {}
        };

        never_return_fn()
    }

    fn never_return_fn() -> ! {
        // panic!("")
        unimplemented!()
        // loop {
        //     std::thread::sleep(std::time::Duration::from_secs(1))
        // }
    }
}

fn a5() {
    let b = false;

    let _v = match b {
        true => 1,
        false => {
            println!("success");
            panic!("we have no value for false")
        }
    };

    println!("5 excercise failed");
}
