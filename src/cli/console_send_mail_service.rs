use crate::entity::{Credential, User};
use crate::service::SendMailService;

pub struct ConsoleSendMailService;

impl ConsoleSendMailService {
    pub fn new() -> Self {
        Self
    }
}

impl SendMailService for ConsoleSendMailService {
    fn send_create_user_mail(&self, credential: &Credential) {
        let verification = credential.verification().unwrap();
        println!(
            "メールアドレス ({}) を登録しました。",
            credential.mail_address()
        );
        println!("ユーザーを作成するには次のコマンドを実行してください。");
        println!("$ social-bookmarking create-user {}", verification.secret());
        println!(
            "{} までに実行しない場合は登録したメールアドレスは無効になります。",
            verification.expired_at()
        );
    }

    fn send_update_password_mail(&self, credential: &Credential) {
        let password_reset = credential.password_reset().unwrap();
        println!("パスワードのリセット要求を受け付けました。");
        println!("パスワードを変更するには次のコマンドを実行してください。");
        println!(
            "$ social-bookmarking update-password-by-secret {} <NEW_PASSWORD>",
            password_reset.secret()
        );
        println!(
            "{} までに実行しない場合はリセット要求は無効になります。",
            password_reset.expired_at()
        );
    }

    fn send_user_verified_mail(&self, user: &User, credential: &Credential) {
        println!("ユーザー ({}) を作成しました。", String::from(user.key()));
        println!("サインインするには次のコマンドを実行してください。");
        println!(
            "$ social-bookmarking sign-in {} <YOUR_PASSWORD>",
            credential.mail_address()
        );
    }

    fn send_verify_mail_address_mail(&self, credential: &Credential) {
        println!("メールアドレスの変更要求を受け付けました。");
        println!("メールアドレスを変更するには次のコマンドを実行してください。");
        println!(
            "$ social-bookmarking verify-mail-address {}",
            credential.verification().unwrap().secret()
        );
    }
}
