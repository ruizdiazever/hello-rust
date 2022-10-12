use std::collections::HashMap;

pub fn vector_game() {
    let vector = vec![1, 3, 5, 20, 1, 120, 34, 43, 45];

    // Average
    let len: i32 = vector.len() as i32;
    let sum: i32 = vector.iter().sum();
    let average: i32 = sum / (len - 1);

    // Mid value
    let mid = len / 2;
    let value_mid = &vector[mid as usize];

    // Ocurrences
    let mut map = HashMap::new();
    for value in &vector {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    println!("Mid value: {:?}", value_mid);
    println!("Average: {:?}", average);
    println!("Occurrences: {:?}", map);
}