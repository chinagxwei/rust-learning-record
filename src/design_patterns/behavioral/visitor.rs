//!
//! 在访问者模式（Visitor Pattern）中，我们使用了一个访问者类，它改变了元素类的执行算法。
//! 通过这种方式，元素的执行算法可以随着访问者改变而改变。
//! 这种类型的设计模式属于行为型模式。
//! 根据模式，元素对象已接受访问者对象，这样访问者对象就可以处理元素对象上的操作。
//!

use std::fmt::{Display, Formatter};

enum ComputerPartType {
    KEYBOARD,
    MOUSE,
    MONITOR,
    COMPUTER,
}

impl Display for ComputerPartType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ComputerPartType {
    fn as_str(&self) -> &'static str {
        match *self {
            ComputerPartType::KEYBOARD => "Keyboard",
            ComputerPartType::MOUSE => "Mouse",
            ComputerPartType::MONITOR => "Monitor",
            ComputerPartType::COMPUTER => "Computer",
        }
    }
}

trait ComputerPartVisitor {
    fn visit<'part_lifetime>(&self, part: Box<&'part_lifetime dyn ComputerPart>);
}

trait ComputerPart {
    fn get_type(&self) -> ComputerPartType;
    fn accept<'visitor_lifetime>(&self, computer_part_visitor: Box<&'visitor_lifetime dyn ComputerPartVisitor>);
}

struct Keyboard;

impl Keyboard {
    fn new() -> Keyboard {
        Keyboard
    }
}

impl ComputerPart for Keyboard {
    fn get_type(&self) -> ComputerPartType {
        ComputerPartType::KEYBOARD
    }
    fn accept<'visitor_lifetime>(&self, computer_part_visitor: Box<&'visitor_lifetime dyn ComputerPartVisitor>) {
        computer_part_visitor.visit(Box::new(self));
    }
}

struct Mouse;

impl Mouse {
    fn new() -> Mouse {
        Mouse
    }
}

impl ComputerPart for Mouse {
    fn get_type(&self) -> ComputerPartType {
        ComputerPartType::MOUSE
    }
    fn accept<'visitor_lifetime>(&self, computer_part_visitor: Box<&'visitor_lifetime dyn ComputerPartVisitor>) {
        computer_part_visitor.visit(Box::new(self));
    }
}

struct Monitor;

impl Monitor {
    fn new() -> Monitor {
        Monitor
    }
}

impl ComputerPart for Monitor {
    fn get_type(&self) -> ComputerPartType {
        ComputerPartType::MONITOR
    }
    fn accept<'visitor_lifetime>(&self, computer_part_visitor: Box<&'visitor_lifetime dyn ComputerPartVisitor>) {
        computer_part_visitor.visit(Box::new(self));
    }
}

struct Computer {
    parts: Vec<Box<dyn ComputerPart>>
}

impl Computer {
    fn new() -> Computer {
        Computer {
            parts: vec![
                Box::new(Keyboard::new()),
                Box::new(Mouse::new()),
                Box::new(Monitor::new())
            ]
        }
    }
}

impl ComputerPart for Computer {
    fn get_type(&self) -> ComputerPartType {
        ComputerPartType::COMPUTER
    }

    fn accept<'visitor_lifetime>(&self, computer_part_visitor: Box<&'visitor_lifetime dyn ComputerPartVisitor>) {
        for part in self.parts.iter() {
            part.accept(Box::new(*computer_part_visitor))
        }
        computer_part_visitor.visit(Box::new(self));
    }
}

struct ComputerPartDisplayVisitor;

impl ComputerPartDisplayVisitor {
    fn new() -> ComputerPartDisplayVisitor {
        ComputerPartDisplayVisitor
    }
}

impl ComputerPartVisitor for ComputerPartDisplayVisitor {
    fn visit<'part_lifetime>(&self, part: Box<&'part_lifetime dyn ComputerPart>) {
        println!("Displaying {}.", part.get_type());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor() {
        let computer = Computer::new();
        let computer_part_display_visitor = ComputerPartDisplayVisitor::new();
        computer.accept(Box::new(&computer_part_display_visitor));
    }
}
