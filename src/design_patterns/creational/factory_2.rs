//!
//! 这种类型的设计模式属于创建型模式，它提供了一种创建对象的最佳方式。
//!
//! 在工厂模式中，我们在创建对象时不会对客户端暴露创建逻辑，并且是通过使用一个共同的接口来指向新创建的对象。
//!


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
        println!("Inside Rectangle::draw() method.")
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
        println!("Inside Square::draw() method.")
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
        println!("Inside Circle::draw() method.")
    }
}

enum ShapeType {
    CIRCLE,
    RECTANGLE,
    SQUARE,
}

struct ShapeFactory;

impl ShapeFactory {
    fn get_shape(r#type: ShapeType) -> Box<dyn Shape> {
        match r#type {
            ShapeType::CIRCLE => Box::new(Circle::new()),
            ShapeType::RECTANGLE => Box::new(Rectangle::new()),
            ShapeType::SQUARE => Box::new(Square::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_factory() {
        let shape_1 = ShapeFactory::get_shape(ShapeType::CIRCLE);
        shape_1.draw();
        let shape_2 = ShapeFactory::get_shape(ShapeType::RECTANGLE);
        shape_2.draw();
        let shape_3 = ShapeFactory::get_shape(ShapeType::SQUARE);
        shape_3.draw();
    }
}
