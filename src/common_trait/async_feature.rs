use std::future::Future;
use async_trait::async_trait;

async fn fn_point(string: &String) -> String {
    string.into()
}

#[async_trait]
trait App: Sync {
    async fn call(&self, value: &String) -> String;
}

trait Wrapper<'str_lifetime>: Send + Sync {
    type Res: Future<Output=String> + Send + Sync;
    fn wrapped_call(&self, s: &'str_lifetime String) -> Self::Res;
}

impl<'str_lifetime, F, Fut> Wrapper<'str_lifetime> for F
    where
        F: Send + Sync + Fn(&'str_lifetime String) -> Fut,
        Fut: Send + Sync + Future<Output=String>,
{
    type Res = Fut;
    fn wrapped_call(&self, s: &'str_lifetime String) -> Self::Res {
        self(s)
    }
}

#[async_trait]
impl<T> App for T
    where
        T: for<'str_lifetime> Wrapper<'str_lifetime>,
{
    async fn call(&self, value: &String) -> String {
        self.wrapped_call(value).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() {
        let dyn_test: &dyn App = &fn_point;
        let string = String::from("233");
        let s = dyn_test.call(&string).await;
        assert_eq!(String::from("233"), s);
    }
}
