// TODO: Replace `todo!()`s with the correct implementation.
//  Implement additional traits on `TicketId` if needed.

use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: HashMap<TicketId, Ticket>,
    counter: u64,
}

// need to add Eq and Hash for TicketId (the key in HashMap)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TicketId(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: HashMap::new(), //previously was Vec::new(), now that tickets are HashMap, so we create an empty HashMap
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        // previously was: self.tickets.push(ticket)
        // to extend the Vec<Ticket> as new ticket being added
        // Now that TIcketStore.tickets is a HashMap, so we insert a new key-value pair instead
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        // previously was: self.tickets.iter().find(|&t| t.id == id)
        // now, we want to return Ticket (value),  when supplying TicketId (key)
        // this is exactly what .get() does, supply the key, and it will return Option<&Value> (Rustbook Sec8.3)
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        // since it returns mut, so we use get_mut
        self.tickets.get_mut(&id)
    }
}

impl Index<TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: TicketId) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl Index<&TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: &TicketId) -> &Self::Output {
        &self[*index]
    }
}

impl IndexMut<TicketId> for TicketStore {
    fn index_mut(&mut self, index: TicketId) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl IndexMut<&TicketId> for TicketStore {
    fn index_mut(&mut self, index: &TicketId) -> &mut Self::Output {
        &mut self[*index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id = store.add_ticket(draft.clone());
        let ticket = &store[id];
        assert_eq!(draft.title, ticket.title);
        assert_eq!(draft.description, ticket.description);
        assert_eq!(ticket.status, Status::ToDo);

        let ticket = &mut store[id];
        ticket.status = Status::InProgress;

        let ticket = &store[id];
        assert_eq!(ticket.status, Status::InProgress);
    }
}
