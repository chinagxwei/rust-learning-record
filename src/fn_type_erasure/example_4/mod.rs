use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use async_trait::async_trait;


trait Handler<Arg, Res>: Send
{
    type Res: Future<Output=Res> + Send;
    fn call(&self, arg: Arg) -> Self::Res;
}

#[async_trait]
trait Service {
    async fn handle(&self, request: &Request) -> Response;
}


impl<F, Fut> Handler<(), ()> for F
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output=()> + Send
{
    type Res = Fut;

    fn call(&self, _: ()) -> Self::Res {
        (self)()
    }
}


impl<F, Arg, Fut, Res> Handler<(Arg, ), Res> for F
    where
        Arg: Send + 'static,
        F: Fn(Arg) -> Fut + Send + Sync + 'static,
        Fut: Future<Output=Res> + Send
{
    type Res = Fut;

    fn call(&self, arg: (Arg, )) -> Self::Res {
        (self)(arg.0)
    }
}


impl<F, Arg1, Arg2, Fut, Res> Handler<(Arg1, Arg2), Res> for F
    where
        Arg1: Send + 'static,
        Arg2: Send + 'static,
        F: Fn(Arg1, Arg2) -> Fut + Send + Sync + 'static,
        Fut: Future<Output=Res> + Send
{
    type Res = Fut;

    fn call(&self, arg: (Arg1, Arg2)) -> Self::Res {
        (self)(arg.0, arg.1)
    }
}


impl<F, Arg1, Arg2, Arg3, Fut, Res> Handler<(Arg1, Arg2, Arg3), Res> for F
    where
        Arg1: Send + 'static,
        Arg2: Send + 'static,
        Arg3: Send + 'static,
        F: Fn(Arg1, Arg2, Arg3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output=Res> + Send
{
    type Res = Fut;

    fn call(&self, arg: (Arg1, Arg2, Arg3)) -> Self::Res {
        (self)(arg.0, arg.1, arg.2)
    }
}

#[derive(Debug)]
struct FunctionWrapper<F, Arg, Res>(F, PhantomData<Arg>, PhantomData<Res>);

impl<F, Arg, Res> FunctionWrapper<F, Arg, Res>
    where
        F: Handler<Arg, Res>
{
    pub fn new(f: F) -> Self {
        Self(f, PhantomData, PhantomData)
    }
}

#[async_trait]
impl<F, Arg, Res> Service for FunctionWrapper<F, Arg, Res>
    where
        F: Handler<Arg, Res> + Sync + Send,
        Arg: FromRequest + Debug + 'static + Sync + Send,
        Res: IntoResponse + Debug + 'static + Sync + Send
{
    async fn handle(&self, request: &Request) -> Response {
        let arg = Arg::from_request(request);
        println!("arg: {:?}", arg);
        let res: Res = self.0.call(arg).await;
        res.into_response()
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

struct Server {
    services: Vec<Box<dyn Service>>,
}


impl Server {
    pub fn new() -> Self {
        Server { services: vec![] }
    }

    pub fn service<F, Arg, Res>(mut self, service: F) -> Server
        where
            F: Handler<Arg, Res> + Sync + Send + 'static,
            Arg: FromRequest + Debug + 'static + Sync + Send,
            Res: IntoResponse + Debug + 'static + Sync + Send
    {
        let fw = FunctionWrapper::new(service);
        self.services.push(Box::new(fw));
        self
    }

    pub async fn start(&self, request: Request) {
        for service in self.services.iter() {
            let res = service.handle(&request).await;
            println!("{:?}", res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    #[async_trait]
    pub trait Handler2 {
        async fn f0();
        async fn f1(str: String) -> String;
        async fn f2(number: u32) -> String;
        async fn f3(number: String, str: String) -> (String, String);
    }

    struct HandlerImpl2;

    #[async_trait]
    impl Handler2 for HandlerImpl2{
        async fn f0() {
            println!("fun f0")
        }

        async fn f1(str: String) -> String {
            println!("{}", str);
            str
        }

        async fn f2(number: u32) -> String {
            number.to_string()
        }

        async fn f3(number: String, str: String) -> (String, String) {
            (number, str)
        }
    }

    #[tokio::test]
    async fn test() {
        let server = Server::new()
            .service(HandlerImpl2::f0)
            .service(HandlerImpl2::f1)
            .service(HandlerImpl2::f2)
            .service(HandlerImpl2::f3);
        server.start(Request::new("8888")).await;
    }
}

