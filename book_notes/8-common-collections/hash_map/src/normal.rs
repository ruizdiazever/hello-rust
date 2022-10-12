use std::collections::HashMap;

pub fn scores() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}",scores);
}