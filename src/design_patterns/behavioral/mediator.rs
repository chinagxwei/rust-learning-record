//!
//! 中介者模式（Mediator Pattern）是用来降低多个对象和类之间的通信复杂性。
//! 这种模式提供了一个中介类，该类通常处理不同类之间的通信，并支持松耦合，使代码易于维护。
//! 中介者模式属于行为型模式。
//!

struct ChatRoom;

impl ChatRoom {
    fn show_message(user: &User, message: String) {
        println!("[{}]: {}", user.get_name(), message)
    }
}

struct User {
    name: String
}

impl User{
    fn new(name: String) -> User {
        User{ name }
    }
}

impl User {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn send_message(&self, message: String) {
        ChatRoom::show_message(self, message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mediator() {
        let robert = User::new(String::from("Robert"));
        let john = User::new(String::from("John"));

        robert.send_message(String::from("Hi! john!"));
        john.send_message(String::from("Hello! Robert!"));
    }
}
