use crate::entity::credential_verification_expired_at::CredentialVerificationExpiredAt;
use crate::entity::verify_user_secret::VerifyUserSecret;

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

    pub fn expired_at(&self) -> CredentialVerificationExpiredAt {
        self.expired_at
    }

    pub fn secret(&self) -> VerifyUserSecret {
        self.secret.clone()
    }
}
