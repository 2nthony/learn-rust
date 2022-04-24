#[derive(Debug)]
enum Food {
    CordonBlue,
    Steak,
    Sushi,
}

#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn have_ingredients(food: &Food) -> Option<&Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: &Food) -> Option<&Food> {
    match food {
        Food::CordonBlue => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: &Food) -> Option<&Food> {
    match have_ingredients(food) {
        None => None,
        Some(food) => match have_recipe(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

fn cookable_v2(food: &Food) -> Option<&Food> {
    have_ingredients(food).and_then(have_recipe)
}

fn eat(food: &Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On '{:?}' we get to eat '{:?}'", day, food),
        None => println!("Oh no! We don't get to eat '{:?}' on '{:?}'", food, day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBlue, Food::Steak, Food::Sushi);

    eat(&cordon_bleu, Day::Monday);
    eat(&steak, Day::Tuesday);
    eat(&sushi, Day::Wednesday);
}
