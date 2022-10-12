use std::collections::HashMap;

pub fn test() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name y field_value no son válidos en este punto, intente usarlos y
    // ¡mira qué error de compilación recibes!

    println!("{:?}", map);
}
