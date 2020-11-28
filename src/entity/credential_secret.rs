use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CredentialSecret(String);

impl CredentialSecret {
    pub fn generate() -> Self {
        // TODO: generate secret
        CredentialSecret("1234567890".repeat(25) + "abcde")
    }
}

impl TryFrom<&str> for CredentialSecret {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        if s.len() == 255 && s.chars().all(|c| c.is_ascii_alphanumeric()) {
            Ok(CredentialSecret(s.to_owned()))
        } else {
            Err("Invalid format")
        }
    }
}

impl FromStr for CredentialSecret {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Self::try_from(s)
    }
}

impl From<CredentialSecret> for String {
    fn from(secret: CredentialSecret) -> Self {
        secret.0
    }
}

impl std::fmt::Display for CredentialSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
