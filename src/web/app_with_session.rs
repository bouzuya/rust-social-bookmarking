use super::{app_base::AppBase, ActixSessionService};
use crate::use_case::{
    CreateBookmarkUseCase, CreateUserUseCase, DeleteBookmarkUseCase, DeleteUserUseCase,
    GetCurrentUserUseCase, ListBookmarksByUserKeyUseCase, ListCurrentUserBookmarksUseCase,
    ResetPasswordUseCase, SignInUseCase, SignOutUseCase, SignUpUseCase, UpdateBookmarkUseCase,
    UpdateMailAddressUseCase, UpdatePasswordBySecretUseCase, UpdatePasswordUseCase,
    VerifyMailAddressUseCase,
};
use actix_session::Session;
use actix_web::web::Data;
use std::sync::Arc;

pub struct AppWithSession {
    app: Data<AppBase>,
    session_service: Arc<ActixSessionService>,
}

impl AppWithSession {
    pub fn new(app: Data<AppBase>, session: Session) -> Self {
        let session_service = Arc::new(ActixSessionService::new(
            app.user_repository.clone(),
            session,
        ));
        Self {
            app,
            session_service,
        }
    }

    pub fn create_bookmark_use_case(&self) -> CreateBookmarkUseCase {
        CreateBookmarkUseCase::new(
            self.app.bookmark_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn create_user_use_case(&self) -> CreateUserUseCase {
        CreateUserUseCase::new(
            self.app.credential_repository.clone(),
            self.app.user_repository.clone(),
            self.app.send_mail_service.clone(),
        )
    }

    pub fn delete_bookmark_use_case(&self) -> DeleteBookmarkUseCase {
        DeleteBookmarkUseCase::new(
            self.app.bookmark_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn delete_user_use_case(&self) -> DeleteUserUseCase {
        DeleteUserUseCase::new(
            self.session_service.clone(),
            self.app.bookmark_repository.clone(),
            self.app.credential_repository.clone(),
            self.app.user_repository.clone(),
        )
    }

    pub fn get_current_user_use_case(&self) -> GetCurrentUserUseCase {
        GetCurrentUserUseCase::new(self.session_service.clone())
    }

    pub fn list_bookmarks_by_user_key_use_case(&self) -> ListBookmarksByUserKeyUseCase {
        ListBookmarksByUserKeyUseCase::new(
            self.app.bookmark_repository.clone(),
            self.app.user_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn list_current_user_bookmarks_use_case(&self) -> ListCurrentUserBookmarksUseCase {
        ListCurrentUserBookmarksUseCase::new(
            self.app.bookmark_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn update_bookmark_use_case(&self) -> UpdateBookmarkUseCase {
        UpdateBookmarkUseCase::new(
            self.app.bookmark_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn update_mail_address_use_case(&self) -> UpdateMailAddressUseCase {
        UpdateMailAddressUseCase::new(
            self.app.credential_repository.clone(),
            self.session_service.clone(),
            self.app.send_mail_service.clone(),
        )
    }

    pub fn update_password_use_case(&self) -> UpdatePasswordUseCase {
        UpdatePasswordUseCase::new(
            self.app.credential_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn update_password_by_secret_use_case(&self) -> UpdatePasswordBySecretUseCase {
        UpdatePasswordBySecretUseCase::new(self.app.credential_repository.clone())
    }

    pub fn reset_password_use_case(&self) -> ResetPasswordUseCase {
        ResetPasswordUseCase::new(
            self.app.credential_repository.clone(),
            self.app.send_mail_service.clone(),
        )
    }

    pub fn sign_in_use_case(&self) -> SignInUseCase {
        SignInUseCase::new(
            self.app.credential_repository.clone(),
            self.app.user_repository.clone(),
            self.session_service.clone(),
        )
    }

    pub fn sign_out_use_case(&self) -> SignOutUseCase {
        SignOutUseCase::new(self.session_service.clone())
    }

    pub fn sign_up_use_case(&self) -> SignUpUseCase {
        SignUpUseCase::new(
            self.app.credential_repository.clone(),
            self.app.user_repository.clone(),
            self.app.send_mail_service.clone(),
        )
    }

    pub fn verify_mail_address_use_case(&self) -> VerifyMailAddressUseCase {
        VerifyMailAddressUseCase::new(self.app.credential_repository.clone())
    }
}
