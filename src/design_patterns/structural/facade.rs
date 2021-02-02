//!
//! 外观模式（Facade Pattern）隐藏系统的复杂性，并向客户端提供了一个客户端可以访问系统的接口。
//! 这种类型的设计模式属于结构型模式，它向现有的系统添加一个接口，来隐藏系统的复杂性。
//!
//! 这种模式涉及到一个单一的类，该类提供了客户端请求的简化方法和对现有系统类方法的委托调用。

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
