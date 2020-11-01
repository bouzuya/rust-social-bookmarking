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
