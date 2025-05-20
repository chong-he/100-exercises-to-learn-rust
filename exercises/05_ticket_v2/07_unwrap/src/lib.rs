// TODO: `easy_ticket` should panic when the title is invalid.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    // The below also works, but it's lengthy. After looking at the solution, it's simpler
    // if title.is_empty() {
    //     panic!("Title cannot be empty")
    // };

    // if title.len() > 50 {
    //     panic!("Title cannot be longer than 50 bytes")
    // };

    // let description_to_use = if description.is_empty() || description.len() > 500 {
    //     "Description not provided".to_string()
    // } else {
    //     description
    // };

    // Ticket {
    //     title,
    //     description: description_to_use,
    //     status,
    // }
    match Ticket::new(title.clone(), description, status.clone()) {
        // Ok will return Ticket type, this is from the Result<Ticket, String>
        Ok(ticket) => ticket,
        // Err is a String, when it errors, it will return the Err() defined in Ticket::new
        // When the error contains description, we know that it has hit either "if description.is_empty()" or "if description.len() > 500"
        // (because or else it wouldn't reach the Err variant)
        // All description related errors contain the word "Description", so if it contains this word,
        // we return a Ticket with the field description: "Description not provided"
        // the .unwrap() is used to extract the Ticket type from Result<Ticket, String> in the Ticket::new(...).unwrap()
        Err(e) => {
            if e.contains("Description") {
                Ticket::new(title, "Description not provided".to_string(), status).unwrap()
            } else {
                panic!("{}", e)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, String> {
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if title.len() > 50 {
            return Err("Title cannot be longer than 50 bytes".to_string());
        }
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if description.len() > 500 {
            return Err("Description cannot be longer than 500 bytes".to_string());
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
