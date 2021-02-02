//!
//! 抽象工厂模式（Abstract Factory Pattern）是围绕一个超级工厂创建其他工厂。
//! 该超级工厂又称为其他工厂的工厂。
//! 这种类型的设计模式属于创建型模式，它提供了一种创建对象的最佳方式。
//!
//! 在抽象工厂模式中，接口是负责创建一个相关对象的工厂，不需要显式指定它们的类。
//! 每个生成的工厂都能按照工厂模式提供对象。
//!

trait AbstractFactory {
    fn create_product_a(&self) -> Box<dyn AbstractProductA>;
    fn create_product_b(&self) -> Box<dyn AbstractProductB>;
}

trait AbstractProductA {
    fn useful_function_a(&self) -> String;
}

trait AbstractProductB {
    fn useful_function_b(&self) -> String;
    fn another_useful_function_b(&self, collaborator: Box<dyn AbstractProductA>) -> String;
}

struct ConcreteFactory1;

struct ConcreteFactory2;

impl ConcreteFactory1 {
    pub fn new() -> ConcreteFactory1 {
        ConcreteFactory1
    }
}

impl ConcreteFactory2 {
    pub fn new() -> ConcreteFactory2 {
        ConcreteFactory2
    }
}

struct ConcreteProductA1;

struct ConcreteProductA2;

impl ConcreteProductA1 {
    pub fn new() -> ConcreteProductA1 {
        ConcreteProductA1
    }
}

impl ConcreteProductA2 {
    pub fn new() -> ConcreteProductA2 {
        ConcreteProductA2
    }
}

impl AbstractProductA for ConcreteProductA1 {
    fn useful_function_a(&self) -> String {
        String::from("The result of the product A1.")
    }
}

impl AbstractProductA for ConcreteProductA2 {
    fn useful_function_a(&self) -> String {
        String::from("The result of the product A2.")
    }
}

struct ConcreteProductB1;

struct ConcreteProductB2;

impl ConcreteProductB1 {
    pub fn new() -> ConcreteProductB1 {
        ConcreteProductB1
    }
}

impl ConcreteProductB2 {
    pub fn new() -> ConcreteProductB2 {
        ConcreteProductB2
    }
}

impl AbstractProductB for ConcreteProductB1 {
    fn useful_function_b(&self) -> String {
        String::from("The result of the product B1.")
    }

    fn another_useful_function_b(&self, collaborator: Box<dyn AbstractProductA>) -> String {
        let result = collaborator.useful_function_a();
        format!("The result of the B1 collaborating with the ({})", result)
    }
}

impl AbstractProductB for ConcreteProductB2 {
    fn useful_function_b(&self) -> String {
        String::from("The result of the product B2.")
    }

    fn another_useful_function_b(&self, collaborator: Box<dyn AbstractProductA>) -> String {
        let result = collaborator.useful_function_a();
        format!("The result of the Bs collaborating with the ({})", result)
    }
}

impl AbstractFactory for ConcreteFactory1 {
    fn create_product_a(&self) -> Box<dyn AbstractProductA> {
        Box::new(ConcreteProductA1::new())
    }

    fn create_product_b(&self) -> Box<dyn AbstractProductB> {
        Box::new(ConcreteProductB1::new())
    }
}

impl AbstractFactory for ConcreteFactory2 {
    fn create_product_a(&self) -> Box<dyn AbstractProductA> {
        Box::new(ConcreteProductA2::new())
    }

    fn create_product_b(&self) -> Box<dyn AbstractProductB> {
        Box::new(ConcreteProductB2::new())
    }
}


fn client_code(factory: Box<dyn AbstractFactory>) {
    let product_a = factory.create_product_a();
    let product_b = factory.create_product_b();
    println!("{}", product_b.useful_function_b());
    println!("{}", product_b.another_useful_function_b(product_a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory() {
        println!("Client: Testing client code with the first factory type...");
        client_code(Box::new(ConcreteFactory1::new()));
        println!(" ");
        println!("Client: Testing the same client code with the second factory type...");
        client_code(Box::new(ConcreteFactory2::new()));
    }
}

