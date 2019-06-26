
///
/// 通过工厂生产水果
///

///
/// 水果类型
///
pub enum Fruit {
    Apple,
    Banana,
}

///
/// 水果特性
///
pub trait FruitTrait {
    fn eat(&self);
    fn action(&self, origin: &String, kind: &Fruit) {
        println!("产地 {} 的 {} 可以吃！", origin, kind.as_str())
    }
}

///
/// 苹果结构
///
pub struct Apple {
    origin: String,
    kind: Fruit,
}

///
/// 香蕉结构
///
pub struct Banana {
    origin: String,
    kind: Fruit,
}

///
/// 类型实现
///
impl Fruit {
    fn as_str(&self) -> &'static str {
        match *self {
            Fruit::Apple => "苹果",
            Fruit::Banana => "香蕉",
        }
    }
}

///
/// 苹果实现
///
impl Apple {
    pub fn new(origin: String, kind: Fruit) -> Apple {
        Apple { origin, kind }
    }
}

///
/// 苹果的水果特性实现
///
impl FruitTrait for Apple {
    fn eat(&self) {
        self.action(&self.origin, &self.kind);
    }
}

///
/// 香蕉实现
///
impl Banana {
    pub fn new(origin: String, kind: Fruit) -> Banana {
        Banana { origin, kind }
    }
}

///
/// 香蕉的水果特性实现
///
impl FruitTrait for Banana {
    fn eat(&self) {
        self.action(&self.origin, &self.kind);
    }
}

///
/// 工厂结构
///
struct Factory;

///
/// 工厂实现
///
impl Factory {
    pub fn get_instance(kind: Fruit) -> Box<FruitTrait> {
        match kind {
            Fruit::Apple => Box::new(Apple::new("美国".to_string(), Fruit::Apple)),
            Fruit::Banana => Box::new(Banana::new("泰国".to_string(), Fruit::Banana)),
        }
    }
}


fn main() {

    ///
    /// 创建一个苹果实例
    ///

    let apple: Box<FruitTrait> = Factory::get_instance(Fruit::Apple);
//
    apple.eat();

    ///
    /// 创建一个香蕉实例
    ///

    let banana: Box<FruitTrait> = Factory::get_instance(Fruit::Banana);

    banana.eat();

}
