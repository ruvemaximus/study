use std::ops;


struct Point {
    x: u8,
    y: u8,
}


impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { 
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Point { 
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { 
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


impl Point { 
    fn is_zero(&self) -> bool {
        return self.x == self.y && self.x == 0;
    }
    fn distance_to(&self, other: &Self) -> f32 {
        return (((self.x - other.x) ^ 2 + (self.y - other.y) ^ 2) as f32).sqrt();
    }
}


fn main() {
    let p1 = Point {
        x: 10, 
        y: 10
    };

    let p2 = Point { 
        x: 2, 
        y: 3
    };

    let res = p1 + p2;

    println!("x is {}; y is {}", res.x, res.y);
}

