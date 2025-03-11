// src/bin/basic_structs.rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle(width, height);
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("Running basic_structs example...\n");
    
    let rect = Rectangle::new(10, 20);
    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());
}