use crate::entity::credential_secret::CredentialSecret;
use crate::entity::credential_secret_expired_at::CredentialSecretExpiredAt;
use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct CredentialSecretWithExpiration {
    expired_at: CredentialSecretExpiredAt,
    secret: CredentialSecret,
}

impl CredentialSecretWithExpiration {
    pub fn new() -> Self {
        Self {
            expired_at: CredentialSecretExpiredAt::new(),
            secret: CredentialSecret::generate(),
        }
    }

    pub fn of(expired_at: CredentialSecretExpiredAt, secret: CredentialSecret) -> Self {
        Self { expired_at, secret }
    }

    pub fn expired(&self) -> bool {
        self.expired_at.expired()
    }

    pub fn expired_at(&self) -> CredentialSecretExpiredAt {
        self.expired_at
    }

    pub fn secret(&self) -> CredentialSecret {
        self.secret.clone()
    }

    pub fn verify(&self, secret: &CredentialSecret) -> Result<()> {
        if self.expired() {
            return Err(anyhow!("verification expired"));
        }
        if &self.secret != secret {
            return Err(anyhow!("credential_secret does not match"));
        }
        Ok(())
    }
}
