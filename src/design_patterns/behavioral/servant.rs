//!
//! 雇工模式也叫做仆人模式，其意图是：
//!
//! 雇工模式是行为模式的一种，他为一组类提供通用的功能，而不需要类实现这些功能，他是命令模式的一种扩展。
//!

use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

trait Movable: Display {
    fn get_position(&self) -> Position;
    fn set_position(&mut self, position: Position);
}

struct Triangle {
    position: Position
}

impl Triangle {
    fn new(position: Position) -> Triangle {
        Triangle { position }
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[x: {}, y: {}]", self.position.x, self.position.y)
    }
}

impl Movable for Triangle {
    fn get_position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}

struct Ellipse {
    position: Position
}

impl Ellipse {
    fn new(position: Position) -> Ellipse {
        Ellipse { position }
    }
}

impl Display for Ellipse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[x: {}, y: {}]", self.position.x, self.position.y)
    }
}

impl Movable for Ellipse {
    fn get_position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}

struct MoveServant;

impl MoveServant {
    fn move_to(mut serviced: Box<dyn Movable>, r#where: Position) {
        println!("current position: {}", serviced);
        serviced.set_position(r#where);
        println!("move to position: {}", serviced);
    }

    fn move_by(mut serviced: Box<dyn Movable>, mut x: i32, mut y: i32) {
        println!("current position: {}", serviced);
        x += serviced.get_position().x;
        y += serviced.get_position().y;
        serviced.set_position(Position::new(x, y));
        println!("move to position: {}", serviced);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_servant() {
        let triangle = Triangle::new(Position::new(10, 30));
        let ellipse = Ellipse::new(Position::new(30, 30));

        MoveServant::move_to(Box::new(triangle), Position::new(20, 60));

        MoveServant::move_by(Box::new(ellipse), 15, 15);
    }
}
