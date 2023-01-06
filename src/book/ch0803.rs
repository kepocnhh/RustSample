use std::collections::HashMap;

pub fn run() {
    const CHAPTER: u8 = 8;
    const PART: u8 = 3;
    const TITLE: &str = "Storing Keys with Associated Values in Hash Maps";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    creating()
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
