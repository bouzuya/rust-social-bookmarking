use crate::entity::credential_id::CredentialId;
use crate::entity::credential_verification::CredentialVerification;
use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;

pub struct Credential {
    id: CredentialId,
    mail_address: MailAddress,
    password: Password,
    verification: Option<CredentialVerification>,
}

impl Credential {
    pub fn new(id: CredentialId, mail_address: MailAddress, password: Password) -> Self {
        Self {
            id,
            mail_address,
            password,
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
}
