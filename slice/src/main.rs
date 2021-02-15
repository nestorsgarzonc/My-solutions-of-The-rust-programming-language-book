fn main() {
    // Note:
    // println!("Using b' ' convert the value to ASCII");
    // println!("{}", b' ');
    print_lines();
    println!("First exercise");
    let word = String::from("asda ");
    let value = first_word(&word);
    println!("{}", value);
    print_lines();
    println!("Second exercise\n");
    let s = String::from("Hello NW");
    let hello = &s[0..5];
    let nw = &s[6..];
    println!("{}", hello);
    println!("{}", nw);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn print_lines() {
    for _ in 0..2 {
        println!("#################################");
    }
}
