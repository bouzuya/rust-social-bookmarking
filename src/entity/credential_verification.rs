use crate::entity::credential_verification_expired_at::CredentialVerificationExpiredAt;
use crate::entity::verify_user_secret::VerifyUserSecret;
use anyhow::{anyhow, Result};

#[derive(Clone)]
pub struct CredentialVerification {
    expired_at: CredentialVerificationExpiredAt,
    secret: VerifyUserSecret, // TODO: rename
}

impl CredentialVerification {
    pub fn new() -> Self {
        Self {
            expired_at: CredentialVerificationExpiredAt::new(),
            secret: VerifyUserSecret::generate(),
        }
    }

    pub fn of(expired_at: CredentialVerificationExpiredAt, secret: VerifyUserSecret) -> Self {
        Self { expired_at, secret }
    }

    pub fn expired(&self) -> bool {
        self.expired_at.expired()
    }

    pub fn expired_at(&self) -> CredentialVerificationExpiredAt {
        self.expired_at
    }

    pub fn secret(&self) -> VerifyUserSecret {
        self.secret.clone()
    }

    pub fn verify(&self, verify_user_secret: &VerifyUserSecret) -> Result<()> {
        if self.expired() {
            return Err(anyhow!("verification expired"));
        }
        if &self.secret != verify_user_secret {
            return Err(anyhow!("verify_user_secret does not match"));
        }
        Ok(())
    }
}
