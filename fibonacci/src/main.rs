use std::io;

fn main() {
    println!("Enter the number to calculate the fibonacci number!");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    println!("You guessed: {}", number);
    let number: i32 = number.trim().parse().expect("Opps");
    let res = fibonacci(number);
    println!("The result is: {}", res);
}

fn fibonacci(number: i32) -> i64 {
    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    } else {
        return fibonacci(number - 1) + fibonacci(number - 2);
    }
}
