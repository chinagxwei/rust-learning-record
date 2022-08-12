///
/// example from https://rustcc.cn/article?id=4489fae2-7dfc-4543-a61a-7c2d113735ee
///

use std::marker::PhantomData;
use std::fmt::Debug;

trait Service {
    fn handle(&self, request: &Request);
}

struct Server {
    services: Vec<Box<dyn Service>>
}

impl Server {
    pub fn new() -> Self {
        Server { services: vec![] }
    }

    pub fn service<F, Arg>(mut self, service: F) -> Server
        where
            F: Handler<Arg>,
            Arg: FromRequest + Debug + 'static
    {
        let fw = FunctionWrapper::new(service);
        self.services.push(Box::new(fw));
        self
    }

    pub fn start(&self, request: Request) {
        for service in self.services.iter() {
            service.handle(&request);
        }
    }
}

///
/// 通过虚类型参数 PhantomData 区分 函数指针类型 ,
/// 虚函数类型参数描述了函数指针的参数个数
///
///
#[derive(Debug)]
struct FunctionWrapper<F, Arg>(F, PhantomData<Arg>);


impl<F, Arg> FunctionWrapper<F, Arg>
    where
        F: Handler<Arg>
{
    pub fn new(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<F, Arg> Service for FunctionWrapper<F, Arg>
    where
        F: Handler<Arg>,
        Arg: FromRequest + Debug + 'static
{
    fn handle(&self, request: &Request) {
        let arg = Arg::from_request(&request);
        println!("arg: {:?}", arg);
        self.0.call(arg)
    }
}

trait Handler<Arg>: 'static {
    fn call(&self, arg: Arg);
}

///
/// 无参数时函数指针执行
///
/// PhantomData<()>
///
impl<F> Handler<()> for F where F: Fn() -> () + 'static {
    fn call(&self, _: ()) {
        (self)()
    }
}

///
/// 一个参数时函数指针执行
///
/// PhantomData<(T1,)>
///
impl<F, Arg> Handler<(Arg, )> for F
    where
        F: Fn(Arg) -> () + 'static
{
    fn call(&self, arg: (Arg, )) {
        (self)(arg.0)
    }
}

///
/// 两个参数时函数指针执行
///
/// PhantomData<(T1,T2)>
///
impl<F, Arg1, Arg2> Handler<(Arg1, Arg2)> for F
    where
        F: Fn(Arg1, Arg2) -> () + 'static
{
    fn call(&self, arg: (Arg1, Arg2)) {
        (self)(arg.0, arg.1)
    }
}

///
/// 两个参数时函数指针执行
///
/// PhantomData<(T1, T2, T3)>
///
impl<F, Arg1, Arg2, Arg3> Handler<(Arg1, Arg2, Arg3)> for F
    where
        F: Fn(Arg1, Arg2, Arg3) -> () + 'static
{
    fn call(&self, arg: (Arg1, Arg2, Arg3)) {
        println!("call three arg fun");
        (self)(arg.0, arg.1, arg.2)
    }
}

struct Request {
    data: String
}

impl Request {
    pub fn new(data: impl Into<String>) -> Self {
        Request { data: data.into() }
    }
}

trait FromRequest {
    fn from_request(request: &Request) -> Self;
}

impl FromRequest for String {
    fn from_request(request: &Request) -> Self {
        request.data.clone()
    }
}

impl FromRequest for u32 {
    fn from_request(request: &Request) -> Self {
        request.data.parse().unwrap()
    }
}

impl FromRequest for () {
    fn from_request(_: &Request) -> Self {
        ()
    }
}

impl<T1> FromRequest for (T1, )
    where
        T1: FromRequest
{
    fn from_request(request: &Request) -> Self {
        (T1::from_request(request), )
    }
}

impl<T1, T2> FromRequest for (T1, T2)
    where
        T1: FromRequest,
        T2: FromRequest
{
    fn from_request(request: &Request) -> Self {
        (T1::from_request(request), T2::from_request(request))
    }
}

impl<T1, T2, T3> FromRequest for (T1, T2, T3)
    where
        T1: FromRequest,
        T2: FromRequest,
        T3: FromRequest
{
    fn from_request(request: &Request) -> Self {
        (T1::from_request(request), T2::from_request(request), T3::from_request(request))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    pub trait Handler2 {
        fn f0();
        fn f1(str: String);
        fn f2(number: u32, str: String);
        fn f3(status: (), number: u32, str: String);
    }

    struct HandlerImpl2;

    impl Handler2 for HandlerImpl2 {
        fn f0() { println!("fun f0") }
        fn f1(str: String) { println!("{}", str) }
        fn f2(number: u32, str: String) {}
        fn f3(status: (), number: u32, str: String) {}
    }

    #[test]
    fn test() {
        let server = Server::new()
            .service(HandlerImpl2::f0)
            .service(HandlerImpl2::f1)
            .service(HandlerImpl2::f2)
            .service(HandlerImpl2::f3);
        server.start(Request::new("666"));
    }
}
