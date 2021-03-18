///
/// example from https://rustcc.cn/article?id=4489fae2-7dfc-4543-a61a-7c2d113735ee
///

struct Server {
    services: Vec<Box<dyn Fn() -> () + 'static>>
}

impl Server {
    pub fn new() -> Self {
        Server { services: vec![] }
    }

    pub fn service(mut self, service: impl Fn() -> () + 'static) -> Server {
        self.services.push(Box::new(service));
        self
    }

    pub fn start(&self) {
        for service in self.services.iter() {
            (service)()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub trait Handler {
        fn f1();
        fn f2();
        fn f3();
    }

    struct HandlerImpl;

    impl Handler for HandlerImpl {
        fn f1() {}
        fn f2() {}
        fn f3() {}
    }

    #[test]
    fn test() {
        let server = Server::new()
            .service(HandlerImpl::f1)
            .service(HandlerImpl::f2)
            .service(HandlerImpl::f3);
        server.start();
    }
}
