use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

// TODO: Implement the patching functionality.
use crate::data::{Ticket, TicketDraft, TicketPatch};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }

    pub fn update(&self, ticket_patch: TicketPatch) -> Result<(), OverloadedError> {
        // Same logic as other functions like insert and get
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Update {
                patch: ticket_patch,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("The store is overloaded")]
pub struct OverloadedError;

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
    Update {
        patch: TicketPatch,
        response_channel: SyncSender<()>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Ok(Command::Update {
                patch,
                response_channel,
            }) => {
                // here, we want to update the fields
                // get_mut returns a &mut Ticket type
                // the whole thing inside the "if let" only runs if the ticket exists
                if let Some(ticket) = store.get_mut(patch.id) {
                    // if the ticket exists, we modify the ticket's fields
                    // patch is a TicketPatch, sent by the client
                    // patch.title is of type Option<TicketTitle>, it can be Some(title) or None
                    // the "ticket.title = title" only runs if patch.title is Some(value)
                    // Some(title) is the pattern, patch.title is the expression
                    // So the pattern we are trying to match against is: Some(title)
                    // patch.title returns Option<TicketTitle>
                    // Some(title) = Some(TicketTitle) = Option<TicketTitle>, so it matches
                    if let Some(title) = patch.title {
                        // modify the ticket title if it matches
                        ticket.title = title;
                    }
                    if let Some(description) = patch.description {
                        ticket.description = description;
                    }
                    if let Some(status) = patch.status {
                        ticket.status = status;
                    }
                }
                let _ = response_channel.send(());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
