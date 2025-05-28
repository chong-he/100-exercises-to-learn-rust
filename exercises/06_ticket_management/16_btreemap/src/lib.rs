// TODO: Replace `todo!()`s with the correct implementation.
//  Implement `IntoIterator` for `&TicketStore`. The iterator should yield immutable
//  references to the tickets, ordered by their `TicketId`.
//  Implement additional traits on `TicketId` if needed.

use std::collections::{btree_map, BTreeMap};
use std::ops::{Index, IndexMut};
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

// Implement IntoIterator for the &TicketStore struct as required by the exercise
// refer chap6sec06 for lifetime
impl<'a> IntoIterator for &'a TicketStore {
    type Item = &'a Ticket; // immutable reference to Ticket, so &Ticket
                            // The RHS of the line below is the return type
                            // we can see the return type by defining a variable a
                            // btree_map is a module: https://doc.rust-lang.org/std/collections/btree_map/index.html
    type IntoIter = btree_map::Values<'a, TicketId, Ticket>;
    fn into_iter(self) -> Self::IntoIter {
        // A method in BTreeMap to iterate over values only:
        // https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html#impl-BTreeMap%3CK,+V,+A%3E-2
        // Ticket is the value in the BTreeMap, so we iterate over the Ticket
        let a = self.tickets.values();
        a
    }
}

// Ord is required to be implemented, the get method below will prompt this if don't have Ord
// Because got Ord, also needs PartialOrd and Eq
#[derive(Clone, Copy, Debug, PartialEq, Ord, PartialOrd, Eq)]
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
            tickets: BTreeMap::new(),
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
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
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
    use crate::{Status, TicketDraft, TicketId, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let n_tickets = 5;

        for i in 0..n_tickets {
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

        let ids: Vec<TicketId> = (&store).into_iter().map(|t| t.id).collect();
        let sorted_ids = {
            let mut v = ids.clone();
            v.sort();
            v
        };
        assert_eq!(ids, sorted_ids);
    }
}
