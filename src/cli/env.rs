use crate::fake::send_mail_service_impl::SendMailServiceImpl;
use crate::fake::session_service_impl::SessionServiceImpl;
use crate::pg::*;
use crate::repository::*;
use crate::service::*;
use crate::use_case::*;
use diesel::{Connection, PgConnection};
use std::sync::Arc;

pub struct CliEnv {
    bookmark_repository: PgBookmarkRepository,
    credential_repository: PgCredentialRepository,
    send_mail_service: SendMailServiceImpl,
    session_service: SessionServiceImpl,
    user_repository: PgUserRepository,
}

impl CliEnv {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        let connection = Arc::new(connection);
        Self {
            bookmark_repository: PgBookmarkRepository::new(connection.clone()),
            credential_repository: PgCredentialRepository::new(connection.clone()),
            send_mail_service: SendMailServiceImpl::new(),
            session_service: SessionServiceImpl::new(),
            user_repository: PgUserRepository::new(connection.clone()),
        }
    }
}

//  repository

impl UseBookmarkRepository for CliEnv {
    type BookmarkRepository = PgBookmarkRepository;
    fn bookmark_repository(&self) -> &Self::BookmarkRepository {
        &self.bookmark_repository
    }
}

impl UseCredentialRepository for CliEnv {
    type CredentialRepository = PgCredentialRepository;
    fn credential_repository(&self) -> &Self::CredentialRepository {
        &self.credential_repository
    }
}

impl UseUserRepository for CliEnv {
    type UserRepository = PgUserRepository;
    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

// service

impl UseSendMailService for CliEnv {
    type SendMailService = SendMailServiceImpl;
    fn send_mail_service(&self) -> &Self::SendMailService {
        &self.send_mail_service
    }
}

impl UseSessionService for CliEnv {
    type SessionService = SessionServiceImpl;
    fn session_service(&self) -> &Self::SessionService {
        &self.session_service
    }
}

// usecase

impl UseCreateBookmarkUseCase for CliEnv {
    type CreateBookmarkUseCase = Self;
    fn create_bookmark_use_case(&self) -> &Self::CreateBookmarkUseCase {
        self
    }
}

impl UseCreateUserUseCase for CliEnv {
    type CreateUserUseCase = Self;
    fn create_user_use_case(&self) -> &Self::CreateUserUseCase {
        self
    }
}

impl UseDeleteBookmarkUseCase for CliEnv {
    type DeleteBookmarkUseCase = Self;
    fn delete_bookmark_use_case(&self) -> &Self::DeleteBookmarkUseCase {
        self
    }
}

impl UseDeleteUserUseCase for CliEnv {
    type DeleteUserUseCase = Self;
    fn delete_user_use_case(&self) -> &Self::DeleteUserUseCase {
        self
    }
}

impl UseGetCurrentUserUseCase for CliEnv {
    type GetCurrentUserUseCase = Self;
    fn get_current_user_use_case(&self) -> &Self::GetCurrentUserUseCase {
        self
    }
}

impl UseListBookmarksByUserKeyUseCase for CliEnv {
    type ListBookmarksByUserKeyUseCase = Self;
    fn list_bookmarks_by_user_key_use_case(&self) -> &Self::ListBookmarksByUserKeyUseCase {
        self
    }
}

impl UseListCurrentUserBookmarksUseCase for CliEnv {
    type ListBookmarksUseCase = Self;
    fn list_current_user_bookmarks_use_case(&self) -> &Self::ListBookmarksUseCase {
        self
    }
}

impl UseUpdateBookmarkUseCase for CliEnv {
    type UpdateBookmarkUseCase = Self;
    fn update_bookmark_use_case(&self) -> &Self::UpdateBookmarkUseCase {
        self
    }
}

impl UseUpdateMailAddressUseCase for CliEnv {
    type UpdateMailAddressUseCase = Self;
    fn update_mail_address_use_case(&self) -> &Self::UpdateMailAddressUseCase {
        self
    }
}

impl UseUpdatePasswordBySecretUseCase for CliEnv {
    type UpdatePasswordBySecretUseCase = Self;
    fn update_password_by_secret_use_case(&self) -> &Self::UpdatePasswordBySecretUseCase {
        self
    }
}

impl UseUpdatePasswordUseCase for CliEnv {
    type UpdatePasswordUseCase = Self;
    fn update_password_use_case(&self) -> &Self::UpdatePasswordUseCase {
        self
    }
}

impl UseResetPasswordUseCase for CliEnv {
    type ResetPasswordUseCase = Self;
    fn reset_password_use_case(&self) -> &Self::ResetPasswordUseCase {
        self
    }
}

impl UseSignInUseCase for CliEnv {
    type SignInUseCase = Self;
    fn sign_in_use_case(&self) -> &Self::SignInUseCase {
        self
    }
}

impl UseSignOutUseCase for CliEnv {
    type SignOutUseCase = Self;
    fn sign_out_use_case(&self) -> &Self::SignOutUseCase {
        self
    }
}

impl UseSignUpUseCase for CliEnv {
    type SignUpUseCase = Self;
    fn sign_up_use_case(&self) -> &Self::SignUpUseCase {
        self
    }
}

impl UseVerifyMailAddressUseCase for CliEnv {
    type VerifyMailAddressUseCase = Self;
    fn verify_mail_address_use_case(&self) -> &Self::VerifyMailAddressUseCase {
        self
    }
}
