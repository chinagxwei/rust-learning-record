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

struct ShapeDecorator {
    decorated_shape: Box<dyn Shape>
}

impl ShapeDecorator {
    fn new(decorated_shape: Box<dyn Shape>) -> ShapeDecorator {
        ShapeDecorator { decorated_shape }
    }
}

impl Shape for ShapeDecorator {
    fn draw(&self) {
        self.decorated_shape.draw()
    }
}

struct RedShapeDecorator {
    shape_decorator: ShapeDecorator
}

impl RedShapeDecorator {
    fn new(decorated_shape: Box<dyn Shape>) -> RedShapeDecorator {
        RedShapeDecorator { shape_decorator: ShapeDecorator::new(decorated_shape) }
    }
}

impl RedShapeDecorator {
    fn set_red_border(&self, _: Box<&dyn Shape>) {
        println!("Border Color: Red");
    }
}

impl Shape for RedShapeDecorator {
    fn draw(&self) {
        self.shape_decorator.draw();
        self.set_red_border(Box::new(&self.shape_decorator));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decorator() {
        let circle = Circle::new();
        let red_circle = RedShapeDecorator::new(Box::new(Circle::new()));
        let red_rectangle = RedShapeDecorator::new(Box::new(Rectangle::new()));

        println!("Circle with normal border");
        circle.draw();

        println!("\nCircle of red border");
        red_circle.draw();

        println!("\nRectangle of red border");
        red_rectangle.draw();
    }
}
