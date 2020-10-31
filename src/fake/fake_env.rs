use crate::fake::send_mail_service_impl::SendMailServiceImpl;
use crate::fake::user_repository_impl::UserRepositoryImpl;
use crate::repository::user_repository::UseUserRepository;
use crate::service::send_mail_service::UseSendMailService;
use crate::use_case::create_user_use_case::UseCreateUserUseCase;
use crate::use_case::verify_user_use_case::UseVerifyUserUseCase;

pub struct FakeEnv {
  send_mail_service: SendMailServiceImpl,
  user_repository: UserRepositoryImpl,
}

impl FakeEnv {
  pub fn new() -> Self {
    Self {
      send_mail_service: SendMailServiceImpl::new(),
      user_repository: UserRepositoryImpl::new(),
    }
  }
}

//  repository

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

// usecase

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
