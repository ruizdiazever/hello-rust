use std::collections::HashMap;

pub fn banana() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    
    println!("----- REWRITE");
    println!("{:?}", scores);
}
