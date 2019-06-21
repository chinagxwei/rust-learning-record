use std::collections::HashMap;

struct Source<'a>(&'a str, &'a str);

///
/// 不带键值的 订阅结构体
/// 回调 fn 参数是两个字符串引用
///
struct Weather<F> where F: Fn(&str, &str) {
    list: Vec<F>
}

///
/// 带键值的 订阅结构体
/// 回调 fn 参数是两个字符串引用
///
struct Weather2<'a, F> where F: Fn(&str, &str) {
    list: HashMap<&'a str, F>
}

///
/// 带键值的 订阅结构体
/// 回调 fn 参数是一个 源结构体
///
struct Weather3<'a, F> where F: Fn(Source) {
    list: HashMap<&'a str, F>
}

impl<F: Fn(&str, &str)> Weather<F> {
    fn listen(&mut self, f: F) {
        self.list.push(f);
    }

    fn publish(&self, weather: &str, wind: &str) {
        for fun in &self.list {
            fun(weather, wind);
        }
    }
}

impl<'a, F: Fn(&str, &str)> Weather2<'a, F> {
    fn listen(&mut self, k: &'a str, f: F) {
        self.list.insert(k, f);
    }

    fn publish(&self, k: &str, weather: &str, wind: &str) {
        match self.list.get(k) {
            Some(f) => {
                f(weather, wind)
            }
            _ => {}
        }
    }
}

impl<'a, F: Fn(Source)> Weather3<'a, F> {
    fn listen(&mut self, k: &'a str, f: F) {
        self.list.insert(k, f);
    }

    fn publish(&self, k: &str, s: Source) {
        match self.list.get(k) {
            Some(f) => {
                f(s)
            }
            _ => {}
        }
    }
}


fn main() {
    let mut w = Weather { list: vec![] };
    let ww: &mut Weather<fn(&str, &str)> = &mut w;
    ww.listen(|a, b| {
        println!("天气: {} ,风力: {}", a, b);
    });

    ww.publish("晴天", "微风");
    ww.publish("雷阵雨", "5级风");

    let mut w2 = Weather2 { list: HashMap::new() };

    let ww2: &mut Weather2<fn(&str, &str)> = &mut w2;

    ww2.listen("天气", |a, b| {
        println!("天气: {} ,风力: {}", a, b);
    });

    ww2.publish("天气", "多云转晴", "东南风3级");

    let mut w3 = Weather3 { list: HashMap::new() };

    let ww3: &mut Weather3<fn(Source)> = &mut w3;

    ww3.listen("Weather", |s| {
        println!("天气: {} ,风力: {}", s.0, s.1);
    });

    ww3.publish("Weather", Source("台风", "狂风6级"));
}
