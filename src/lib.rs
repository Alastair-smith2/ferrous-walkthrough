use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub enum Command {
    Publish(String),
    Retrieve,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    UnknownVerb,
    UnexpectedPayload,
    MissingPayload,
    EmptyMessage,
    IncompleteMessage,
    TrailingData,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing is command: {:?}!", self)
    }
}

impl std::error::Error for Error {}

pub fn parse(input: &str) -> Result<Command, Error> {
    if let Some(pos) = input.find('\n') {
        if !((pos + 1) == input.len()) {
            return Err(Error::TrailingData);
        }
    } else {
        return Err(Error::IncompleteMessage);
    }

    let mut split = input.splitn(2, ' ');

    if let Some(verb) = split.next() {
        match verb.trim() {
            "RETRIEVE" => {
                if split.next() == None {
                    Ok(Command::Retrieve)
                } else {
                    Err(Error::UnexpectedPayload)
                }
            }
            "PUBLISH" => {
                if let Some(payload) = split.next() {
                    Ok(Command::Publish(String::from(payload.trim())))
                } else {
                    Err(Error::MissingPayload)
                }
            }
            "" => Err(Error::EmptyMessage),
            _ => Err(Error::UnknownVerb),
        }
    } else {
        Err(Error::EmptyMessage)
    }
}

#[test]
fn test_retrieve() {
    let line = "RETRIEVE\n";
    let result = parse(line);
    assert_eq!(result, Ok(Command::Retrieve));
}

#[test]
fn test_publish() {
    let line = "PUBLISH TestMessage\n";
    let result = parse(line);
    assert_eq!(result, Ok(Command::Publish("TestMessage".into())));
}

#[test]
fn test_empty_string() {
    let line = "";
    let result = parse(line);
    assert_eq!(result, Err(Error::IncompleteMessage));
}

#[test]
fn test_empty_message() {
    let line = "\n";
    let result = parse(line);
    assert_eq!(result, Err(Error::EmptyMessage));
}

#[test]
fn test_missing_newline() {
    let line = "FooBar";
    let result = parse(line);
    assert_eq!(result, Err(Error::IncompleteMessage));
}

#[test]
fn test_retrieve_with_payload() {
    let line = "RETRIEVE Payload\n";
    let result = parse(line);
    assert_eq!(result, Err(Error::UnexpectedPayload));
}

#[test]
fn test_publish_without_payload() {
    let line = "PUBLISH\n";
    let result = parse(line);
    assert_eq!(result, Err(Error::MissingPayload));
}

#[test]
fn test_publish_with_empty_payload() {
    let line = "PUBLISH \n";
    let result = parse(line);
    assert_eq!(result, Ok(Command::Publish("".into())));
}

#[test]
fn test_inline_newline() {
    let line = "PUBLISH fooo\nbar\n";
    let result = parse(line);
    assert_eq!(result, Err(Error::TrailingData));
}
