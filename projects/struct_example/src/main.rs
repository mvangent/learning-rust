#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.h
    }

     fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

fn main() {
    let r1 = Rectangle{w: 50, h: 30};
    let r2 = Rectangle{w: 30, h: 29};

    println!("The area of the r1 = {}", r1.area());
    println!("The area of the r2 = {}", r2.area());
    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r2 hold r1? {}", r2.can_hold(&r1));
}

