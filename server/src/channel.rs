use std::sync::{Arc, mpsc, Mutex};

pub struct AtomicChannel<T> {
    message_sender: Arc<Mutex<mpsc::Sender<T>>>,
    message_receiver: Arc<Mutex<mpsc::Receiver<T>>>,
}

impl<T> AtomicChannel<T> {

    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            message_sender: Arc::new(Mutex::new(sender)),
            message_receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn send(&self, message: T) {
        self.message_sender.lock().unwrap().send(message).unwrap();
    }

    pub fn receive(&self) -> T {
        self.message_receiver.lock().unwrap().recv().unwrap()
    }

}

impl<T> Clone for AtomicChannel<T> {

    fn clone(&self) -> Self {
        Self {
            message_sender: self.message_sender.clone(),
            message_receiver: self.message_receiver.clone(),
        }
    }
}