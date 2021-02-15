#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, rectangle!");
    let rectangle = Rectangle {
        height: 20,
        width: 100,
    };

    let calc_area = area(&rectangle);
    println!("Area: {}", calc_area);
    println!("Rectangle: {:#?}", rectangle);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
