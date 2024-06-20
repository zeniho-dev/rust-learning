/*
    Source : https://www.youtube.com/watch?v=784JWR4oxOI
*/

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn print_area<T: Shape>(shape: &T) {
    println!("Area: {}", shape.area());
}

fn main() {
    let rectangle = Rectangle {
        width: 5.0,
        height: 3.0,
    };
    print_area(&rectangle);

    let circle = Circle { radius: 2.5 };
    print_area(&circle);

    // A vector of shapes.
    //
    // This vector holds instances of different shapes that implement the `Shape` trait.
    // It allows for storing and manipulating different shapes in a polymorphic way.
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(rectangle), Box::new(circle)];

    for shape in shapes.iter() {
        println!("Area: {}", shape.area());
        // Tell me how to send a shape vec item to print_area function.
    }
}
