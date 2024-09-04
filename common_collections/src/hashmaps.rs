use std::collections::HashMap;

pub fn create_hashmap() -> HashMap<String, i32> {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    let score = scores.get(&team_name).copied().unwrap_or(0);
    scores
}

pub fn iterate_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (k, v) in &scores {
        println!("{k}: {v}");
    }
}

pub fn update_hashmap() {
    let text = "hello world wonderfull world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}")
}