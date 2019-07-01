///
/// 通过工厂生产水果
///

///
/// 水果类型推导
///
pub enum Fruit {
    Apple(Box<Apple>),
    Banana(Box<Banana>),
}

///
/// 水果类型
///
pub enum FruitKind {
    Apple,
    Banana,
}

///
/// 水果特性
///
pub trait FruitTrait {
    fn eat(&self);
    fn action(&self, origin: &String, kind: &FruitKind) {
        println!("产地 {} 的 {} 可以吃！", origin, kind.as_str())
    }
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

///
/// 苹果实现
///
impl Apple {
    pub fn new(origin: String, kind: FruitKind) -> Apple {
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
    pub fn new(origin: String, kind: FruitKind) -> Banana {
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
    pub fn get_instance(kind: FruitKind) -> Fruit {
        match kind {
            FruitKind::Apple => Fruit::Apple(Box::new(Apple::new("美国".to_string(), FruitKind::Apple))),
            FruitKind::Banana => Fruit::Banana(Box::new(Banana::new("泰国".to_string(), FruitKind::Banana))),
        }
    }
}


fn main() {

    if let Fruit::Apple(apple) = Factory::get_instance(FruitKind::Apple){
        apple.eat();
    }


    if let Fruit::Banana(banana) = Factory::get_instance(FruitKind::Banana){
        banana.eat();
    }
}
