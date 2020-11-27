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
