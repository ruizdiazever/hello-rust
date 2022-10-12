use std::io;

fn main() {
    let mut name = String::new();
    
    println!("Please introduce your name");
    io::stdin().read_line(&mut name)
            .expect("Failed to read line");

    println!("Your name is {}", name);


}
