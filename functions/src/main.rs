fn main() {
    println!("Hello, world!");
    print_something();
    function_with_params(100);
}

// Rust use snack case
fn print_something() {
    println!("Hi I'm a function");
}

fn function_with_params(x: i32) {
    let square: i32 = x * x;
    println!("The square of {} is {}", x, square);
}
