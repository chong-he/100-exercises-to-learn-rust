pub mod ticket {
    pub struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Ticket {
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
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }

        // TODO: Add three public methods to the `Ticket` struct:
        //  - `title` that returns the `title` field.
        //  - `description` that returns the `description` field.
        //  - `status` that returns the `status` field.
        pub fn title(&self) -> &str {
            &self.title
        }

        pub fn description(&self) -> &str {
            &self.description
        }

        pub fn status(&self) -> &str {
            &self.status
        }
        // the idea is that we keep the fields of Ticket struct private (so they can't be modified directly)
        // and if we want to use Ticket struct, we have to go through Ticket::new()
        // and we implement the constraint (checking) of the fields (e.g., cannot be too long etc) on new
        // and to modify the values of title, description and status, we do it via: ticket.title()
    }
}

#[cfg(test)]
mod tests {
    use super::ticket::Ticket;

    #[test]
    fn description() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.description(), "A description");
    }

    #[test]
    fn title() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.title(), "A title");
    }

    #[test]
    fn status() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.status(), "To-Do");
    }
}
