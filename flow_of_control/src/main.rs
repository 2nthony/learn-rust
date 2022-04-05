fn main() {
    if_else();
    loop_example();
    nesting_and_labels();
    returning_from_loops();
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
