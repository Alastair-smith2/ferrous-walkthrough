use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Command<'a> {
    Publish(&'a str),
    Retrieve,
}

pub fn parse(input: &str) -> std::result::Result<Command, Error> {
    // unimplemented!()
    let message_type: Vec<&str> = input.splitn(2, '\n').map(|s| s.trim()).collect();
    if message_type.len() > 2 {
        return Err(Error {});
    }

    let result = match (
        message_type[0].starts_with("PUBLISH"),
        message_type[0].starts_with("RETRIEVE"),
    ) {
        (true, false) => {
            let message = message_type[0];
            let split_message: Vec<&str> = message.splitn(2, ' ').collect();
            Ok(Command::Publish(&split_message[1]))
        }
        (false, true) => Ok(Command::Retrieve),
        _ => Err(Error {}),
    };

    result
}

#[test]
fn test_publish() {
    let line = "PUBLISH TestMessage\n";
    let result: Result<Command, Error> = parse(line);
    let expected = Ok(Command::Publish("TestMessage".into()));
    assert_eq!(result, expected);
}
