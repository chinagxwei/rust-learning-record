//!
//! 在状态模式（State Pattern）中，类的行为是基于它的状态改变的。
//! 这种类型的设计模式属于行为型模式。
//!
//! 在状态模式中，我们创建表示各种状态的对象和一个行为随着状态对象改变而改变的状态对象。
//!
//! 以下是由 trait 实现状态模式
//!

///
/// 下载状态特性
///
trait State {
    fn init(self:Box<Self>)-> Box<dyn State>;
    fn download(self: Box<Self>) -> Box<dyn State>;
    fn pause(self: Box<Self>) -> Box<dyn State>;
    fn fail(self: Box<Self>) -> Box<dyn State>;
    fn finish(self: Box<Self>) -> Box<dyn State>;
}

///
/// 读取状态结构
///
struct ReadState {
    state: Option<Box<dyn State>>
}

///
/// 读取状态结构实现
///
impl ReadState {
    fn new() -> ReadState {
        ReadState { state: Some(Box::new(Init)) }
    }

    fn init(&mut self){
        if let Some(state) = self.state.take() {
            self.state = Some(state.init())
        }
    }

    fn download(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.download())
        }
    }

    fn pause(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.pause())
        }
    }

    fn fail(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.fail())
        }
    }

    fn finish(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.finish())
        }
    }
}

///
/// 初始化状态结构
///
struct Init;

///
/// 下载状态结构
///
struct Download;

///
/// 暂停状态结构
///
struct Pause;

///
/// 完成状态结构
///
struct Finish;

///
/// 失败状态结构
///
struct Fail;

impl State for Init{
    fn init(self: Box<Self>) -> Box<dyn State> {
        println!("初始化中");
        self
    }

    fn download(self: Box<Self>) -> Box<dyn State> {
        println!("进行下载");
        Box::new(Download)
    }

    fn pause(self: Box<Self>) -> Box<dyn State> {
        println!("初始化中");
        self
    }

    fn fail(self: Box<Self>) -> Box<dyn State> {
        println!("初始化中");
        self
    }

    fn finish(self: Box<Self>) -> Box<dyn State> {
        println!("初始化中");
        self
    }
}

///
/// 下载状态结构实现
///
impl State for Download {
    fn init(self: Box<Self>) -> Box<dyn State> {
        println!("下载中");
        self
    }

    fn download(self: Box<Self>) -> Box<dyn State> {
        println!("下载中");
        self
    }

    fn pause(self: Box<Self>) -> Box<dyn State> {
        println!("暂停下载");
        Box::new(Pause)
    }

    fn fail(self: Box<Self>) -> Box<dyn State> {
        println!("下载失败");
        Box::new(Fail)
    }

    fn finish(self: Box<Self>) -> Box<dyn State> {
        println!("下载完成");
        Box::new(Finish)
    }
}

///
/// 暂停状态结构实现
///
impl State for Pause {
    fn init(self: Box<Self>) -> Box<dyn State> {
        println!("暂停中");
        self
    }

    fn download(self: Box<Self>) -> Box<dyn State> {
        println!("继续下载");
        Box::new(Download)
    }

    fn pause(self: Box<Self>) -> Box<dyn State> {
        println!("暂停中无法暂停");
        self
    }

    fn fail(self: Box<Self>) -> Box<dyn State> {
        println!("暂停中");
        self
    }

    fn finish(self: Box<Self>) -> Box<dyn State> {
        println!("暂停中");
        self
    }
}

///
/// 完成状态结构实现
///
impl State for Finish {
    fn init(self: Box<Self>) -> Box<dyn State> {
        println!("重新初始化");
        Box::new(Init)
    }

    fn download(self: Box<Self>) -> Box<dyn State> {
        println!("重新下载");
        Box::new(Download)
    }

    fn pause(self: Box<Self>) -> Box<dyn State> {
        println!("下载已完成无法暂停");
        self
    }

    fn fail(self: Box<Self>) -> Box<dyn State> {
        println!("下载已完成无法失败");
        self
    }

    fn finish(self: Box<Self>) -> Box<dyn State> {
        println!("下载已完成");
        self
    }
}

///
/// 失败状态结构实现
///
impl State for Fail {
    fn init(self: Box<Self>) -> Box<dyn State> {
        println!("重新初始化");
        Box::new(Init)
    }

    fn download(self: Box<Self>) -> Box<dyn State> {
        println!("尝试重新下载");
        Box::new(Download)
    }

    fn pause(self: Box<Self>) -> Box<dyn State> {
        println!("下载失败无法暂停");
        self
    }

    fn fail(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn finish(self: Box<Self>) -> Box<dyn State> {
        println!("下载失败无法完成");
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let mut rs = ReadState::new();
        rs.fail();
        rs.pause();
        rs.finish();
        rs.download();
        rs.download();
        rs.fail();
        rs.pause();
        rs.download();
        rs.pause();
        rs.download();
        rs.finish();
        rs.pause();
        rs.fail();
    }
}
