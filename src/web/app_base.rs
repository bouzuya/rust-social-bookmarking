use crate::repository::UserRepository;
use crate::{
    repository::{BookmarkRepository, CredentialRepository},
    service::SendMailService,
};
use std::sync::Arc;

pub struct AppBase {
    pub bookmark_repository: Arc<dyn BookmarkRepository + Send + Sync>,
    pub credential_repository: Arc<dyn CredentialRepository + Send + Sync>,
    pub send_mail_service: Arc<dyn SendMailService + Send + Sync>,
    pub user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl AppBase {
    pub fn new(
        bookmark_repository: Arc<dyn BookmarkRepository + Send + Sync>,
        credential_repository: Arc<dyn CredentialRepository + Send + Sync>,
        send_mail_service: Arc<dyn SendMailService + Send + Sync>,
        user_repository: Arc<dyn UserRepository + Send + Sync>,
    ) -> Self {
        Self {
            bookmark_repository,
            credential_repository,
            send_mail_service,
            user_repository,
        }
    }
}
