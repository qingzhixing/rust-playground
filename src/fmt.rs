use std::fmt;

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let c = Coordinate { x: 1, y: 2 };
    println!("c = {:?}", c);
    println!("c = {:#?}", c);
    println!("c = {}", c);
}
