use crate::entity::credential::Credential;
use crate::entity::credential_id::CredentialId;
use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use anyhow::Result;

pub trait UseCredentialRepository {
  type CredentialRepository: CredentialRepository;
  fn credential_repository(&self) -> &Self::CredentialRepository;
}

pub trait CredentialRepository {
  fn find_by_mail_address(&self, mail_address: &MailAddress) -> Result<Option<Credential>>;
  fn create(&self, mail_address: MailAddress, password: Password) -> Result<Credential>;
  fn delete(&self, credential_id: &CredentialId) -> Result<()>;
}
