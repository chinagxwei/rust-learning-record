//!
//! 备忘录模式（Memento Pattern）保存一个对象的某个状态，以便在适当的时候恢复对象。
//! 备忘录模式属于行为型模式。
//!

struct Memento {
    state: &'static str
}

impl Memento {
    fn new(state: &'static str) -> Memento {
        Memento { state }
    }
}

impl Memento {
    fn get_state(&self) -> &'static str {
        &self.state
    }
}

struct Originator {
    state: Option<&'static str>
}

impl Originator {
    fn new() -> Originator {
        Originator { state: None }
    }
}

impl Originator {
    fn get_state(&self) -> &'static str {
        &self.state.unwrap()
    }

    fn set_state(&mut self, state: &'static str) {
        self.state = Some(state);
    }

    fn save_state_to_memento(&self) -> Memento {
        Memento::new(self.state.unwrap())
    }

    fn get_state_from_memento(&mut self, memento: &Memento) {
        self.state = Some(memento.get_state());
    }
}

struct CareTaker {
    memento_list: Vec<Memento>
}

impl CareTaker {
    fn new() -> CareTaker {
        CareTaker { memento_list: vec![] }
    }
}

impl CareTaker {
    fn add(&mut self, memento: Memento) {
        self.memento_list.push(memento);
    }

    fn get(&self, index: usize) -> Option<&Memento> {
        self.memento_list.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memento() {
        let mut o = Originator::new();
        let mut ct = CareTaker::new();

        o.set_state("State #1");
        o.set_state("State #2");
        ct.add(o.save_state_to_memento());
        o.set_state("State #3");
        ct.add(o.save_state_to_memento());
        o.set_state("State #4");

        println!("Current State: {}", o.get_state());
        o.get_state_from_memento(ct.get(0).unwrap());
        println!("First saved State: {}", o.get_state());
        o.get_state_from_memento(ct.get(1).unwrap());
        println!("Second saved State: {}", o.get_state())
    }
}
