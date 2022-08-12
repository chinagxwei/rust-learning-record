use std::fmt::Debug;
use std::marker::PhantomData;

trait Handler<Arg, Output>: 'static {
    fn call(&self, arg: Arg) -> Output;
}

impl<F> Handler<(), ()> for F where F: Fn() + 'static {
    fn call(&self, _: ()) -> () {
        (self)()
    }
}

impl<F, Arg, Output> Handler<(Arg, ), Output> for F
    where
        F: Fn(Arg) -> Output + 'static
{
    fn call(&self, arg: (Arg, )) -> Output {
        (self)(arg.0)
    }
}

impl<F, Arg1, Arg2, Output> Handler<(Arg1, Arg2), Output> for F
    where
        F: Fn(Arg1, Arg2) -> Output + 'static
{
    fn call(&self, arg: (Arg1, Arg2)) -> Output {
        (self)(arg.0, arg.1)
    }
}

impl<F, Arg1, Arg2, Arg3, Output> Handler<(Arg1, Arg2, Arg3), Output> for F
    where
        F: Fn(Arg1, Arg2, Arg3) -> Output + 'static
{
    fn call(&self, arg: (Arg1, Arg2, Arg3)) -> Output {
        (self)(arg.0, arg.1, arg.2)
    }
}

#[derive(Debug)]
struct FunctionWrapper<F, Arg, Output>(F, PhantomData<Arg>, PhantomData<Output>);

impl<F, Arg, Output> FunctionWrapper<F, Arg, Output>
    where
        F: Handler<Arg, Output>
{
    pub fn new(f: F) -> Self {
        Self(f, PhantomData, PhantomData)
    }
}

struct Request {
    data: String,
}

impl Request {
    pub fn new(data: impl Into<String>) -> Self {
        Request { data: data.into() }
    }
}

#[derive(Debug)]
struct Response {
    data: String,
}

trait Service {
    fn handle(&self, request: &Request) -> Response;
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

trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for () {
    fn into_response(self) -> Response {
        Response { data: "".to_string() }
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response { data: self }
    }
}

impl IntoResponse for u32 {
    fn into_response(self) -> Response {
        Response { data: self.to_string() }
    }
}

impl<T1, T2> IntoResponse for (T1, T2)
    where
        T1: Into<String>,
        T2: Into<String>
{
    fn into_response(self) -> Response {
        Response { data: format!("{},{}", self.0.into(), self.1.into()) }
    }
}

impl<F, Arg, Output> Service for FunctionWrapper<F, Arg, Output>
    where
        F: Handler<Arg, Output>,
        Arg: FromRequest + Debug + 'static,
        Output: IntoResponse + Debug + 'static
{
    fn handle(&self, request: &Request) -> Response {
        let arg = Arg::from_request(request);
        println!("arg: {:?}", arg);
        self.0.call(arg).into_response()
    }
}

struct Server {
    services: Vec<Box<dyn Service>>,
}

impl Server {
    pub fn new() -> Self {
        Server { services: vec![] }
    }

    pub fn service<F, Arg, Output>(mut self, service: F) -> Server
        where
            F: Handler<Arg, Output>,
            Arg: FromRequest + Debug + 'static,
            Output: IntoResponse + Debug + 'static
    {
        let fw = FunctionWrapper::new(service);
        self.services.push(Box::new(fw));
        self
    }

    pub fn start(&self, request: Request) {
        for service in self.services.iter() {
            let res = service.handle(&request);
            println!("{:?}", res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub trait Handler2 {
        fn f0();
        fn f1(str: String) -> String;
        fn f2(number: u32) -> String;
        fn f3(number: String, str: String) -> (String, String);
    }

    struct HandlerImpl2;

    impl Handler2 for HandlerImpl2 {
        fn f0() { println!("fun f0") }
        fn f1(str: String) -> String {
            println!("{}", str);
            str
        }
        fn f2(number: u32) -> String {
            number.to_string()
        }
        fn f3(number: String, str: String) -> (String, String) {
            (number, str)
        }
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


