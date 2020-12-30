use std::{fs::read_to_string, fs::remove_file, fs::write, path::Path, sync::Arc};

use crate::service::SessionService;
use crate::{entity::User, repository::UserRepository};
use anyhow::Result;

pub struct FsSessionService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl FsSessionService {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }
}

impl SessionService for FsSessionService {
    fn get_current_user(&self) -> Result<Option<User>> {
        let path = Path::new(".rust-social-bookmarking-session");
        if path.is_file() {
            let key = read_to_string(path).map_err(anyhow::Error::msg)?;
            let key = key.parse().map_err(anyhow::Error::msg)?;
            self.user_repository.find_by_user_key(&key)
        } else {
            Ok(None)
        }
    }

    fn set_current_user(&self, user: Option<User>) -> Result<()> {
        let path = Path::new(".rust-social-bookmarking-session");
        match user {
            None => {
                if path.is_file() {
                    remove_file(path).map_err(anyhow::Error::msg)
                } else {
                    Ok(())
                }
            }
            Some(user) => write(path, String::from(user.key())).map_err(anyhow::Error::msg),
        }
    }
}
