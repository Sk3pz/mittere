use std::sync::{Arc, mpsc, Mutex};
use crate::message::Message;

pub struct ClientChannel {
    message_sender: Arc<Mutex<mpsc::Sender<Message>>>,
    message_receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
}

impl ClientChannel {

    pub fn new(broadcast_receiver: Arc<Mutex<mpsc::Receiver<Message>>>, client_sender: Arc<Mutex<mpsc::Sender<Message>>>) -> Self {
        Self {
            message_sender: client_sender,
            message_receiver: broadcast_receiver,
        }
    }

    pub fn send(&self, message: Message) {
        self.message_sender.lock().unwrap().send(message).unwrap();
    }

    pub fn receive(&self) -> Message {
        self.message_receiver.lock().unwrap().recv().unwrap()
    }

}

impl Clone for ClientChannel {
    fn clone(&self) -> Self {
        Self {
            message_sender: self.message_sender.clone(),
            message_receiver: self.message_receiver.clone(),
        }
    }
}

pub struct ServerChannel {
    message_sender: mpsc::Sender<Message>,
    message_receiver: mpsc::Receiver<Message>,
}

impl ServerChannel {

        pub fn new() -> (Self, ClientChannel) {
            // client_sender is used to send messages from the client to the server
            // client_receiver receives messages from the client
            let (client_sender, client_receiver) = mpsc::channel::<Message>();
            // broadcast_sender is used to send messages from the server to the client
            // broadcast_receiver receives messages from the server to the client
            let (broadcast_sender, broadcast_receiver) = mpsc::channel::<Message>();

            let client_channel = ClientChannel {
                message_sender: Arc::new(Mutex::new(client_sender)),
                message_receiver: Arc::new(Mutex::new(broadcast_receiver)),
            };

            let server_channel = ServerChannel {
                message_sender: broadcast_sender,
                message_receiver: client_receiver,
            };

            (server_channel, client_channel)
        }

        pub fn send(&self, message: Message) {
            self.message_sender.send(message).unwrap();
        }

        pub fn receive(&self) -> Message {
            self.message_receiver.recv().unwrap()
        }

}