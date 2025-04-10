fn main() {
    let rt = Rectangle::new(12, 9.9);

    println!("area is -> {}", rt.area());
    println!("perimeter is -> {}", rt.perimeter());
}

struct Rectangle {
    width: u32,
    height: f64,
}

impl Rectangle {
    fn new(width: u32, height: f64) -> Self {
        Rectangle { width, height }
    }
    fn area(&self) -> f64 {
        self.width as f64 * self.height
    }

    fn perimeter(&self) -> f64 {
        self.width as f64 + self.height
    }
}
