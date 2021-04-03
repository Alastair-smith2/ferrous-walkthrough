use url::{Host, Position, Url};

pub type Result<T> = std::result::Result<T, std::io::Error>;

pub fn parse_url(line: String) -> Option<Url> {
    Url::parse(&line).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_url_works() {
        let url = parse_url(String::from("https://docs.rs/lyon_core/0.8.0/lyon_core/")).unwrap();
        assert!(url.scheme() == "https");
        assert!(url.username() == "");
        assert!(url.password() == None);
        assert!(url.host_str() == Some("docs.rs"));
        assert!(url.host() == Some(Host::Domain("docs.rs")));
        assert!(url.port() == None);
        assert!(url.path() == "/lyon_core/0.8.0/lyon_core/");
        assert!(url.query() == None);
        assert!(&url[Position::BeforePath..] == "/lyon_core/0.8.0/lyon_core/");
        assert!(url.fragment() == None);
        assert!(!url.cannot_be_a_base());
    }

    #[test]
    fn parse_url_fails() {
        assert_eq!(parse_url(String::from("Fizz")), None)
    }
}
