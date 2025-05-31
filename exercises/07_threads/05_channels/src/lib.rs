use crate::data::TicketDraft;
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    // this is just like Variant(type) where Variant = Insert, type = TicketDraft, see Rustbook sec. 6.2 the Coin example
    Insert(TicketDraft),
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.

// The return type of Sender<Command> tells us that the type of tx and rx in the channel is: Command
// So when the client launches a sender (see insert.rs), the sender (in insert.rs) is of type Sender<Command>
// so we can call .send() on sender, i.e., sender.send()
// send what? Command type. So it is sender.send(Command::Insert())
// Why need Insert(TicketDraft)? It is just Insert(some type)
// e.g., later we can have Delete(TicketId) under the same enum Command also
// so that the channel can still have Command as the Sender and Receiver type, which every "function" (e.g., Insert, Delete) will fall under the enum Command
pub fn launch() -> Sender<Command> {
    // This creates a channel
    // A channel can have multiple Senders / sending ends, but only one Receiver
    let (sender, receiver) = std::sync::mpsc::channel();
    // Create a new thread that runs the server() function
    std::thread::spawn(move || server(receiver));
    // so this creates/launches a server that runs in a separate thread
    // the client can then call .send() to send data to this server (see tests/insert.rs)
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    // The server contains the TicketStore database
    let mut store = TicketStore::new();
    // loops means the server never stops (because it needs to receive request from client)
    loop {
        // The line below is addressing: " wait for a command to show up in the channel"
        // the command here is the message from the sender, which the server will receive
        // this is the server, this current server, waiting to receive a message
        // who sends the message? The client, i.e., the client will call .send() to send the message
        let received = receiver.recv().unwrap();
        // when the server receives something (message from client), it does the following:
        // add the TicketDraft (message from client is a TicketDraft) to the store (TicketStore type)
        match received {
            Command::Insert(draft) => {
                store.add_ticket(draft);
            }
        }
    }
}
