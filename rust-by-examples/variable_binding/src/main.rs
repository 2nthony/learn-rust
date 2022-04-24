fn main() {
    basic();
    mutable_variables();
    scope_and_shadowing();
    declare_first();
    freeze();
}

fn basic() {
    let an_integer = 1u32;
    let unit = ();

    // copy an_integer to copied_integer
    // ?
    let copied_integer = an_integer;

    println!("An integer: {}", an_integer);
    println!("An integer: {}", copied_integer);
    println!("The unit value: {:?}", unit);
}

fn mutable_variables() {
    let immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutaion: {}", mutable_binding);

    // immutable_binding += 1;
}

fn scope_and_shadowing() {
    println!("---------------------");
    let long_lived_binding = 1;

    {
        println!(
            "access out of scope long_lived_binding: {}",
            long_lived_binding
        );
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}

fn declare_first() {
    println!("-------------------");
    let a_binding;

    {
        let x = 2;
        a_binding = x;
    }

    println!("a_binding: {}", a_binding);

    let another_binding: i32;
    // println!("another binding: {}", another_binding);

    another_binding = 1;
    println!("another_binding: {}", another_binding);
}

fn freeze() {
    println!("-----------------");
    let mut mutable_integer = 7i32;

    {
        let mutable_integer = mutable_integer;
        // let mut mutable_integer = mutable_integer;
        // mutable_integer = 50;
        // println!("inner mutable_integer: {}", mutable_integer);
    }

    mutable_integer = 3;
    println!("mutable_integer: {}", mutable_integer);
}
