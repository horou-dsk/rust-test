use std::fs::read;

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn like(&self, rect: &Rectangle) -> bool {
        self.width == rect.width && self.height == rect.height
    }

    fn clone(&self) -> Rectangle {
        Rectangle {
            width: self.width,
            height: self.height
        }
    }
}

pub fn run() {
    let rect = Rectangle {
        width: 100,
        height: 900
    };
    let rect1 = rect.clone();
    println!("Hello, world! 我只想说能输出中文不 {} {}", rect.like(&rect1), rect.area());
}
