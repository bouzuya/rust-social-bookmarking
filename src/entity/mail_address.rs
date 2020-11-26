use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MailAddress(String);

impl TryFrom<&str> for MailAddress {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        let parts: Vec<&str> = s.split('@').collect();
        if s.len() <= 254 && parts.len() == 2 && parts[0].len() <= 64 && parts[1].len() <= 255 {
            Ok(MailAddress(s.to_owned()))
        } else {
            Err("Invalid format")
        }
    }
}

impl FromStr for MailAddress {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::try_from(s)
    }
}

impl From<MailAddress> for String {
    fn from(mail_address: MailAddress) -> Self {
        mail_address.0
    }
}

impl std::fmt::Display for MailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let mail_address: MailAddress = "m@bouzuya.net".parse().unwrap();
        assert_eq!("m@bouzuya.net".to_owned(), mail_address.to_string());
        assert_eq!("m@bouzuya.net".to_owned(), format!("{}", mail_address));
    }
}
