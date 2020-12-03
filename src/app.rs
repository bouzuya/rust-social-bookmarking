use crate::repository::*;
use crate::service::*;
use crate::use_case::*;
use std::sync::Arc;

pub struct App {
    bookmark_repository: Arc<dyn BookmarkRepository>,
    credential_repository: Arc<dyn CredentialRepository>,
    send_mail_service: Arc<dyn SendMailService>,
    session_service: Arc<dyn SessionService>,
    user_repository: Arc<dyn UserRepository>,
}

impl App {
    pub fn new(
        bookmark_repository: Arc<dyn BookmarkRepository>,
        credential_repository: Arc<dyn CredentialRepository>,
        send_mail_service: Arc<dyn SendMailService>,
        session_service: Arc<dyn SessionService>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            bookmark_repository,
            credential_repository,
            send_mail_service,
            session_service,
            user_repository,
        }
    }

    pub fn create_bookmark_use_case(&self) -> CreateBookmarkUseCase {
        CreateBookmarkUseCase::new(
            self.bookmark_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn create_user_use_case(&self) -> CreateUserUseCase {
        CreateUserUseCase::new(
            self.credential_repository.clone(),
            self.user_repository.clone(),
            self.send_mail_service.clone(),
        )
    }

    pub fn delete_bookmark_use_case(&self) -> DeleteBookmarkUseCase {
        DeleteBookmarkUseCase::new(
            self.bookmark_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn delete_user_use_case(&self) -> DeleteUserUseCase {
        DeleteUserUseCase::new(self.session_service.clone(), self.user_repository.clone())
    }

    pub fn get_current_user_use_case(&self) -> GetCurrentUserUseCase {
        GetCurrentUserUseCase::new(self.session_service.clone())
    }

    pub fn list_bookmarks_by_user_key_use_case(&self) -> ListBookmarksByUserKeyUseCase {
        ListBookmarksByUserKeyUseCase::new(
            self.bookmark_repository.clone(),
            self.user_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn list_current_user_bookmarks_use_case(&self) -> ListCurrentUserBookmarksUseCase {
        ListCurrentUserBookmarksUseCase::new(
            self.bookmark_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn update_bookmark_use_case(&self) -> UpdateBookmarkUseCase {
        UpdateBookmarkUseCase::new(
            self.bookmark_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn update_mail_address_use_case(&self) -> UpdateMailAddressUseCase {
        UpdateMailAddressUseCase::new(
            self.credential_repository.clone(),
            self.session_service.clone(),
            self.send_mail_service.clone(),
        )
    }

    pub fn update_password_use_case(&self) -> UpdatePasswordUseCase {
        UpdatePasswordUseCase::new(
            self.credential_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn update_password_by_secret_use_case(&self) -> UpdatePasswordBySecretUseCase {
        UpdatePasswordBySecretUseCase::new(self.credential_repository.clone())
    }

    pub fn reset_password_use_case(&self) -> ResetPasswordUseCase {
        ResetPasswordUseCase::new(
            self.credential_repository.clone(),
            self.send_mail_service.clone(),
        )
    }

    pub fn sign_in_use_case(&self) -> SignInUseCase {
        SignInUseCase::new(
            self.credential_repository.clone(),
            self.user_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn sign_out_use_case(&self) -> SignOutUseCase {
        SignOutUseCase::new(self.session_service.clone())
    }

    pub fn sign_up_use_case(&self) -> SignUpUseCase {
        SignUpUseCase::new(
            self.credential_repository.clone(),
            self.user_repository.clone(),
            self.send_mail_service.clone(),
        )
    }

    pub fn verify_mail_address_use_case(&self) -> VerifyMailAddressUseCase {
        VerifyMailAddressUseCase::new(self.credential_repository.clone())
    }
}
