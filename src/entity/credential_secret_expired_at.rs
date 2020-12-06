use chrono::prelude::*;
use chrono::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CredentialSecretExpiredAt(DateTime<Utc>);

impl CredentialSecretExpiredAt {
    pub fn new() -> Self {
        Self(Utc::now() + Duration::hours(1))
    }

    pub fn expired(&self) -> bool {
        self.0 < Utc::now()
    }
}

impl From<NaiveDateTime> for CredentialSecretExpiredAt {
    fn from(dt: NaiveDateTime) -> Self {
        Self(DateTime::from_utc(dt, Utc))
    }
}

impl From<CredentialSecretExpiredAt> for NaiveDateTime {
    fn from(expired_at: CredentialSecretExpiredAt) -> Self {
        expired_at.0.naive_utc()
    }
}

impl std::fmt::Display for CredentialSecretExpiredAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let dt = "2020-12-06T22:23:24".parse::<NaiveDateTime>().unwrap();
        let expired_at = CredentialSecretExpiredAt::from(dt);
        assert_eq!("2020-12-06T22:23:24Z".to_owned(), expired_at.to_string());
    }
}
