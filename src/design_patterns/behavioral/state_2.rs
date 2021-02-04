//!
//! 在状态模式（State Pattern）中，类的行为是基于它的状态改变的。
//! 这种类型的设计模式属于行为型模式。
//!
//! 在状态模式中，我们创建表示各种状态的对象和一个行为随着状态对象改变而改变的状态对象。
//!
//! 以下是由 enum 实现状态模式
//!

use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
enum StateType {
    INIT,
    START,
    PAUSE,
    FAIL,
    COMPLETE,
}

impl StateType {
    fn init(&self) -> StateType {
        match self {
            StateType::INIT => *self,
            StateType::START => *self,
            StateType::PAUSE => *self,
            StateType::FAIL => *self,
            StateType::COMPLETE => {
                StateType::INIT
            }
        }
    }

    fn start(&self) -> StateType {
        match *self {
            StateType::INIT => {
                StateType::START
            }
            StateType::START => *self,
            StateType::PAUSE => {
                StateType::START
            }
            StateType::FAIL => *self,
            StateType::COMPLETE => {
                StateType::START
            }
        }
    }

    fn pause(&self) -> StateType {
        match self {
            StateType::INIT => *self,
            StateType::START => {
                StateType::PAUSE
            }
            StateType::PAUSE => *self,
            StateType::FAIL => *self,
            StateType::COMPLETE => *self,
        }
    }

    fn fail(&self) -> StateType {
        match *self {
            StateType::INIT => *self,
            StateType::START => {
                StateType::FAIL
            }
            StateType::PAUSE => *self,
            StateType::FAIL => *self,
            StateType::COMPLETE => *self,
        }
    }

    fn complete(&self) -> StateType {
        match *self {
            StateType::INIT => *self,
            StateType::START => {
                StateType::COMPLETE
            }
            StateType::PAUSE => *self,
            StateType::FAIL => *self,
            StateType::COMPLETE => *self,
        }
    }
}

impl StateType {
    fn as_str(&self) -> &'static str {
        match self {
            StateType::INIT => "初始化",
            StateType::START => "开始",
            StateType::PAUSE => "暂停",
            StateType::FAIL => "失败",
            StateType::COMPLETE => "完成",
        }
    }
}

impl Display for StateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
struct ReadState {
    state: StateType
}

impl ReadState {
    fn new() -> ReadState {
        ReadState { state: StateType::INIT }
    }
}

impl ReadState {
    fn init(&mut self) {
        self.state = self.state.init();
        println!("当前状态：{}", self.state);
    }
    fn start(&mut self) {
        self.state = self.state.start();
        println!("当前状态：{}", self.state);
    }
    fn pause(&mut self) {
        self.state = self.state.pause();
        println!("当前状态：{}", self.state);
    }
    fn fail(&mut self) {
        self.state = self.state.fail();
        println!("当前状态：{}", self.state);
    }
    fn complete(&mut self) {
        self.state = self.state.complete();
        println!("当前状态：{}", self.state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_2() {
        let mut rs = ReadState::new();
        rs.init();
        rs.fail();
        rs.pause();
        rs.complete();
        rs.start();
        rs.pause();
        rs.complete();
        rs.start();
        rs.complete();
        println!("{:?}", rs);
    }
}
