use std::collections::HashMap;

pub fn run() {
    const CHAPTER: u8 = 8;
    const PART: u8 = 3;
    const TITLE: &str = "Storing Keys with Associated Values in Hash Maps";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    creating();
    ownership();
    updating();
}

fn creating() {
    println!("\nCreating a New Hash Map");

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    println!("map: {:?}", scores);

    let team = "Blue";
    let score = scores.get(team).copied().unwrap_or(0);
    println!("team: \"{team}\" score: {score}");

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}

fn ownership() {
    println!("\nHash Maps and Ownership");

    let mut map = HashMap::new();
    let key = "foo";
    map.insert(key, 1);
    println!("{:?}", map);
    map.insert(key, 2);
    println!("{:?}", map);
    map.entry(key).or_insert(3);
    println!("{:?}", map);
    map.entry("bar").or_insert_with(|| 4);
    println!("{:?}", map);
}

fn updating() {
    println!("\nUpdating a Value Based on the Old Value");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
