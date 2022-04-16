fn main() {
    basic();
    mutablity();
    partial_moves();
}

fn basic() {
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);
    }

    let x = 5u32;

    let y = x;

    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);
    println!("a contains {}", a);

    let b = a;
    // println!("a contains {}", a);

    destroy_box(b);
}

fn mutablity() {
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);

    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);
}

fn partial_moves() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    let Person { name, ref age } = person;
    println!("The person age is {}", age);
    println!("The person name is {}", name);

    // println!("The person struct is {:?}", person);

    // name;

    println!("The person's age from person struct is {}", person.age);
}
