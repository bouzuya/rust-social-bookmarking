use crate::fake::bookmark_repository_impl::BookmarkRepositoryImpl;
use crate::fake::send_mail_service_impl::SendMailServiceImpl;
use crate::fake::session_service_impl::SessionServiceImpl;
use crate::fake::user_repository_impl::UserRepositoryImpl;
use crate::repository::bookmark_repository::UseBookmarkRepository;
use crate::repository::user_repository::UseUserRepository;
use crate::service::send_mail_service::UseSendMailService;
use crate::service::session_service::UseSessionService;
use crate::use_case::create_bookmark_use_case::UseCreateBookmarkUseCase;
use crate::use_case::create_user_use_case::UseCreateUserUseCase;
use crate::use_case::verify_user_use_case::UseVerifyUserUseCase;

pub struct FakeEnv {
  bookmark_repository: BookmarkRepositoryImpl,
  send_mail_service: SendMailServiceImpl,
  session_service: SessionServiceImpl,
  user_repository: UserRepositoryImpl,
}

impl FakeEnv {
  pub fn new() -> Self {
    Self {
      bookmark_repository: BookmarkRepositoryImpl::new(),
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

impl UseVerifyUserUseCase for FakeEnv {
  type VerifyUserUseCase = Self;
  fn verify_user_use_case(&self) -> &Self::VerifyUserUseCase {
    self
  }
}
