fn main() {
    structures();
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
