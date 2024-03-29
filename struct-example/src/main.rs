struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 87,
        height: 50,
    };
    println!("Width: {}\nHeight: {}", rect1.width, rect1.height);
    println!("Area: {}", rect1.area());
}
