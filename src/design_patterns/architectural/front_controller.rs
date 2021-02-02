//!
//! 前端控制器模式（Front Controller Pattern）是用来提供一个集中的请求处理机制，所有的请求都将由一个单一的处理程序处理。
//! 该处理程序可以做认证/授权/记录日志，或者跟踪请求，然后把请求传给相应的处理程序。
//! 以下是这种设计模式的实体。
//!
//! 前端控制器（Front Controller） - 处理应用程序所有类型请求的单个处理程序，应用程序可以是基于 web 的应用程序，也可以是基于桌面的应用程序。
//! 调度器（Dispatcher） - 前端控制器可能使用一个调度器对象来调度请求到相应的具体处理程序。
//! 视图（View） - 视图是为请求而创建的对象。
//!

use std::fmt::{Display, Formatter};

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum ViewType {
    Home,
    Student,
}

impl ViewType {
    fn as_str(&self) -> &'static str {
        match *self {
            ViewType::Home => "Home",
            ViewType::Student => "Student"
        }
    }
}

impl Display for ViewType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

struct HomeView;

impl HomeView {
    fn new() -> HomeView {
        HomeView
    }
}

impl HomeView {
    fn show(&self) {
        println!("Displaying Home Page")
    }
}

struct StudentView;

impl StudentView {
    fn new() -> StudentView {
        StudentView
    }
}

impl StudentView {
    fn show(&self) {
        println!("Displaying Student Page")
    }
}

struct Dispatcher {
    home_view: HomeView,
    student_view: StudentView,
}

impl Dispatcher {
    fn new() -> Dispatcher {
        Dispatcher { home_view: HomeView::new(), student_view: StudentView::new() }
    }
}

impl Dispatcher {
    fn dispatcher(&self, request: ViewType) {
        match request {
            ViewType::Home => self.home_view.show(),
            ViewType::Student => self.student_view.show()
        }
    }
}

struct FrontController {
    dispatcher: Dispatcher
}

impl FrontController {
    fn new() -> FrontController {
        FrontController { dispatcher: Dispatcher::new() }
    }
}

impl FrontController {
    fn is_authentic_user(&self) -> bool {
        println!("User is authenticated successfully.");
        true
    }

    fn track_request(&self, request: ViewType) {
        println!("Page requested: {}", request)
    }

    fn dispatcher_request(&self, request: ViewType) {
        self.track_request(request);
        if self.is_authentic_user() {
            self.dispatcher.dispatcher(request)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_front_controller() {
        let front_controller = FrontController::new();
        front_controller.dispatcher_request(ViewType::Home);
        front_controller.dispatcher_request(ViewType::Student);
    }
}
