use crate::entity::credential_id::CredentialId;
use crate::entity::credential_verification::CredentialVerification;
use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::entity::verify_user_secret::VerifyUserSecret;
use anyhow::{anyhow, Result};

pub struct Credential {
    id: CredentialId,
    mail_address: MailAddress,
    password: Password,
    verification: Option<CredentialVerification>,
}

impl Credential {
    pub fn new(id: CredentialId, mail_address: &MailAddress, password: &Password) -> Self {
        Self {
            id,
            mail_address: mail_address.clone(),
            password: password.clone(),
            verification: Some(CredentialVerification::new()),
        }
    }

    pub fn of(
        id: CredentialId,
        mail_address: MailAddress,
        password: Password,
        verification: Option<CredentialVerification>,
    ) -> Self {
        Self {
            id,
            mail_address,
            password,
            verification,
        }
    }

    pub fn id(&self) -> CredentialId {
        self.id
    }

    pub fn mail_address(&self) -> MailAddress {
        self.mail_address.clone()
    }

    pub fn password(&self) -> Password {
        self.password.clone()
    }

    pub fn verification(&self) -> Option<CredentialVerification> {
        self.verification.clone()
    }

    pub fn verify(&self, verify_user_secret: &VerifyUserSecret) -> Result<Self> {
        match &self.verification {
            None => Err(anyhow!("no verification")),
            Some(verification) => {
                verification.verify(verify_user_secret)?;
                Ok(Self {
                    id: self.id(),
                    mail_address: self.mail_address(),
                    password: self.password(),
                    verification: None,
                })
            }
        }
    }
}
