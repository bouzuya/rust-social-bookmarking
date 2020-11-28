use crate::entity::credential_secret_with_expiration::CredentialSecretWithExpiration;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CredentialStatus {
    WaitingForVerification(CredentialSecretWithExpiration),
    Verified,
}
