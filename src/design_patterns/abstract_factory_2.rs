trait Shape {
    fn draw(&self);
}

enum ShapeType {
    CIRCLE,
    RECTANGLE,
    SQUARE,
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

trait Color {
    fn fill(&self);
}

enum ColorType {
    RED,
    GREEN,
    BLUE,
}

struct Red;

impl Red {
    fn new() -> Red {
        Red
    }
}

impl Color for Red {
    fn fill(&self) {
        println!("Inside Red::fill() method.")
    }
}

struct Green;

impl Green {
    fn new() -> Green {
        Green
    }
}

impl Color for Green {
    fn fill(&self) {
        println!("Inside Green::fill() method.")
    }
}

struct Blue;

impl Blue {
    fn new() -> Blue {
        Blue
    }
}

impl Color for Blue {
    fn fill(&self) {
        println!("Inside Blue::fill() method.")
    }
}

trait AbstractFactory {
    fn get_color(&self, r#type: ColorType) -> Option<Box<dyn Color>>;
    fn get_shape(&self, r#type: ShapeType) -> Option<Box<dyn Shape>>;
}


enum ChoiceType {
    SHAPE,
    COLOR,
}

struct ColorFactory;

impl ColorFactory {
    fn new() -> ColorFactory {
        ColorFactory
    }
}

impl AbstractFactory for ColorFactory {
    fn get_color(&self, r#type: ColorType) -> Option<Box<dyn Color>> {
        match r#type {
            ColorType::RED => Some(Box::new(Red::new())),
            ColorType::GREEN => Some(Box::new(Green::new())),
            ColorType::BLUE => Some(Box::new(Blue::new()))
        }
    }

    fn get_shape(&self, _: ShapeType) -> Option<Box<dyn Shape>> {
        None
    }
}

struct ShapeFactory;

impl ShapeFactory {
    fn new() -> ShapeFactory {
        ShapeFactory
    }
}

impl AbstractFactory for ShapeFactory {
    fn get_color(&self, _: ColorType) -> Option<Box<dyn Color>> {
        None
    }

    fn get_shape(&self, r#type: ShapeType) -> Option<Box<dyn Shape>> {
        match r#type {
            ShapeType::CIRCLE => Some(Box::new(Circle::new())),
            ShapeType::RECTANGLE => Some(Box::new(Rectangle::new())),
            ShapeType::SQUARE => Some(Box::new(Square::new()))
        }
    }
}

struct FactoryProducer;

impl FactoryProducer {
    fn get_factory(r#type: ChoiceType) -> Box<dyn AbstractFactory> {
        match r#type {
            ChoiceType::COLOR => Box::new(ColorFactory::new()),
            ChoiceType::SHAPE => Box::new(ShapeFactory::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory() {
        let shape_factory = FactoryProducer::get_factory(ChoiceType::SHAPE);
        let shape_1 = shape_factory.get_shape(ShapeType::CIRCLE);
        shape_1.unwrap().draw();
        let shape_2 = shape_factory.get_shape(ShapeType::RECTANGLE);
        shape_2.unwrap().draw();
        let shape_3 = shape_factory.get_shape(ShapeType::SQUARE);
        shape_3.unwrap().draw();

        let color_factory = FactoryProducer::get_factory(ChoiceType::COLOR);
        let color_1 = color_factory.get_color(ColorType::RED);
        color_1.unwrap().fill();
        let color_2 = color_factory.get_color(ColorType::BLUE);
        color_2.unwrap().fill();
        let color_3 = color_factory.get_color(ColorType::GREEN);
        color_3.unwrap().fill();
    }
}

