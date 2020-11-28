use chrono::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CredentialVerifiedAt(DateTime<Utc>);

impl CredentialVerifiedAt {
    pub fn new() -> Self {
        Self(Utc::now())
    }
}

impl From<NaiveDateTime> for CredentialVerifiedAt {
    fn from(dt: NaiveDateTime) -> Self {
        Self(DateTime::from_utc(dt, Utc))
    }
}

impl From<CredentialVerifiedAt> for NaiveDateTime {
    fn from(verified_at: CredentialVerifiedAt) -> Self {
        verified_at.0.naive_utc()
    }
}
