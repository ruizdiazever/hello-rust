fn main() {
    // VECTOR 1
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // VECTOR 2
    let mut v2 = vec![1, 2, 3];
    v2.push(1);

    // GET VALUES
    let third_1: &i32 = &v[2];
    let third_2: Option<&i32> = v.get(2);

    // DONT EXIST
    /* let does_not_exist_1 = &v[100];
    println!("{:?}", does_not_exist_1); */
    
    println!("{:?} {:?}", v, v2);
    println!("{:?} {:?}", third_1, third_2);
    
    let does_not_exist_2= v.get(100);
    println!("{:?}", does_not_exist_2);

    // ITERATION IN NOT MUTABLE
    let v3 = vec![100, 29, 2, 31, 1000];
    for i in &v3 {
        println!("{}", i)
    }

    // ITERATION IN MUTABLE A CHANGING VALUES
    let mut v4 = vec![10, 20, 30, 40, 50];
    for i in &mut v4 {
        *i += 100;
    }
    println!("{:?}", v4);
}
