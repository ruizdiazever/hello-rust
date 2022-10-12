mod basic;
mod method;
mod ownership;
mod exercises;

fn main() {
    use basic::name_age;
    use basic::sum;
    use basic::super_basic;
    use basic::triangle;
    use basic::QUEEN;
    use ownership::batman;
    use exercises::vector_game;

    let cancel: bool = false;
    if cancel {
        name_age();
    }

    let separator = String::from("----------------------");

    println!("{}", separator);
    let num_1: i32 = 5;
    let num_2: i32 = 5;
    println!("The sum is: {}", sum(num_1, num_2));

    println!("{}", separator);
    println!("My queen is {:?}", QUEEN);

    println!("{}", separator);
    super_basic();

    // Triangle
    let a: u32 = 8;
    let b: u32 = 10;
    let c: u32 = 11;
    triangle(a, b, c);

    // OWNERSHIP
    batman();

    // Methods
    struct Square {
        side: u32,
    }

    impl Square {
        fn area(&self) -> u32 {
            self.side * self.side
        }
    }

    let square_1 = Square { side: 10 };
    println!("Area de square_1: {}", square_1.area());

    // EXERCISES
    println!("{}", separator);
    vector_game();
}
