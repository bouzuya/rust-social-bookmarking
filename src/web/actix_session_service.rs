use crate::service::SessionService;
use crate::{entity::User, repository::UserRepository};
use actix_session::Session;
use anyhow::Result;
use std::sync::Arc;

pub struct ActixSessionService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    session: Session,
}

impl ActixSessionService {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>, session: Session) -> Self {
        Self {
            user_repository,
            session,
        }
    }
}

impl SessionService for ActixSessionService {
    fn get_current_user(&self) -> Result<Option<User>> {
        let key = "session";
        match self
            .session
            .get::<String>(key)
            .map_err(|_| anyhow::Error::msg("no session value"))?
        {
            None => return Ok(None),
            Some(user_key) => {
                let user_key = user_key.parse().map_err(anyhow::Error::msg)?;
                self.user_repository.find_by_user_key(&user_key)
            }
        }
    }

    fn set_current_user(&self, user: Option<User>) -> Result<()> {
        let key = "session";
        match user {
            None => self.session.remove(key),
            Some(user) => {
                self.session
                    .set(key, user.key().to_string())
                    .map_err(|_| anyhow::Error::msg("session set failed"))?;
            }
        }
        Ok(())
    }
}
