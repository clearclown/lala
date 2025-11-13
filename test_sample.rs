// Test Rust file for syntax highlighting verification

use std::collections::HashMap;

/// Fibonacci function with memoization
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    // Variables and literals
    let x = 42;
    let y = 3.14;
    let name = "Alice";
    let is_valid = true;

    // String formatting
    println!("Hello, {}! x = {}, y = {}", name, x, y);

    // Collections
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("key1", 100);
    map.insert("key2", 200);

    // Vector and iteration
    let numbers = vec![1, 2, 3, 4, 5];
    for num in &numbers {
        println!("Number: {}", num);
    }

    // Conditional statements
    if x > 10 {
        println!("x is greater than 10");
    } else {
        println!("x is not greater than 10");
    }

    // Pattern matching
    match x {
        0..=10 => println!("Small"),
        11..=50 => println!("Medium"),
        _ => println!("Large"),
    }

    // Function call
    let fib_10 = fibonacci(10);
    println!("Fibonacci(10) = {}", fib_10);
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }
}
