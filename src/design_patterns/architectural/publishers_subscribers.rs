//!
//! 发布-订阅是一种消息范式，消息的发送者（称为发布者）不会将消息直接发送给特定的接收者（称为订阅者）。
//! 而是将发布的消息分为不同的类别，无需了解哪些订阅者（如果有的话）可能存在。
//! 同样的，订阅者可以表达对一个或多个类别的兴趣，只接收感兴趣的消息，无需了解哪些发布者（如果有的话）存在。
//!

use std::collections::HashMap;

///
/// 源结构体
///
struct Source<'a>(&'a str, &'a str);

///
/// 1号结构体
/// 不带键值的 订阅结构体
/// 回调 fn 参数是两个字符串引用
///
struct Weather<F> where F: Fn(&str, &str) {
    list: Vec<Box<F>>
}

///
/// 2号结构体
/// 带键值的 订阅结构体
/// 回调 fn 参数是两个字符串引用
///
struct Weather2<'a, F> where F: Fn(&str, &str) {
    list: HashMap<&'a str, Box<F>>
}

///
/// 3号结构体
/// 带键值的 订阅结构体
/// 回调 fn 参数是一个 源结构体
///
struct Weather3<'a, F> where F: Fn(Source) -> bool {
    list: HashMap<&'a str, Box<F>>
}

///
/// 1号结构体实例化
///
impl<F: Fn(&str, &str)> Weather<F> {
    fn listen(&mut self, f: F) {
        self.list.push(Box::new(f));
    }

    fn publish(&self, weather: &str, wind: &str) {
        &self.list
            .iter()
            .for_each(|fun| fun(weather, wind));
    }
}

///
/// 2号结构体实例化
///
impl<'a, F: Fn(&str, &str)> Weather2<'a, F> {
    fn listen(&mut self, k: &'a str, f: F) {
        self.list.insert(k, Box::new(f));
    }

    fn unlisten(&mut self, k: &'a str) -> bool {
        if let Some(_v) = self.list.remove(k) {
            true
        } else {
            false
        }
    }

    fn publish(&self, k: &str, weather: &str, wind: &str) -> bool {
        if let Some(f) = self.list.get(k) {
            f(weather, wind);
            true
        } else {
            false
        }
    }
}

///
/// 3号结构体实例化
///
impl<'a, F: Fn(Source) -> bool> Weather3<'a, F> {
    fn listen(&mut self, k: &'a str, f: F) {
        self.list.insert(k, Box::new(f));
    }

    fn unlisten(&mut self, k: &'a str) -> bool {
        if let Some(_v) = self.list.remove(k) {
            true
        } else {
            false
        }
    }

    fn publish(&self, k: &str, s: Source) -> bool {
        if let Some(f) = self.list.get(k) {
            f(s)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribers() {
        ///
        /// 1号实体案例调用
        ///

        let mut w = Weather { list: vec![] };
        let ww: &mut Weather<fn(&str, &str)> = &mut w;
        ww.listen(|a, b| {
            println!("天气: {} ,风力: {}", a, b);
        });

        ww.publish("晴天", "微风");
        ww.publish("雷阵雨", "5级风");

        ///
        /// 2号实体案例调用
        ///

        let mut w2 = Weather2 { list: HashMap::new() };
        let ww2: &mut Weather2<fn(&str, &str)> = &mut w2;

        ww2.listen("天气", |a, b| {
            println!("天气: {} ,风力: {}", a, b);
        });

        ww2.publish("天气", "多云转晴", "东南风3级");

        ///
        /// 3号实体案例调用
        ///

        let mut w3 = Weather3 { list: HashMap::new() };
        let ww3: &mut Weather3<fn(Source) -> bool> = &mut w3;

        ww3.listen("Weather", |s| -> bool {
            println!("天气: {} ,风力: {}", s.0, s.1);
            true
        });

        ww3.publish("Weather", Source("台风", "狂风6级"));
    }
}
