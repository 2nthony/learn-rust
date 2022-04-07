fn main() {
    if_else();
    loop_example();
    nesting_and_labels();
    returning_from_loops();
    // while_example();
    // for_and_range();
    // for_and_iterator();
    // for_and_iterator_into_iter();
    for_and_iterator_iter_mut();
    match_example();
    match_destructure();
    structs();
    guards();
    binding();
}

fn if_else() {
    let n = 10;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        10 * n
    } else {
        println!(", and is a big number, half the number");

        n / 2
    };

    println!("{} -> {}", n, big_n);
}

fn loop_example() {
    let mut count = 0u32;
    println!("count until infinity");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough!");

            break;
        }
    }
}

fn nesting_and_labels() {
    'outer: loop {
        println!("entered the outer loop");

        'inner: loop {
            println!("entered the inner loop");

            break 'outer;
        }
    }

    println!("exited the outer loop");
}

fn returning_from_loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("loop result is {}", result);
}

fn while_example() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
}

fn for_and_range() {
    // for n in 1..101 {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn for_and_iterator() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("hello {}", name),
        }
    }
}

fn for_and_iterator_into_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("hello {}", name),
        }
    }

    // names; // moved
}

fn for_and_iterator_iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "hello",
        }
    }

    println!("names: {:?}", names);
}

fn match_example() {
    println!("---------------");

    let number = 19;
    println!("Tell me about {}", number);

    match number {
        1 => println!("one!"),
        2 | 3 | 5 | 7 | 11 => println!("prime"),
        13..=19 => println!("range of 13 and 19(includes)"),
        _ => println!("sepcial"),
    }

    let boolean = false;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}

fn match_destructure() {
    // tuples
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        // (0, y, z) => println!("first is 0, y is {:?}, z is {:?}", y, z),
        (x, -2, z) => println!("x is {:?}, second is -2, z is {}", x, z),
        (1, ..) => println!("first is 1 and the rest doesn't matter"),
        _ => println!("it doesn't matter what they are"),
    }

    // array/slices
    // similar to tuples
    // let array = [1, 2, 6];
    let array = [4, -2, 6];
    match array {
        [0, second, third] => println!(
            "array[0] is 0, array[1] is {}, array[2] is {}",
            second, third
        ),
        [1, _, third] => println!(
            "array[0] is 1, array[2] is {}, the second(array[1] was ignored)",
            third
        ),
        [-1, second, ..] => println!(
            "array[0] is -1, array[1] is {} and all the others were ignored",
            second
        ),
        [3, second, tail @ ..] => println!(
            "array[0] is 3, array[1] is {} and the others were {:?}",
            second, tail
        ),
        [first, middle @ .., last] => println!(
            "array[0] is {}, middle is {:?}, array[2] is {}",
            first, middle, last
        ),
        _ => println!("_"),
    }

    // enums
    // similar to tuples
    // Color::RGB(r, g, b)
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32),
    }

    // let color = Color::RGB(122, 17, 40);
    let color = Color::HSV(28, 23, 33);
    println!("what color is it?");

    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::RGB(r, g, b) => println!("rgb {} {} {}", r, g, b),
        Color::HSV(h, s, v) => println!("hsv {} {} {}", h, s, v),
        // ..
        _ => println!("_"),
    }

    // pointers/ref
    let reference = &4;
    match reference {
        &val => println!("got a value via destructuring {:?}", val),
    }

    match *reference {
        val => println!("got a value via dereferencing {:?}", val),
    }

    let not_a_reference = 3;
    let ref is_a_reference = 5;
    // match is_a_reference {
    //     &val => println!("is a reference {:?}", val),
    // }

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. mut_value is {:?}", m);
        }
    }
}

fn structs() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y: c } = foo;
    println!("a {}, b {}, c {}", a, b, c);

    let Foo { x: i, y: j } = foo;
    println!("i {:?}, j {:?}", i, j);

    let Foo { y, .. } = foo;
    println!("y {}", y);

    // let Foo { y } = foo; // error: missing field x
}

fn guards() {
    let pair = (3, 3);
    println!("Tell me about {:?}", pair);

    match pair {
        (a, b) if a == b => println!("These are twins"),
        (a, b) if a + b == 0 => println!("Antimatter, kaboom!"),
        (a, b) if a % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn binding() {
    fn age() -> u32 {
        15
    }

    println!("tell me what type of person you are");

    match age() {
        0 => println!("haven't celebrated my first birthday yet"),
        n @ 1..=12 => println!("i am a child of age {}", n),
        n @ 13..=19 => println!("i am a teen of age {}", n),
        n => println!("i am a elder of age {}", n),
    }

    fn some_number() -> Option<u32> {
        Some(41)
    }

    println!("match with Option");

    match some_number() {
        Some(n @ 42) => println!("The answer: {}", n),
        Some(n) => println!("not interesting in {}", n),
        _ => (),
    }
}
