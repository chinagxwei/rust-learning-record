use std::rc::Rc;

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

struct Proxy {
    real_subject: Rc<RealSubject>
}

impl Proxy {
    fn new(real_subject: Rc<RealSubject>) -> Proxy {
        Proxy { real_subject }
    }
}

impl Proxy {
    fn check_access(&self) -> bool {
        println!("Proxy: Checking access prior to firing a real request.");
        true
    }

    fn log_access(&self) {
        println!("Proxy: Logging the time of request.")
    }
}

impl Subject for Proxy {
    fn request(&self) {
        if self.check_access() {
            self.real_subject.request();
            self.log_access();
        }
    }
}

fn client_code(subject: Rc<dyn Subject>) {
    subject.request();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy() {
        println!("Client: Executing the client code with a real subject:");
        let real_subject = Rc::new(RealSubject::new());
        client_code(real_subject.clone());

        println!(" ");
        println!("Client: Executing the same client code with a proxy:");
        let proxy = Proxy::new(real_subject.clone());
        client_code(Rc::new(proxy));
    }
}
