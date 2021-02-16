#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn build_square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    println!("Hello, rectangle!");
    let rectangle = Rectangle {
        height: 20,
        width: 100,
    };
    let rectangle1 = Rectangle {
        height: 100,
        width: 100,
    };
    let calc_area = rectangle.area();
    println!("Area: {}", calc_area);
    println!("Rectangle 1: {:#?}", rectangle);
    println!("Rectangle 2: {:#?}", rectangle1);
    println!(
        "Rectangle 1 can hold rectangle 2: {:#?}",
        rectangle.can_hold(&rectangle1)
    );
    println!("Square: {:#?}", Rectangle::build_square(100));
}
