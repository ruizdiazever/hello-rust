fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    no_dangle();
    let s = String::from("Hola mundo");
    let hello = &s[0..4];
	let world = &s[5..10];
    println!("{} {}", hello, world);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    println!("{}", s);
    s
}

/* fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
} */
