//!
//! 在程序设计中, 惰性初始是一种拖延战术。在第一次需求出现以前，先延迟创建物件、计算值或其它昂贵程序。
//!
//! 这通常是以一个旗号来实现，用旗号来标示是否完成其程序。
//! 每次请求对象时，会先测试此旗号。如果已完成，直接传回，否则当场执行。
//!
//! 对于此想法更一般的论述，可见惰性求值。
//!
//! 对指令式语言，这个模式可能潜藏着危险，尤其是使用共享状态的程序习惯。
//!

use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Clone)]
struct Fruit {
    r#type: &'static str
}

impl Fruit {
    fn new(r#type: &'static str) -> Fruit {
        Fruit { r#type }
    }
}

struct Fruits {
    types: RefCell<HashMap<&'static str, Fruit>>,
}

impl Fruits {
    fn new() -> Fruits {
        Fruits { types: RefCell::new(Default::default()) }
    }
}

impl Fruits {
    fn get_fruit(&self, r#type: &'static str) -> Fruit {
        self.types
            .borrow_mut()
            .entry(r#type)
            .or_insert_with(|| {
                println!("Lazy initialization.");
                Fruit::new(r#type)
            })
            .clone()
    }

    fn print(&self) {
        println!("Number of instances made: {}", self.types.borrow().len());
        self.types
            .borrow()
            .iter()
            .for_each(|(k,_)| println!("{}", *k));
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy() {
        let fruits = Fruits::new();
        fruits.get_fruit("Apple");
        fruits.print();
        fruits.get_fruit("Lime");
        fruits.print();
        fruits.get_fruit("Banana");
        fruits.print();
        fruits.get_fruit("Apple");
        fruits.print();
    }
}
