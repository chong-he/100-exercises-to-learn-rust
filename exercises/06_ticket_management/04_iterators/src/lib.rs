use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Let's start sketching our ticket store!
//  First task: implement `IntoIterator` on `TicketStore` to allow iterating over all the tickets
//  it contains using a `for` loop.
//
// Hint: you shouldn't have to implement the `Iterator` trait in this case.
//   You want to *delegate* the iteration to the `Vec<Ticket>` field in `TicketStore`.
//   Look at the standard library documentation for `Vec` to find the right type
//   to return from `into_iter`.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

// Implement IntoIterator for the TicketStore struct
impl IntoIterator for TicketStore {
    // Item is the thing to be iterated, e.g., in "for i in array", i is the Item
    // So, here, we iterate over each Ticket in the Vec<Ticket>
    type Item = Ticket;
    // The below type IntoIter is about: which kind of iterator are we turning into
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-for-Vec%3CT,+A%3E
    // this is about when we call .into_iter(), what type we get back? A: we get back std::vec::IntoIter<Ticket>
    // e.g., when we do: let store = TicketStore::new();
    // let iterator = store.into_iter(); --> this iterator is of type std::vec::IntoIter<Ticket>
    // for ticket in iterator {} --> the iterator is as above, is the type that allows iteration
    // and ticket is the individual Ticket that we iterate through
    type IntoIter = std::vec::IntoIter<Ticket>;
    fn into_iter(self) -> Self::IntoIter {
        // self is TicketStore
        // so, self.tickets is Vec<Ticket>
        // self.tickets.into_iter() is calling into_iter() on Vec<Ticket>
        self.tickets.into_iter()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
