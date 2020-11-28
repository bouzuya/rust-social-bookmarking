use crate::entity::credential_id::CredentialId;
use crate::entity::credential_secret::CredentialSecret;
use crate::entity::credential_secret_with_expiration::CredentialSecretWithExpiration;
use crate::entity::credential_status::CredentialStatus;
use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::entity::user_id::UserId;
use crate::entity::CredentialVerifiedAt;
use anyhow::{anyhow, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Credential {
    id: CredentialId,
    user_id: UserId,
    mail_address: MailAddress,
    password: Password,
    password_reset: Option<CredentialSecretWithExpiration>,
    status: CredentialStatus,
}

impl Credential {
    pub fn new(
        id: CredentialId,
        user_id: UserId,
        mail_address: &MailAddress,
        password: &Password,
    ) -> Self {
        Self {
            id,
            user_id,
            mail_address: mail_address.clone(),
            password: password.clone(),
            password_reset: None,
            status: CredentialStatus::WaitingForVerification(CredentialSecretWithExpiration::new()),
        }
    }

    pub fn of(
        id: CredentialId,
        user_id: UserId,
        mail_address: MailAddress,
        password: Password,
        password_reset: Option<CredentialSecretWithExpiration>,
        status: CredentialStatus,
    ) -> Self {
        Self {
            id,
            user_id,
            mail_address,
            password,
            password_reset,
            status,
        }
    }

    pub fn id(&self) -> CredentialId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn mail_address(&self) -> MailAddress {
        self.mail_address.clone()
    }

    pub fn password(&self) -> Password {
        self.password.clone()
    }

    pub fn password_reset(&self) -> Option<CredentialSecretWithExpiration> {
        self.password_reset.clone()
    }

    pub fn status(&self) -> CredentialStatus {
        self.status.clone()
    }

    pub fn verification(&self) -> Option<CredentialSecretWithExpiration> {
        match &self.status {
            CredentialStatus::WaitingForVerification(verification) => Some(verification.clone()),
            CredentialStatus::Verified(_) => None,
        }
    }

    pub fn reset_password(&self) -> Result<Self> {
        match &self.status {
            CredentialStatus::WaitingForVerification(_) => Err(anyhow!("invalid status")),
            CredentialStatus::Verified(_) => Ok(Self {
                id: self.id(),
                user_id: self.user_id(),
                mail_address: self.mail_address(),
                password: self.password(),
                password_reset: Some(CredentialSecretWithExpiration::new()),
                status: self.status(),
            }),
        }
    }

    pub fn update_password(&self, password: &Password) -> Result<Self> {
        match self.status {
            CredentialStatus::WaitingForVerification(_) => Err(anyhow!("invalid status")),
            CredentialStatus::Verified(_) => Ok(Self {
                id: self.id(),
                user_id: self.user_id(),
                mail_address: self.mail_address(),
                password: password.clone(),
                password_reset: None,
                status: self.status(),
            }),
        }
    }

    pub fn verify(&self, secret: &CredentialSecret) -> Result<Self> {
        match &self.status {
            CredentialStatus::Verified(_) => Err(anyhow!("invalid status")),
            CredentialStatus::WaitingForVerification(verification) => {
                verification.verify(secret)?;
                Ok(Self {
                    id: self.id(),
                    user_id: self.user_id(),
                    mail_address: self.mail_address(),
                    password: self.password(),
                    password_reset: self.password_reset(),
                    status: CredentialStatus::Verified(CredentialVerifiedAt::new()),
                })
            }
        }
    }
}
