// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketDescription` type,
//   enforcing that the description is not empty and is not longer than 500 bytes.
//   Implement the traits required to make the tests pass too.

use std::fmt::{self, Display, Formatter};

// because Ticket struct in lib.rs implements Clone (which means all fields in Ticket got Clone)
// so we need clone for TicketDescription too
#[derive(PartialEq, Debug, Clone)]
pub struct TicketDescription(String);

#[derive(Debug)]
pub struct ParseDescriptionError {
    invalid_description: String,
}

impl TryFrom<&str> for TicketDescription {
    type Error = ParseDescriptionError;

    fn try_from(description: &str) -> Result<Self, Self::Error> {
        if description.is_empty() {
            return Err(ParseDescriptionError {
                invalid_description: description.to_string(),
            });
        }
        if description.len() > 500 {
            return Err(ParseDescriptionError {
                invalid_description: description.to_string(),
            });
        }

        Ok(TicketDescription(description.to_string()))
    }
}

impl TryFrom<String> for TicketDescription {
    type Error = ParseDescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // as_str() is to convert String to &str, so that when we call try_into(),
        // we are calling try_into() on a &str, so Rust goes to TryFrom<&str> impl and executes
        // without as_str(), it would remains as String, and result in infinite loop because it remains in this very impl
        value.as_str().try_into()
    }
}

// need to implement Display because got .to_string() in the test
// refer chap5sec12
// in chap5sec12 for example, we implement Display for TicketNewError, which is an enum with a few variants
// so we use match self to match for each variant
// In this impl, the ParseDescriptionError is a struct, so we use the field
impl Display for ParseDescriptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.invalid_description.is_empty() {
            write!(f, "The description cannot be empty")
        } else {
            write!(f, "The description cannot be longer than 500 bytes")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let description = TicketDescription::try_from("A description".to_string()).unwrap();
        assert_eq!(description.0, "A description");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketDescription::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The description cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let description = "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.".to_string();
        let err = TicketDescription::try_from(description).unwrap_err();
        assert_eq!(
            err.to_string(),
            "The description cannot be longer than 500 bytes"
        );
    }

    #[test]
    fn test_try_from_str() {
        let description = TicketDescription::try_from("A description").unwrap();
        assert_eq!(description.0, "A description");
    }
}
