fn main() {
    println!("Hello, world!");
    print_something();
    function_with_params(100);
    let res = function_with_return(100);
    println!("Result of function_with_params: {}", res);
}

// Rust use snack case in functions
fn print_something() {
    println!("Hi I'm a function");
}

fn function_with_params(x: i32) {
    let square: i32 = x * x;
    println!("The square of {} is {}", x, square);
}

fn function_with_return(x: i32) -> String {
    let square: i32 = x * x;
    println!("The square of {} is {}", x, square);
    let res: String = format!("The square of {} is {}", x, square);
    return res;
}
