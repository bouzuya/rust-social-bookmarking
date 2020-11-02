use anyhow::Result;

pub trait UseSignOutUseCase {
    type SignOutUseCase: SignOutUseCase;
    fn sign_out_use_case(&self) -> &Self::SignOutUseCase;
}

pub trait SignOutUseCase {
    fn sign_out(&self) -> Result<()> {
        todo!()
    }
}

impl<T> SignOutUseCase for T {}
