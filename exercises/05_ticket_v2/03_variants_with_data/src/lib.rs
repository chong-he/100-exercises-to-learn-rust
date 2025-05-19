// TODO: Implement `Ticket::assigned_to`.
//  Return the name of the person assigned to the ticket, if the ticket is in progress.
//  Panic otherwise.

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
    pub fn assigned_to(&self) -> &str {
        // self is an instance of Ticket, so we can access status using Ticket.status or self.status
        // note that when calling ticket.assigned_to(), there is no input required
        // when we call ticket.assigned_to() (where ticket is a Ticket struct), it will then match the Ticket.status
        // if the status is InProgress, we return "assigned_to", which will be defined in the Ticket struct
        match &self.status {
            Status::InProgress { assigned_to: name } => &name,
            Status::ToDo | Status::Done => {
                panic!("Only `In-Progress` tickets can be assigned to someone")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Only `In-Progress` tickets can be assigned to someone")]
    fn test_todo() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::ToDo);
        ticket.assigned_to();
    }

    #[test]
    #[should_panic(expected = "Only `In-Progress` tickets can be assigned to someone")]
    fn test_done() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::Done);
        ticket.assigned_to();
    }

    #[test]
    fn test_in_progress() {
        let ticket = Ticket::new(
            valid_title(),
            valid_description(),
            Status::InProgress {
                assigned_to: "Alice".to_string(),
            },
        );
        assert_eq!(ticket.assigned_to(), "Alice");
    }
}
