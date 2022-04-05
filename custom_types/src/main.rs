fn main() {
    structures();
    enums();
    type_aliases();
    use_use();
    linked_list();
    consts();
}

fn structures() {
    println!("----------------structures-------------------");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    struct Unit;

    struct Pair(i32, f32);

    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let name = String::from("peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // destructure
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    // the Unit struct (like empty/null?)
    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    // practice
    fn rect_area() {
        let rect = Rectangle {
            top_left: Point { x: 1.0, y: 2.0 },
            bottom_right: Point { x: 2.0, y: 1.0 },
        };

        let Rectangle {
            top_left: tl,
            bottom_right: br,
        } = rect;

        println!("rect_area() {:?} {:?}", tl, br);
    }
    rect_area();
    fn square(p: Point, n: f32) -> Rectangle {
        Rectangle {
            top_left: p,
            bottom_right: Point { x: n, y: n },
        }
    }

    println!("square() {:?}", square(Point { x: 1.0, y: 2.0 }, 0.2));
}

fn enums() {
    m();

    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page load"),
            WebEvent::PageUnload => println!("page unload"),
            WebEvent::KeyPress(c) => println!("pressed '{}'", c),
            WebEvent::Paste(s) => println!("pasted '{}'", s),
            WebEvent::Click { x, y } => println!("clicked at x {}, y {}", x, y),
        }
    }

    fn m() {
        let pressed = WebEvent::KeyPress('x');
        inspect(pressed);

        // let my_text = String::from("my text");
        // let pasted = WebEvent::Paste(my_text);
        let pasted = WebEvent::Paste("my text".to_owned());
        inspect(pasted);

        let clicked = WebEvent::Click { x: 20, y: 30 };
        inspect(clicked);

        let load = WebEvent::PageLoad;
        inspect(load);

        let unload = WebEvent::PageUnload;
        inspect(unload);
    }
}

fn type_aliases() {
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    let verbose_add = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    let operations_add = Operations::Add;
    let verbose_substract = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;
    let operations_substract = Operations::Subtract;

    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    println!("verbose_add.run: {}", verbose_add.run(2, 2));
    println!("operations_add.run: {}", operations_add.run(3, 2));
    println!("verbose_substract.run: {}", verbose_substract.run(3, 2));
}

fn use_use() {
    m();

    enum Status {
        Rich,
        Poor,
    }

    enum Work {
        Civilian,
        Soldier,
    }

    fn m() {
        use Status::{Poor, Rich};
        use Work::*;

        let status = Poor;
        let status_poor_origin_use = Status::Poor;
        let civilian = Civilian;
        let vibilian_origin = Work::Civilian;

        match status {
            Poor => println!("The poor guy has no money"),
            Rich => println!("The rich guy has a lot of money"),
        }

        match civilian {
            Civilian => println!("Civilians work!"),
            Soldier => println!("Soldiers fight!"),
        }
    }
}

fn linked_list() {
    m();

    // use List::*;

    enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List {
            List::Nil
        }

        fn prepend(self, element: u32) -> List {
            List::Cons(element, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                List::Cons(_, ref tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail) => {
                    format!("{}, {}", head, tail.stringify())
                }
                List::Nil => {
                    format!("Nil")
                }
            }
        }
    }

    fn m() {
        let mut list = List::new();

        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        println!("linked_list has length: {}", list.len());
        println!("{}", list.stringify());
    }
}

static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;
fn consts() {
    let n = 16;
    fn is_big(n: i32) -> bool {
        n > THRESHOLD
    }

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "Big" } else { "Small" });

    // const cannot modify
    // THRESHOLD = 4;
}
