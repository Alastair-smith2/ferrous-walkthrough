use url::Url;

pub type Result<T> = std::result::Result<T, std::io::Error>;

pub fn parse_url(line: String) -> Option<Url> {
    Url::parse(&line).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_url_works() {
        assert!(parse_url(String::from("https://docs.rs/lyon_core/0.8.0/lyon_core/")).is_some());
    }

    #[test]
    fn parse_url_fails() {
        assert!(parse_url(String::from("Fizz")).is_none());
    }
}
