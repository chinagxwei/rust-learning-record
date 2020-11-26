///
/// 下载状态特性
///
trait State {
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
        ReadState { state: Some(Box::new(Download {}))}
    }

    fn download(&mut self) {
        if let Some(state) = self.state.take(){
            self.state = Some(state.download())
        }
    }

    fn pause(&mut self) {
        if let Some(state) = self.state.take(){
            self.state = Some(state.pause())
        }
    }

    fn fail(&mut self) {
        if let Some(state) = self.state.take(){
            self.state = Some(state.fail())
        }
    }

    fn finish(&mut self) {
        if let Some(state) = self.state.take(){
            self.state = Some(state.finish())
        }
    }
}

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

///
/// 下载状态结构实现
///
impl State for Download {
    fn download(self: Box<Self>) -> Box<dyn State> {
        println!("下载中");
        self
    }

    fn pause(self: Box<Self>) -> Box<dyn State> {
        println!("暂停下载");
        Box::new(Pause {})
    }

    fn fail(self: Box<Self>) -> Box<dyn State> {
        println!("下载失败");
        Box::new(Fail {})
    }

    fn finish(self: Box<Self>) -> Box<dyn State> {
        println!("下载完成");
        Box::new(Finish {})
    }
}

///
/// 暂停状态结构实现
///
impl State for Pause {
    fn download(self: Box<Self>) -> Box<dyn State> {
        println!("继续下载");
        Box::new(Download {})
    }

    fn pause(self: Box<Self>) -> Box<dyn State> {
        println!("暂停中无法暂停");
        self
    }

    fn fail(self: Box<Self>) -> Box<dyn State> {
        println!("下载失败");
        Box::new(Fail {})
    }

    fn finish(self: Box<Self>) -> Box<dyn State> {
        println!("下载完成");
        Box::new(Finish {})
    }
}

///
/// 完成状态结构实现
///
impl State for Finish {
    fn download(self: Box<Self>) -> Box<dyn State> {
        println!("重新下载");
        Box::new(Download {})
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
    fn download(self: Box<Self>) -> Box<dyn State> {
        println!("尝试重新下载");
        Box::new(Download {})
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
