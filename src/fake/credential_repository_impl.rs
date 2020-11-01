use crate::entity::credential::Credential;
use crate::entity::credential_id::CredentialId;
use crate::entity::credential_verification::CredentialVerification;
use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::repository::credential_repository::CredentialRepository;
use anyhow::Result;
use std::convert::TryInto;

pub struct CredentialRepositoryImpl;

impl CredentialRepositoryImpl {
  pub fn new() -> Self {
    Self
  }
}

impl CredentialRepository for CredentialRepositoryImpl {
  fn create(&self, mail_address: MailAddress, password: Password) -> Result<Credential> {
    println!("CredentialRepository#create");
    let credential_id = 1.try_into().unwrap();
    let credential = Credential::new(credential_id, mail_address, password);
    Ok(credential)
  }

  fn find_by_mail_address(&self, mail_address: &MailAddress) -> Result<Option<Credential>> {
    println!("CredentialRepository#find_by_mail_address");
    let credential_id = 1.try_into().unwrap();
    let mail_address = mail_address.clone();
    let password = "password".parse().unwrap();
    let verification = Some(CredentialVerification::new());
    let credential = Credential::of(credential_id, mail_address, password, verification);
    Ok(Some(credential))
  }

  fn delete(&self, _: &CredentialId) -> Result<()> {
    println!("CredentialRepository#delete");
    Ok(())
  }
}
