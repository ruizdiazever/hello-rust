fn main() {
    println!("Print from main function");
    another_fn(86, 01);
    declaration();

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_fn(x: i32, y: i32) {
    println!("Print from another function with value: {} {}", x, y);
}

fn declaration() {
    //let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}