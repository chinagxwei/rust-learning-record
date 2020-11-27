trait Shape {
    fn draw(&self);
}

struct Rectangle;

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle
    }
}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Rectangle::draw()")
    }
}

struct Square;

impl Square {
    fn new() -> Square {
        Square
    }
}

impl Shape for Square {
    fn draw(&self) {
        println!("Square::draw()")
    }
}

struct Circle;

impl Circle {
    fn new() -> Circle {
        Circle
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Circle::draw()")
    }
}

struct ShapeMaker {
    rectangle: Rectangle,
    square: Square,
    circle: Circle,
}

impl ShapeMaker {
    fn new() -> ShapeMaker {
        ShapeMaker {
            rectangle: Rectangle::new(),
            square: Square::new(),
            circle: Circle::new(),
        }
    }
}

impl ShapeMaker {
    fn rectangle_draw(&self) {
        self.rectangle.draw();
    }
    fn square_draw(&self) {
        self.square.draw();
    }
    fn circle_draw(&self) {
        self.circle.draw();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facade() {
        let shape_maker = ShapeMaker::new();

        shape_maker.rectangle_draw();
        shape_maker.square_draw();
        shape_maker.circle_draw();
    }
}
