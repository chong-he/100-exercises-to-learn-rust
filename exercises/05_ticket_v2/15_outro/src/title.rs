// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct TicketTitle(String);

#[derive(Debug)]
pub struct ParseTitleError {
    invalid_title: String,
}

impl TryFrom<&str> for TicketTitle {
    type Error = ParseTitleError;

    fn try_from(title: &str) -> Result<Self, Self::Error> {
        if title.is_empty() {
            return Err(ParseTitleError {
                invalid_title: title.to_string(),
            });
        }
        if title.len() > 50 {
            return Err(ParseTitleError {
                invalid_title: title.to_string(),
            });
        }

        Ok(TicketTitle(title.to_string()))
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = ParseTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl Display for ParseTitleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.invalid_title.is_empty() {
            write!(f, "The title cannot be empty")
        } else {
            write!(f, "The title cannot be longer than 50 bytes")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
