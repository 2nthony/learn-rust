fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    // collected_iterator.push(10);
    println!("Colllected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);
    xs.push(4);
    println!("Vector: {:?}", xs);

    println!("Vector size: {}, second element is: {}", xs.len(), xs[1]);

    println!("Pop last element: {:?}", xs.pop());
    // println!("Fourth element: {}", xs[3]);

    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 5;
    }
    println!("Vector after multiplication: {:?}", xs);
}
