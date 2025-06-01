use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        // How to know it is Option <Ticket>?
        // Refer to store.get() method, it returns Option<&Ticket>, so we know it has something to do about it
        // from the test:  let ticket: Ticket = response_receiver
        //.recv()
        //.expect("No response received!")
        // .unwrap();
        // because of unwrap(), so we know got Option<T>, and ticket = type Ticket, so Option<Ticket>
        response_sender: Sender<Option<Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                let ticket_id = store.add_ticket(draft);
                // this is to send response back to client
                // so we see this in the test:
                // let ticket_id: TicketId = response_receiver.recv().expect("No response received!");
                // The test is the client side, it is expecting to receive a TicketId
                // therefore, in the enum, we write:  response_sender: Sender<TicketId>,
                response_sender.send(ticket_id).unwrap()
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                // use .cloned() can convert Option<&T> to Option<T>
                // we want Option<T> because the client expects to see:
                // let ticket: Ticket = response_receiver
                // .recv()
                // Option<Ticket>, not Option<&Ticket>
                let ticket = store.get(id).cloned();
                response_sender.send(ticket).unwrap()
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
