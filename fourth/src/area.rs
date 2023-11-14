trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn compute_area<T: Area>(shape: &T) -> f64 {
    shape.area()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_area() {
        let circle = Circle { radius: 5.0 };
        let triangle = Triangle {
            base: 10.0,
            height: 8.0,
        };
        let square = Square { side: 6.0 };
        assert_eq!(compute_area(&circle), 25f64 * std::f64::consts::PI);
        assert_eq!(compute_area(&triangle), 40f64);
        assert_eq!(compute_area(&square), 36f64);
    }
}
