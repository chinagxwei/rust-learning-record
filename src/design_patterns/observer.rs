use std::rc::{Rc, Weak};
use std::cell::RefCell;

trait Observer {
    fn update(&self, state: i32);
}

struct Subject {
    observers: Vec<Box<dyn Observer>>,
    state: i32,
}

impl Subject {
    fn new() -> Subject {
        Subject { observers: vec![], state: 0 }
    }
}

impl Subject {
    fn get_state(&self) -> i32 {
        self.state
    }

    fn set_state(&mut self, state: i32) {
        self.state = state;
        self.notify_all_observers()
    }

    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn notify_all_observers(&self) {
        for observer in self.observers.iter() {
            observer.update(self.state);
        }
    }
}

struct BinaryObserver;

impl BinaryObserver {
    fn new() -> BinaryObserver {
        BinaryObserver
    }
}

impl Observer for BinaryObserver {
    fn update(&self, state: i32) {
        println!("Binary String: {}", state)
    }
}

struct OctalObserver;

impl OctalObserver {
    fn new() -> OctalObserver {
        OctalObserver
    }
}

impl Observer for OctalObserver {
    fn update(&self, state: i32) {
        println!("Octal String: {}", state * 2)
    }
}

struct HexObserver;

impl HexObserver {
    fn new() -> HexObserver {
        HexObserver
    }
}

impl Observer for HexObserver {
    fn update(&self, state: i32) {
        println!("Hex String: {}", state * 9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer() {
        let mut s = Subject::new();

        s.attach(Box::new(BinaryObserver::new()));
        s.attach(Box::new(OctalObserver::new()));
        s.attach(Box::new(HexObserver::new()));

        println!("First state change: 15");
        s.set_state(15);

        println!("Second state change: 10");
        s.set_state(10);
    }
}
