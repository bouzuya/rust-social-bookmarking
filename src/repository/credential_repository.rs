use crate::entity::credential::Credential;
use crate::entity::credential_id::CredentialId;
use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::entity::user_id::UserId;
use crate::entity::verify_user_secret::VerifyUserSecret;
use anyhow::Result;

pub trait UseCredentialRepository {
  type CredentialRepository: CredentialRepository;
  fn credential_repository(&self) -> &Self::CredentialRepository;
}

pub trait CredentialRepository {
  fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Credential>>;
  fn find_by_mail_address(&self, mail_address: &MailAddress) -> Result<Option<Credential>>;
  fn find_by_verify_user_secret(
    &self,
    verify_user_secret: &VerifyUserSecret,
  ) -> Result<Option<Credential>>;
  fn create(
    &self,
    user_id: UserId,
    mail_address: &MailAddress,
    password: &Password,
  ) -> Result<Credential>;
  fn delete(&self, credential_id: &CredentialId) -> Result<()>;
  fn save(&self, credential: Credential) -> Result<()>;
}
