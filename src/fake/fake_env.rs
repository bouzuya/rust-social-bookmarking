use crate::fake::bookmark_repository_impl::BookmarkRepositoryImpl;
use crate::fake::credential_repository_impl::CredentialRepositoryImpl;
use crate::fake::send_mail_service_impl::SendMailServiceImpl;
use crate::fake::session_service_impl::SessionServiceImpl;
use crate::fake::user_repository_impl::UserRepositoryImpl;
use crate::repository::bookmark_repository::UseBookmarkRepository;
use crate::repository::credential_repository::UseCredentialRepository;
use crate::repository::user_repository::UseUserRepository;
use crate::service::send_mail_service::UseSendMailService;
use crate::service::session_service::UseSessionService;
use crate::use_case::create_bookmark_use_case::UseCreateBookmarkUseCase;
use crate::use_case::create_user_use_case::UseCreateUserUseCase;
use crate::use_case::delete_bookmark_use_case::UseDeleteBookmarkUseCase;
use crate::use_case::delete_user_use_case::UseDeleteUserUseCase;
use crate::use_case::get_current_user_use_case::UseGetCurrentUserUseCase;
use crate::use_case::list_bookmarks_use_case::UseListBookmarksUseCase;
use crate::use_case::reset_password_use_case::UseResetPasswordUseCase;
use crate::use_case::sign_in_use_case::UseSignInUseCase;
use crate::use_case::sign_out_use_case::UseSignOutUseCase;
use crate::use_case::sign_up_use_case::UseSignUpUseCase;
use crate::use_case::update_bookmark_use_case::UseUpdateBookmarkUseCase;
use crate::use_case::update_mail_address_use_case::UseUpdateMailAddressUseCase;
use crate::use_case::update_password_by_secret_use_case::UseUpdatePasswordBySecretUseCase;
use crate::use_case::update_password_use_case::UseUpdatePasswordUseCase;
use crate::use_case::verify_mail_address_use_case::UseVerifyMailAddressUseCase;

pub struct FakeEnv {
  bookmark_repository: BookmarkRepositoryImpl,
  credential_repository: CredentialRepositoryImpl,
  send_mail_service: SendMailServiceImpl,
  session_service: SessionServiceImpl,
  user_repository: UserRepositoryImpl,
}

impl FakeEnv {
  pub fn new() -> Self {
    Self {
      bookmark_repository: BookmarkRepositoryImpl::new(),
      credential_repository: CredentialRepositoryImpl::new(),
      send_mail_service: SendMailServiceImpl::new(),
      session_service: SessionServiceImpl::new(),
      user_repository: UserRepositoryImpl::new(),
    }
  }
}

//  repository

impl UseBookmarkRepository for FakeEnv {
  type BookmarkRepository = BookmarkRepositoryImpl;
  fn bookmark_repository(&self) -> &Self::BookmarkRepository {
    &self.bookmark_repository
  }
}

impl UseCredentialRepository for FakeEnv {
  type CredentialRepository = CredentialRepositoryImpl;
  fn credential_repository(&self) -> &Self::CredentialRepository {
    &self.credential_repository
  }
}

impl UseUserRepository for FakeEnv {
  type UserRepository = UserRepositoryImpl;
  fn user_repository(&self) -> &Self::UserRepository {
    &self.user_repository
  }
}

// service

impl UseSendMailService for FakeEnv {
  type SendMailService = SendMailServiceImpl;
  fn send_mail_service(&self) -> &Self::SendMailService {
    &self.send_mail_service
  }
}

impl UseSessionService for FakeEnv {
  type SessionService = SessionServiceImpl;
  fn session_service(&self) -> &Self::SessionService {
    &self.session_service
  }
}

// usecase

impl UseCreateBookmarkUseCase for FakeEnv {
  type CreateBookmarkUseCase = Self;
  fn create_bookmark_use_case(&self) -> &Self::CreateBookmarkUseCase {
    self
  }
}

impl UseCreateUserUseCase for FakeEnv {
  type CreateUserUseCase = Self;
  fn create_user_use_case(&self) -> &Self::CreateUserUseCase {
    self
  }
}

impl UseDeleteBookmarkUseCase for FakeEnv {
  type DeleteBookmarkUseCase = Self;
  fn delete_bookmark_use_case(&self) -> &Self::DeleteBookmarkUseCase {
    self
  }
}

impl UseDeleteUserUseCase for FakeEnv {
  type DeleteUserUseCase = Self;
  fn delete_user_use_case(&self) -> &Self::DeleteUserUseCase {
    self
  }
}

impl UseGetCurrentUserUseCase for FakeEnv {
  type GetCurrentUserUseCase = Self;
  fn get_current_user_use_case(&self) -> &Self::GetCurrentUserUseCase {
    self
  }
}

impl UseListBookmarksUseCase for FakeEnv {
  type ListBookmarksUseCase = Self;
  fn list_bookmarks_use_case(&self) -> &Self::ListBookmarksUseCase {
    self
  }
}

impl UseUpdateBookmarkUseCase for FakeEnv {
  type UpdateBookmarkUseCase = Self;
  fn update_bookmark_use_case(&self) -> &Self::UpdateBookmarkUseCase {
    self
  }
}

impl UseUpdateMailAddressUseCase for FakeEnv {
  type UpdateMailAddressUseCase = Self;
  fn update_mail_address_use_case(&self) -> &Self::UpdateMailAddressUseCase {
    self
  }
}

impl UseUpdatePasswordBySecretUseCase for FakeEnv {
  type UpdatePasswordBySecretUseCase = Self;
  fn update_password_by_secret_use_case(&self) -> &Self::UpdatePasswordBySecretUseCase {
    self
  }
}

impl UseUpdatePasswordUseCase for FakeEnv {
  type UpdatePasswordUseCase = Self;
  fn update_password_use_case(&self) -> &Self::UpdatePasswordUseCase {
    self
  }
}

impl UseResetPasswordUseCase for FakeEnv {
  type ResetPasswordUseCase = Self;
  fn reset_password_use_case(&self) -> &Self::ResetPasswordUseCase {
    self
  }
}

impl UseSignInUseCase for FakeEnv {
  type SignInUseCase = Self;
  fn sign_in_use_case(&self) -> &Self::SignInUseCase {
    self
  }
}

impl UseSignOutUseCase for FakeEnv {
  type SignOutUseCase = Self;
  fn sign_out_use_case(&self) -> &Self::SignOutUseCase {
    self
  }
}

impl UseSignUpUseCase for FakeEnv {
  type SignUpUseCase = Self;
  fn sign_up_use_case(&self) -> &Self::SignUpUseCase {
    self
  }
}

impl UseVerifyMailAddressUseCase for FakeEnv {
  type VerifyMailAddressUseCase = Self;
  fn verify_mail_address_use_case(&self) -> &Self::VerifyMailAddressUseCase {
    self
  }
}
