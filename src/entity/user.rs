use crate::entity::mail_address::MailAddress;
use crate::entity::password::Password;
use crate::entity::user_id::UserId;
use crate::entity::user_key::UserKey;
use crate::entity::verify_user_secret::VerifyUserSecret;
use anyhow::{anyhow, Result};

#[derive(Clone)]
pub struct User {
    pub id: UserId,
    pub key: UserKey,
    pub mail_address: MailAddress,
    pub password: Password,
    pub verify_user_secret: Option<VerifyUserSecret>,
}

impl User {
    pub fn new(id: UserId, mail_address: MailAddress, password: Password) -> Self {
        Self {
            id,
            key: UserKey::generate(),
            mail_address,
            password,
            verify_user_secret: Some(VerifyUserSecret::generate()),
        }
    }

    pub fn of(
        id: UserId,
        key: UserKey,
        mail_address: MailAddress,
        password: Password,
        verify_user_secret: Option<VerifyUserSecret>,
    ) -> Self {
        Self {
            id,
            key,
            mail_address,
            password,
            verify_user_secret,
        }
    }

    pub fn verify(&self, verify_user_secret: &VerifyUserSecret) -> Result<Self> {
        match &self.verify_user_secret {
            None => Err(anyhow!("no verify_user_secret")),
            Some(secret) if secret != verify_user_secret => {
                Err(anyhow!("verify_user_secret does not match"))
            }
            Some(_) => Ok(Self {
                id: self.id.clone(),
                key: self.key.clone(),
                mail_address: self.mail_address.clone(),
                password: self.password.clone(),
                verify_user_secret: None,
            }),
        }
    }
}
