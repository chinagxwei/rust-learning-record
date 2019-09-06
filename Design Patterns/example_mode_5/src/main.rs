trait State {
    fn download(self: Box<Self>) -> Box<dyn State>;
    fn pause(self: Box<Self>) -> Box<dyn State>;
    fn fail(self: Box<Self>) -> Box<dyn State>;
    fn finish(self: Box<Self>) -> Box<dyn State>;
}

struct ReadState {
    state: Option<Box<dyn State>>
}

impl ReadState {
    fn new() -> ReadState {
        ReadState { state: Some(Box::new(Download {}))}
    }
}

impl ReadState {
    fn download(&mut self) {
        if let Some(state) = self.state.take(){
            self.state = Some(state.download())
        }
    }
}

impl ReadState {
    fn pause(&mut self) {
        if let Some(state) = self.state.take(){
            self.state = Some(state.pause())
        }
    }
}

impl ReadState {
    fn fail(&mut self) {
        if let Some(state) = self.state.take(){
            self.state = Some(state.fail())
        }
    }
}

impl ReadState {
    fn finish(&mut self) {
        if let Some(state) = self.state.take(){
            self.state = Some(state.finish())
        }
    }
}

struct Download;

struct Pause;

struct Fail;

struct Finish;

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

impl State for Pause {
    fn download(self: Box<Self>) -> Box<dyn State> {
        println!("继续下载");
        Box::new(Download {})
    }

    fn pause(self: Box<Self>) -> Box<dyn State> {
        println!("暂停中");
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

fn main() {
    let mut rs = ReadState::new();
    rs.download();
    rs.fail();
    rs.pause();
    rs.download();
    rs.finish();
    rs.pause();
    rs.fail();
}
