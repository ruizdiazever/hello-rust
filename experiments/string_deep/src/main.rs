fn main() {
    let string_empty = String::new();
    let name = "Ever".to_string();
    let mut daughter = String::from("Sofia");

    daughter.push_str(" Hannah");

    println!(
        "String 1: {}\nString 2: {}\nString 3: {}", 
        string_empty, 
        name,
        daughter
    );

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // FIRST NO REFERENCE, problematic
    let a = "A".to_string();
    let b = "B".to_string();
    let c = "C".to_string();
    let result = a + &b + &c;
    println!("{}", result);

    // ALL REFERENCE with format!
    let name_1 = "Sofia".to_string();
    let name_2 = "Hannah".to_string();
    let surname = "Ruiz Diaz".to_string();
    let queen = format!("{} {} {}", name_1, name_2, surname);
    println!("{}", queen);

}
