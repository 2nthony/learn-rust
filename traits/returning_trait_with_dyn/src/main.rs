use rand::prelude::*;

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_numer = random::<f64>();
    let animal = random_animal(random_numer);

    println!(
        "you've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
