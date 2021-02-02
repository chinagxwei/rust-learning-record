//!
//! 装饰器模式（Decorator Pattern）允许向一个现有的对象添加新的功能，同时又不改变其结构。
//! 这种类型的设计模式属于结构型模式，它是作为现有的类的一个包装。
//!
//! 这种模式创建了一个装饰类，用来包装原有的类，并在保持类方法签名完整性的前提下，提供了额外的功能。
//!
//! 我们通过下面的实例来演示装饰器模式的用法。
//! 其中，我们将把一个形状装饰上不同的颜色，同时又不改变形状类。
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

trait AbsShapeDecorator {
    fn get_shape_decorator(&self) -> &ShapeDecorator;
    fn draw(&self) {
        self.get_shape_decorator()
            .draw();
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

impl AbsShapeDecorator for RedShapeDecorator {
    fn get_shape_decorator(&self) -> &ShapeDecorator {
        &self.shape_decorator
    }

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
