// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    // with Clone, we need to explicit call .clone() to create a duplicate
    // Copy trait is implicit, means Rust does it automatically for us
    // In this exercise, we can't implement Copy trait, because String does not implement Copy
    // therefore, only derive(Clone), no Copy, cannot impl Copy for String also, because String uses heap memory
    // See: https://rust-exercises.com/100-exercises/04_traits/12_copy.html#what-can-be-copy
    (ticket.clone(), ticket.summary())
}

#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
