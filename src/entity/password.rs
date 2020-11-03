use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl TryFrom<&str> for Password {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        if s.len() <= 255 {
            Ok(Password(s.to_owned()))
        } else {
            Err("Invalid format")
        }
    }
}

impl FromStr for Password {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::try_from(s)
    }
}

impl From<Password> for String {
    fn from(password: Password) -> Self {
        password.0
    }
}
