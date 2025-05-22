// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

// The type T is String, so I put <String> here:
// Ref: https://rust-exercises.com/100-exercises/05_ticket_v2/13_try_from#tryfrom-and-tryinto-1
impl TryFrom<String> for Status {
    // we put the Error type as String
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_lowercase = value.to_lowercase(); //Note: to_lowercase() returns a String
                                                    // the pattern on the LHS of match "ToDo" is &str, so need to convert
        match value_lowercase.as_str() {
            // after convert value to lowercase, the pattern should all be lowercase so that they can match
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("Invalid status: {}", value)),
        }
    }
}

// The following, from the solution given, is simpler
// It uses the fact that since we implement TryFrom<&str>, so we get try_into() for free, so we use that
// impl TryFrom<String> for Status {
//     type Error = String;
//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         value.as_str().try_into()
//     }
// }

impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value_lowercase = value.to_lowercase();
        match value_lowercase.as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("Invalid status: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
