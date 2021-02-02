//!
//! 在代理模式（Proxy Pattern）中，一个类代表另一个类的功能。这种类型的设计模式属于结构型模式。
//!
//! 在代理模式中，我们创建具有现有对象的对象，以便向外界提供功能接口。
//!

trait Subject {
    fn request(&self);
}

struct RealSubject;

impl RealSubject {
    fn new() -> RealSubject {
        RealSubject
    }
}

impl Subject for RealSubject {
    fn request(&self) {
        println!("RealSubject: Handling request.");
    }
}

struct Proxy<'a> {
    real_subject: &'a RealSubject
}

impl<'a> Proxy<'a> {
    fn new(real_subject: &'a RealSubject) -> Proxy<'a> {
        Proxy { real_subject }
    }
}

impl Proxy<'_> {
    fn check_access(&self) -> bool {
        println!("Proxy: Checking access prior to firing a real request.");
        true
    }

    fn log_access(&self) {
        println!("Proxy: Logging the time of request.")
    }
}

impl Subject for Proxy<'_> {
    fn request(&self) {
        if self.check_access() {
            self.real_subject.request();
            self.log_access();
        }
    }
}

fn client_code<'a>(subject: Box<&'a dyn Subject>) {
    subject.request();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy() {
        println!("Client: Executing the client code with a real subject:");
        let real_subject = RealSubject::new();
        client_code(Box::new(&real_subject));

        println!(" ");
        println!("Client: Executing the same client code with a proxy:");
        let proxy = Proxy::new(&real_subject);
        client_code(Box::new(&proxy));
    }
}
