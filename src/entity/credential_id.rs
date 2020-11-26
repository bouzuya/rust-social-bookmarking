use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CredentialId(i32);

impl TryFrom<i32> for CredentialId {
    type Error = &'static str;
    fn try_from(i: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        if i >= 1 {
            Ok(CredentialId(i))
        } else {
            Err("CredentialId >= 1")
        }
    }
}

impl From<CredentialId> for i32 {
    fn from(id: CredentialId) -> Self {
        id.0
    }
}

impl std::fmt::Display for CredentialId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        use std::convert::TryInto;
        let credential_id: CredentialId = 1_i32.try_into().unwrap();
        assert_eq!("1".to_owned(), credential_id.to_string());
        assert_eq!("1".to_owned(), format!("{}", credential_id));
    }
}
