fn main() {
    let s = "Hi";
    let string = s.to_string();
    println!("Hello, {}!", string);
    let mut s_1 = String::from("HI");
    s_1.push_str(" Rustaceans");
    println!("{}!", s_1);
    let s3 = string + &s_1;
    println!("{}!", s3);

}
