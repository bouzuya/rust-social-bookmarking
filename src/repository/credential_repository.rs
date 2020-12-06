use crate::entity::{Credential, CredentialId, CredentialSecret, MailAddress, Password, UserId};
use anyhow::Result;

pub trait UseCredentialRepository {
    type CredentialRepository: CredentialRepository;
    fn credential_repository(&self) -> &Self::CredentialRepository;
}

pub trait CredentialRepository {
    fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Credential>>;
    fn find_by_mail_address(&self, mail_address: &MailAddress) -> Result<Option<Credential>>;
    fn find_by_password_reset_secret(
        &self,
        secret: &CredentialSecret,
    ) -> Result<Option<Credential>>;
    fn find_by_verification_secret(&self, secret: &CredentialSecret) -> Result<Option<Credential>>;
    fn create(
        &self,
        user_id: UserId,
        mail_address: &MailAddress,
        password: &Password,
    ) -> Result<Credential>;
    fn delete(&self, credential_id: &CredentialId) -> Result<()>;
    fn delete_by_user_id(&self, user_id: &UserId) -> Result<()>;
    fn save(&self, credential: &Credential) -> Result<()>;
}
