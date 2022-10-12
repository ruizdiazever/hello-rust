use std::io;

pub const QUEEN: &str = "Sofia";

pub fn name_age() {
    let name = String::from("Ever");
    let age: u32 = 29;

    println!("Hi {}, do you are {} yeard old? true?", name, age);
    println!("1. Yes\n2. No");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == 1 {
            println!("Welcome {}", name);
            break;
        } else if input == 2 {
            println!("Please, introduce your correct age:");
            let mut age_str = String::new();
            io::stdin()
                .read_line(&mut age_str)
                .expect("Failed to read line");
            let age_str: u32 = match age_str.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("Ok, {}, of {} years old, welcome!", name, age_str);
            break;
        }
    }
}

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn super_basic() {
    let num_1: u8 = 5;
    let num_2: i8 = -5;
    let num_3: f32 = 3.8;
    let name: &str = "Sofia Hannah";
    let verdad: bool = true;
    let chara: char = 'F';
    let tupla: (u8, i8, f32) = (10, -7, 3.8);
    let matrix = ["Sofia", "Valentina", "Ever"];

    println!("Num (u8): {}", num_1);
    println!("Num (i8): {}", num_2);
    println!("Num (f32): {}", num_3);
    println!("Name (str): {}", name);
    println!("Verdad (bool): {}", verdad);
    println!("Character (char): {}", chara);
    println!("Tupla (u8, i8, f32): {:?}", tupla);
    println!("Matrix (matrix): {:?}", matrix);
}

pub fn triangle(a: u32, b: u32, c: u32) {
    if a == b && a == c {
        println!("The triangle is a Equilatero");
    } else if a == b || a == c {
        println!("The triangle is a Isosceles");
    } else if a != b && a != c && b != c {
        println!("The triangle is a Escaleno");
    }
}
