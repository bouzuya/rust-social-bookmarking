use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CredentialSecret(String);

impl CredentialSecret {
    pub fn generate() -> Self {
        let secret = {
            let mut rng = thread_rng();
            let mut arr = [0u8; 128];
            rng.fill(&mut arr);
            arr
        };
        let hash = {
            let mut hasher = Sha256::new();
            hasher.update(secret);
            hasher.finalize()
        };
        let format = format!("{:X}", hash);
        CredentialSecret(format)
    }
}

impl TryFrom<&str> for CredentialSecret {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        if s.len() == 64 && s.chars().all(|c| c.is_ascii_alphanumeric()) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn generate() {
        let mut set = HashSet::new();
        for _ in 0..100 {
            let key = CredentialSecret::generate();
            assert_eq!(key.0.len(), 64);
            set.insert(key);
        }
        assert_eq!(set.len(), 100);
    }
}
