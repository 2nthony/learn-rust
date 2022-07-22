use std::collections::HashMap;

fn main() {
    a1();
    a2();
    a3();
    a4();
    a5();
}

fn a1() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in &scores {
        println!("The score of {name} is {score}");
    }

    println!("1 ok")
}

fn a2() {
    let teams = [("Chinese", 100), ("American", 10), ("france", 50)];

    let mut teams_map1 = HashMap::new();
    for team in teams {
        teams_map1.insert(team.0, team.1);
    }

    // NOTE: not works via `teams.iter()`
    let teams_map2: HashMap<&str, i32> = teams.into_iter().collect();

    assert_eq!(teams_map1, teams_map2);

    println!("2 ok")
}

fn a3() {
    fn random_stat_buff() -> u8 {
        42
    }

    let mut player_stats = HashMap::new();
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    player_stats
        .entry("health")
        .or_insert_with(random_stat_buff);

    assert_eq!(player_stats["health"], 100);

    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("3 ok")
}

fn a4() {
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Viking {
        name: String,
        country: String,
    }

    impl Viking {
        fn new(name: &str, country: &str) -> Viking {
            Viking {
                name: name.to_string(),
                country: country.to_string(),
            }
        }
    }

    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }

    println!("4 ok")
}

fn a5() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap: {}", v1);

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    m2.insert(&v2, v1);

    assert_eq!(v2, "hello");

    println!("5 ok")
}
