use std::io;

fn main() {
    println!("Welcome to the temperature converter!");
    println!("Write 1 if you want to convert from Fahrenheit to Celsius");
    println!("Write 2 if you want to convert from  Celsius to Fahrenheit");
    println!("Otherwise the program exit");
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");
    println!("You guessed: {}", option);
    let option: u32 = option.trim().parse().expect("Opps");
    println!("Write the temp to convert");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    println!("You guessed the temp of: {}", temp);
    let temp: f32 = temp.trim().parse().expect("Opps");
    if option == 1 {
        let res = fahre_to_cel(temp);
        println!("The temp in Celcius is: {}", res)
    } else if option == 2 {
        let res = cel_to_fahre(temp);
        println!("The temp in Fahrenheit is: {}", res)
    } else {
        println!("Incorrect option")
    }
}

fn fahre_to_cel(temp: f32) -> f32 {
    return (temp - 32.0) * 5.0 / 9.0;
}
fn cel_to_fahre(temp: f32) -> f32 {
    return (temp * 9.0 / 5.0) + 32.0;
}
