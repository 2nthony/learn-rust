fn main() {
    basic();
    mutable_reference();
    aliasing();
    ref_pattern();
}

fn basic() {
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is {}", borrowed_i32);
    }

    let boxed_i32 = Box::new(5i32);
    let stacked_i32 = 6i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;
        // eat_box_i32(&boxed_i32);

        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}

fn mutable_reference() {
    #[derive(Clone, Copy)]
    struct Book {
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    fn borrow_book(book: &Book) {
        println!(
            "I immutably borrowed {} - {} edition",
            book.title, book.year
        );
    }

    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edition", book.title, book.year);
    }

    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    let mut mutabook = immutabook;

    borrow_book(&immutabook);
    borrow_book(&mutabook);

    new_edition(&mut mutabook);
    // new_edition(&mut immutabook);
}

fn aliasing() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // let mutable_borrow = &mut point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    let mutable_borrow = &mut point;
    mutable_borrow.x = 5;
    mutable_borrow.y = 3;
    mutable_borrow.z = 1;

    // let y = &point.y;

    // println!("Point Z coordinate is {}", point.z);

    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // works
    let y = &point.y;
    println!("Point Y coordinate is {}", y);
    println!("Point Z coordinate is {}", point.z);

    let new_borrowed_point = &point;
    println!(
        "Point has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}

fn ref_pattern() {
    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let c = 'Q';
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);
    assert_eq!(*ref_c1, *ref_c2);

    let point = Point { x: 0, y: 0 };

    let _copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;

        *ref_to_x
    };

    let mut mutable_point = point;

    {
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;

        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);
}
