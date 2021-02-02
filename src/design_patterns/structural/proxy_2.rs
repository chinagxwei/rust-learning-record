//!
//! 在代理模式（Proxy Pattern）中，一个类代表另一个类的功能。这种类型的设计模式属于结构型模式。
//!
//! 在代理模式中，我们创建具有现有对象的对象，以便向外界提供功能接口。
//!

///
/// 通过代理帮助男孩购买饼干
///

///
/// 饼干结构
///
struct Cookie {
    name: String
}

///
/// 男孩结构
///
struct Boy {
    name: String
}

///
/// 一般代理结构
///
struct Proxy {
    boy: Boy
}

///
/// 虚拟代理结构
///
struct Proxy2<'a> {
    boy: Option<&'a Boy>,
}

///
/// 男孩实例
///
impl Boy {
    fn new(s: String) -> Boy {
        Boy { name: s }
    }

    fn buy_cookie(&self, name: String) {
        println!("{} 买了 {} 饼干", self.name, name);
    }
}

///
/// 一般代理
///
impl Proxy {
    fn new(b: Boy) -> Proxy {
        Proxy { boy: b }
    }

    fn buy_cookie(&self, c: Cookie) {
        self.boy.buy_cookie(c.name)
    }
}

///
/// 虚拟代理
///
impl<'a> Proxy2<'a> {
    fn new() -> Proxy2<'a> {
        Proxy2 { boy: None }
    }

    fn init(&mut self, b: &'a Boy) -> &mut Proxy2<'a> {
        if let None = self.boy {
            self.boy = Some(b)
        }
        self
    }

    fn buy_cookie(&self, c: Cookie) {
        if let Some(b) = self.boy {
            b.buy_cookie(c.name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy() {
        let b: Boy = Boy::new("小明".to_string());
        let p: Proxy = Proxy::new(b);

        p.buy_cookie(Cookie { name: "曲奇".to_string() });

        let mut p2: Proxy2 = Proxy2::new();
        let b2: Boy = Boy::new("小张".to_string());
        p2.init(&b2).buy_cookie(Cookie { name: "黄油".to_string() });
    }
}
