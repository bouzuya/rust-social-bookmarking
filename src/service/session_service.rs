use crate::entity::User;
use anyhow::Result;

pub trait SessionService {
    fn get_current_user(&self) -> Result<Option<User>>;
    fn set_current_user(&self, user: Option<User>) -> Result<()>;
}
