use anyhow::Result;

pub trait UseGetCurrentUserUseCase {
    type GetCurrentUserUseCase: GetCurrentUserUseCase;
    fn get_current_user_use_case(&self) -> &Self::GetCurrentUserUseCase;
}

pub trait GetCurrentUserUseCase {
    fn get_current_user(&self) -> Result<()> {
        todo!()
    }
}

impl<T> GetCurrentUserUseCase for T {}
