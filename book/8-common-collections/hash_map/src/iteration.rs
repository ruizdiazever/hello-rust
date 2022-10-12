use std::collections::HashMap;

pub fn for_normal() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("----- FOR");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
