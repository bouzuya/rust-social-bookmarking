use crate::entity::{Credential, User};
use crate::service::SendMailService;
use lettre::{SendableEmail, SmtpClient, Transport};
use lettre_email::Email;

struct SimpleEmail {
    subject: String,
    text: String,
    to: String,
}

fn send(simple_email: SimpleEmail) {
    let email: SendableEmail = Email::builder()
        .to(simple_email.to)
        .from("m@bouzuya.net")
        .subject(&simple_email.subject)
        .text(&simple_email.text)
        .build()
        .unwrap()
        .into();
    let smtp_client = SmtpClient::new_unencrypted_localhost().unwrap();
    let mut smtp_transport = smtp_client.transport();
    match smtp_transport.send(email) {
        Ok(res) => println!("Ok: {:?}", res),
        Err(err) => println!("Err: {:?}", err),
    }
}

pub struct SmtpSendMailService;

impl SmtpSendMailService {
    pub fn new() -> Self {
        Self
    }
}

impl SendMailService for SmtpSendMailService {
    fn send_create_user_mail(&self, credential: &Credential) {
        let verification = credential.verification().unwrap();
        let text = format!(
            r#"メールアドレス ({}) を登録しました。
ユーザーを作成するには次のコマンドを実行してください。
$ curl -X POST -H 'Content-Type: application/json' 'http://localhost:8080/mail_address_updates/{}'
{} までに実行しない場合は登録したメールアドレスは無効になります。"#,
            credential.mail_address(),
            verification.secret(),
            verification.expired_at()
        );
        let simple_email = SimpleEmail {
            subject: "メールアドレスを登録しました".to_string(),
            text,
            to: credential.mail_address().to_string(),
        };
        send(simple_email);
    }

    fn send_update_password_mail(&self, credential: &Credential) {
        let password_reset = credential.password_reset().unwrap();
        let text = format!(
            r#"パスワードのリセット要求を受け付けました。
パスワードを変更するには次のコマンドを実行してください。
$ curl -X PATCH -H 'Content-Type: application/json' -d '{{"password":"YOURPASSWORD"}}' 'http://localhost:8080/password_resets/{}'
{} までに実行しない場合はリセット要求は無効になります。"#,
            password_reset.secret(),
            password_reset.expired_at()
        );
        let simple_email = SimpleEmail {
            subject: "パスワードのリセット要求を受け付けました".to_string(),
            text,
            to: credential.mail_address().to_string(),
        };
        send(simple_email);
    }

    fn send_user_verified_mail(&self, user: &User, credential: &Credential) {
        let text = format!(
            r#"ユーザー ({}) を作成しました。
サインインするには次のコマンドを実行してください。
パスワードを変更するには次のコマンドを実行してください。
$ curl -X POST -H 'Content-Type: application/json' -d '{{"mail_address":"{}","password":"YOURPASSWORD"}}' 'http://localhost:8080/sessions'"#,
            String::from(user.key()),
            credential.mail_address()
        );
        let simple_email = SimpleEmail {
            subject: "ユーザーを作成しました".to_string(),
            text,
            to: credential.mail_address().to_string(),
        };
        send(simple_email);
    }

    fn send_verify_mail_address_mail(&self, credential: &Credential) {
        let verification = credential.verification().unwrap();
        let text = format!(
            r#"メールアドレスの変更要求を受け付けました。
メールアドレスを変更するには次のコマンドを実行してください。
$ curl -X PATCH -H 'Content-Type: application/json' 'http://localhost:8080/mail_address_updates/{}'"#,
            verification.secret(),
        );
        let simple_email = SimpleEmail {
            subject: "メールアドレスの変更要求を受け付けました".to_string(),
            text,
            to: credential.mail_address().to_string(),
        };
        send(simple_email);
    }
}
