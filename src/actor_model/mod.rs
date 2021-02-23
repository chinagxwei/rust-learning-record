use tokio::sync::{oneshot, mpsc};

///
/// Example from https://ryhl.io/blog/actors-with-tokio/
///

struct MyActor {
    next_id: u32,
    receiver: mpsc::Receiver<ActorMessage>,
}

enum ActorMessage {
    GetUniqueID {
        respond_to: oneshot::Sender<u32>
    }
}

impl MyActor {
    pub fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor { next_id: 0, receiver }
    }

    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueID { respond_to } => {
                self.next_id += 1;
                respond_to.send(self.next_id);
            }
        }
    }
}

async fn run_my_actor(mut actor: MyActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg)
    }
}

#[derive(Clone)]
pub struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>
}

impl MyActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = MyActor::new(receiver);
        tokio::spawn(run_my_actor(actor));
        MyActorHandle { sender }
    }

    pub fn send(&self,msg:ActorMessage){
        self.sender.send(msg).await;
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetUniqueID { respond_to: send };

        self.sender.send(msg).await;
        recv.await.expect("ctor task has been killed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn my_test() {
        assert!(true);

        let handle = MyActorHandle::new();

        let id = handle.get_unique_id().await;

        println!("{}", id);
    }
}
