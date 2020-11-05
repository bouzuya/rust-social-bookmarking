use crate::entity::credential_id::CredentialId;
use crate::entity::credential_status::CredentialStatus;
use crate::entity::credential_verification::CredentialVerification;
use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::entity::user_id::UserId;
use crate::entity::verify_user_secret::VerifyUserSecret;
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Credential {
    id: CredentialId,
    user_id: UserId,
    mail_address: MailAddress,
    password: Password,
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
            status: CredentialStatus::WaitingForVerification(CredentialVerification::new()),
        }
    }

    pub fn of(
        id: CredentialId,
        user_id: UserId,
        mail_address: MailAddress,
        password: Password,
        status: CredentialStatus,
    ) -> Self {
        Self {
            id,
            user_id,
            mail_address,
            password,
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

    pub fn verification(&self) -> Option<CredentialVerification> {
        match &self.status {
            CredentialStatus::WaitingForVerification(verification) => Some(verification.clone()),
            CredentialStatus::WaitingForDeletion
            | CredentialStatus::Verified
            | CredentialStatus::ResettingPassword(_) => None,
        }
    }

    pub fn reset_password(&self) -> Result<Self> {
        match &self.status {
            CredentialStatus::WaitingForVerification(_) | CredentialStatus::WaitingForDeletion => {
                Err(anyhow!("invalid status"))
            }
            CredentialStatus::Verified | CredentialStatus::ResettingPassword(_) => Ok(Self {
                id: self.id(),
                user_id: self.user_id(),
                mail_address: self.mail_address(),
                password: self.password(),
                status: CredentialStatus::ResettingPassword(CredentialVerification::new()),
            }),
        }
    }

    pub fn update_password(&self, password: &Password) -> Result<Self> {
        match self.status {
            CredentialStatus::WaitingForVerification(_) | CredentialStatus::WaitingForDeletion => {
                Err(anyhow!("invalid status"))
            }
            CredentialStatus::Verified | CredentialStatus::ResettingPassword(_) => Ok(Self {
                id: self.id(),
                user_id: self.user_id(),
                mail_address: self.mail_address(),
                password: password.clone(),
                status: CredentialStatus::Verified,
            }),
        }
    }

    pub fn verify(&self, verify_user_secret: &VerifyUserSecret) -> Result<Self> {
        match &self.status {
            CredentialStatus::WaitingForDeletion
            | CredentialStatus::Verified
            | CredentialStatus::ResettingPassword(_) => Err(anyhow!("invalid status")),
            CredentialStatus::WaitingForVerification(verification) => {
                verification.verify(verify_user_secret)?;
                Ok(Self {
                    id: self.id(),
                    user_id: self.user_id(),
                    mail_address: self.mail_address(),
                    password: self.password(),
                    status: CredentialStatus::Verified,
                })
            }
        }
    }
}
