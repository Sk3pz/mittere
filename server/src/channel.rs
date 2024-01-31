use std::sync::{Arc, mpsc, Mutex};
use common::message::Message;

pub struct Channel {
    message_sender: Arc<Mutex<mpsc::Sender<Message>>>,
    message_receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
}

impl Channel {

    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            message_sender: Arc::new(Mutex::new(sender)),
            message_receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn send(&self, message: Message) {
        self.message_sender.lock().unwrap().send(message).unwrap();
    }

    pub fn receive(&self) -> Message {
        self.message_receiver.lock().unwrap().recv().unwrap()
    }

}

impl Clone for Channel {

    fn clone(&self) -> Self {
        Self {
            message_sender: self.message_sender.clone(),
            message_receiver: self.message_receiver.clone(),
        }
    }
}