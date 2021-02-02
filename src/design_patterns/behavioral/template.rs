//!
//! 在模板模式（Template Pattern）中，一个抽象类公开定义了执行它的方法的方式/模板。
//! 它的子类可以按需要重写方法实现，但调用将以抽象类中定义的方式进行。
//! 这种类型的设计模式属于行为型模式。
//!

trait Game {
    fn initialize(&self);
    fn start_play(&self);
    fn end_play(&self);
    fn play(&self) {
        self.initialize();
        self.start_play();
        self.end_play();
    }
}

struct Cricket;

impl Cricket {
    fn new() -> Cricket {
        Cricket
    }
}

impl Game for Cricket {
    fn initialize(&self) {
        println!("Cricket Game Initialized! Start playing.")
    }

    fn start_play(&self) {
        println!("Cricket Game Started. Enjoy the game!")
    }

    fn end_play(&self) {
        println!("Cricket Game Finished!")
    }
}

struct Football;

impl Football {
    fn new() -> Football {
        Football
    }
}

impl Game for Football {
    fn initialize(&self) {
        println!("Football Game Initialized! Start playing.")
    }

    fn start_play(&self) {
        println!("Football Game Started. Enjoy the game!")
    }

    fn end_play(&self) {
        println!("Football Game Finished!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        let cricket = Cricket::new();
        cricket.play();
        println!("\n");
        let football = Football::new();
        football.play();
    }
}
