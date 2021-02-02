//!
//! 拦截过滤器模式（Intercepting Filter Pattern）用于对应用程序的请求或响应做一些预处理/后处理。
//! 定义过滤器，并在把请求传给实际目标应用程序之前应用在请求上。
//! 过滤器可以做认证/授权/记录日志，或者跟踪请求，然后把请求传给相应的处理程序。
//! 以下是这种设计模式的实体。
//!
//! 过滤器（Filter） - 过滤器在请求处理程序执行请求之前或之后，执行某些任务。
//! 过滤器链（Filter Chain） - 过滤器链带有多个过滤器，并在 Target 上按照定义的顺序执行这些过滤器。
//! Target - Target 对象是请求处理程序。
//! 过滤管理器（Filter Manager） - 过滤管理器管理过滤器和过滤器链。
//! 客户端（Client） - Client 是向 Target 对象发送请求的对象。
//!

trait Filter {
    fn execute(&self, request: &String) -> bool;
}

struct AuthenticationFilter;

struct DebugFilter;

struct Target;

impl AuthenticationFilter {
    fn new() -> AuthenticationFilter {
        AuthenticationFilter
    }
}

impl Filter for AuthenticationFilter {
    fn execute(&self, request: &String) -> bool {
        println!("Authenticating request: {}", request);
        request == &(String::from("HOME"))
    }
}

impl DebugFilter {
    fn new() -> DebugFilter {
        DebugFilter
    }
}

impl Filter for DebugFilter {
    fn execute(&self, request: &String) -> bool {
        println!("Request log: {}", request);
        true
    }
}

impl Target {
    fn new() -> Target {
        Target
    }
}

impl Target {
    fn execute(&self, request: String) {
        println!("Executing request: {}", request)
    }
}

struct FilterChain {
    target: Option<Target>,
    filters: Vec<Box<dyn Filter>>,
}

impl FilterChain {
    fn new() -> FilterChain {
        FilterChain { target: None, filters: vec![] }
    }
}

impl FilterChain {
    fn add(&mut self, filter: Box<dyn Filter>) {
        self.filters.push(filter)
    }

    fn set_target(&mut self, target: Target) {
        self.target = Some(target)
    }

    fn execute(&self, request: String) -> bool {
        for filter in self.filters.iter() {
            if !filter.execute(&request) {
                return false;
            }
        }
        self.target.as_ref().unwrap().execute(request);
        true
    }
}

struct FilterManager {
    filter_chain: FilterChain
}

impl FilterManager {
    fn new(target: Target) -> FilterManager {
        let mut chain = FilterChain::new();
        chain.set_target(target);
        FilterManager { filter_chain: chain }
    }
}

impl FilterManager {
    fn set_filter(&mut self, filter: Box<dyn Filter>) {
        self.filter_chain.add(filter)
    }

    fn filter_request(&self, request: String) -> bool {
        self.filter_chain.execute(request)
    }
}

struct Client {
    filter_manager: Option<FilterManager>
}

impl Client {
    fn new() -> Client {
        Client { filter_manager: None }
    }
}

impl Client {
    fn set_filter_manager(&mut self, filter_manager: FilterManager) {
        self.filter_manager = Some(filter_manager)
    }

    fn send_request(&self, request: String) -> bool {
        self.filter_manager.as_ref().unwrap().filter_request(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        let mut filter_manager = FilterManager::new(Target::new());
        filter_manager.set_filter(Box::new(DebugFilter::new()));
        filter_manager.set_filter(Box::new(AuthenticationFilter::new()));

        let mut client = Client::new();
        client.set_filter_manager(filter_manager);
        let res = client.send_request(String::from("HOME"));
        assert_eq!(res, true);
        println!();
        let res = client.send_request(String::from("ABOUT"));
        assert_eq!(res, false)
    }
}


