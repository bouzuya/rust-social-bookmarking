use chrono::prelude::*;
use chrono::Duration;

#[derive(Clone, Copy, Debug)]
pub struct CredentialVerificationExpiredAt(DateTime<Utc>);

impl CredentialVerificationExpiredAt {
    pub fn new() -> Self {
        Self(Utc::now() + Duration::hours(1))
    }
}

impl From<NaiveDateTime> for CredentialVerificationExpiredAt {
    fn from(dt: NaiveDateTime) -> Self {
        Self(DateTime::from_utc(dt, Utc))
    }
}

impl From<CredentialVerificationExpiredAt> for NaiveDateTime {
    fn from(expired_at: CredentialVerificationExpiredAt) -> Self {
        expired_at.0.naive_utc()
    }
}
