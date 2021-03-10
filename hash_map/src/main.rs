use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Aa"), 10);
    scores.insert(String::from("ea"), 12);
    let score = scores.get(&String::from("ea"));
    match score {
        Some(x) => println!("Score for 'ea' key: {}", x),
        None => println!("Not found"),
    }
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
