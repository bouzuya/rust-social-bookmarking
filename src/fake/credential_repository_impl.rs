use crate::entity::credential::Credential;
use crate::entity::credential_id::CredentialId;
use crate::entity::credential_verification::CredentialVerification;
use crate::entity::credential_verification_expired_at::CredentialVerificationExpiredAt;
use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::entity::user_id::UserId;
use crate::entity::verify_user_secret::VerifyUserSecret;
use crate::repository::CredentialRepository;
use anyhow::Result;
use std::convert::TryInto;

pub struct CredentialRepositoryImpl;

impl CredentialRepositoryImpl {
  pub fn new() -> Self {
    Self
  }
}

impl CredentialRepository for CredentialRepositoryImpl {
  fn create(
    &self,
    user_id: UserId,
    mail_address: &MailAddress,
    password: &Password,
  ) -> Result<Credential> {
    println!("CredentialRepository#create");
    let credential_id = 1.try_into().unwrap();
    let credential = Credential::new(credential_id, user_id, mail_address, password);
    Ok(credential)
  }

  fn find_by_user_id(&self, _: &UserId) -> Result<Vec<Credential>> {
    todo!()
  }

  fn find_by_mail_address(&self, mail_address: &MailAddress) -> Result<Option<Credential>> {
    println!("CredentialRepository#find_by_mail_address");
    let credential_id = 1.try_into().unwrap();
    let user_id = 1.try_into().unwrap();
    let mail_address = mail_address.clone();
    let password = "password".parse().unwrap();
    let verification = Some(CredentialVerification::new());
    let credential = Credential::of(credential_id, user_id, mail_address, password, verification);
    Ok(Some(credential))
  }

  fn find_by_verify_user_secret(
    &self,
    verify_user_secret: &VerifyUserSecret,
  ) -> Result<Option<Credential>> {
    println!("CredentialRepository#find_by_verify_user_secret");
    let credential_id = 1.try_into().unwrap();
    let user_id = 1.try_into().unwrap();
    let mail_address = "m@bouzuya.net".parse().unwrap();
    let password = "password".parse().unwrap();
    let verification = Some(CredentialVerification::of(
      CredentialVerificationExpiredAt::new(),
      verify_user_secret.clone(),
    ));
    let credential = Credential::of(credential_id, user_id, mail_address, password, verification);
    Ok(Some(credential))
  }

  fn delete(&self, _: &CredentialId) -> Result<()> {
    println!("CredentialRepository#delete");
    Ok(())
  }

  fn save(&self, _: &Credential) -> Result<()> {
    println!("CredentialRepository#save");
    Ok(())
  }
}
