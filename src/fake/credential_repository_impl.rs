use crate::entity::{Credential, CredentialId, CredentialSecret, MailAddress, Password, UserId};
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

    fn find_by_mail_address(&self, _: &MailAddress) -> Result<Option<Credential>> {
        todo!()
    }

    fn find_by_password_reset_secret(&self, _: &CredentialSecret) -> Result<Option<Credential>> {
        todo!()
    }

    fn find_by_verification_secret(&self, _: &CredentialSecret) -> Result<Option<Credential>> {
        todo!()
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
