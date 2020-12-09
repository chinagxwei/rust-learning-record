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
