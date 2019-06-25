///
/// 通过代理帮助男孩购买饼干
///

struct Cookie {
    name: String
}

struct Boy {
    name: String
}

struct Proxy<'a> {
    boy: &'a Boy
}

struct Proxy2<'a> {
    boy: Option<&'a Boy>,
}

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
impl<'a> Proxy<'a> {
    fn new(b: &'a Boy) -> Proxy {
        Proxy { boy: b }
    }

    fn buy_cookie(&self, c: Cookie) {
        *&self.boy.buy_cookie(c.name)
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
        if let None = &self.boy {
            self.boy = Some(b)
        }
        self
    }

    fn buy_cookie(&self, c: Cookie) {
        if let Some(b) = &self.boy {
            b.buy_cookie(c.name);
        }
    }
}


fn main() {
//    println!("Hello, world!");
    let b: Boy = Boy::new("小明".to_string());
    let p: Proxy = Proxy::new(&b);

    p.buy_cookie(Cookie { name: "曲奇".to_string() });

    let mut p2: Proxy2 = Proxy2::new();
    let b2: Boy = Boy::new("小张".to_string());
    p2.init(&b2).buy_cookie(Cookie { name: "黄油".to_string() });
}
