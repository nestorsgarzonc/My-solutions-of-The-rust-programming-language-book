fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(10);
    v.push(20);
    let first_element = &v[0];
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    println!("first_element: {}", first_element);
    println!("Hello, world!");
    // Iterate
    for i in &v {
        println!("{}", i);
    }
    // Iterate and modify
    for i in &mut v {
        *i += 10;
    }
    for i in &v {
        println!("{}", i);
    }
}
