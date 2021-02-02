//!
//! 这种类型的设计模式属于创建型模式，它提供了一种创建对象的最佳方式。
//!
//! 在工厂模式中，我们在创建对象时不会对客户端暴露创建逻辑，并且是通过使用一个共同的接口来指向新创建的对象。
//!

///
/// 通过工厂生产水果
///

///
/// 水果类型
///
pub enum FruitKind {
    Apple,
    Banana,
}

///
/// 类型实现
///
impl FruitKind {
    fn as_str(&self) -> &'static str {
        match *self {
            FruitKind::Apple => "苹果",
            FruitKind::Banana => "香蕉",
        }
    }
}

impl std::fmt::Display for FruitKind{
    fn fmt(&self, f: &mut std::fmt::Formatter)  -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

///
/// 水果特性
///
pub trait FruitTrait {
    fn eat(&self);
}


///
/// 苹果结构
///
pub struct Apple {
    origin: String,
    kind: FruitKind,
}

///
/// 香蕉结构
///
pub struct Banana {
    origin: String,
    kind: FruitKind,
}

impl FruitTrait for Banana {
    fn eat(&self) {
        println!("产地 {} 的 {} 可以吃！", self.origin, self.kind)
    }
}

impl Banana {
    fn new(origin: String, kind: FruitKind) -> Banana {
        Banana { origin, kind }
    }
}

impl FruitTrait for Apple {
    fn eat(&self) {
        println!("产地 {} 的 {} 可以吃！", self.origin, self.kind)
    }
}

impl Apple {
    fn new(origin: String, kind: FruitKind) -> Banana {
        Banana { origin, kind }
    }
}

///
/// 工厂结构
///
pub struct Factory;

///
/// 工厂实现
///
impl Factory {
    pub fn get_instance(kind: FruitKind) -> Box<dyn FruitTrait> {
        match kind {
            FruitKind::Apple => Box::new(Apple::new("美国".to_string(), FruitKind::Apple)),
            FruitKind::Banana => Box::new(Banana::new("泰国".to_string(), FruitKind::Banana)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory() {
        let apple= Factory::get_instance(FruitKind::Apple);

        apple.eat();

        let banana = Factory::get_instance(FruitKind::Banana);

        banana.eat();
    }
}

