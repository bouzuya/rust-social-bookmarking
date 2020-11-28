use crate::entity::credential_secret_with_expiration::CredentialSecretWithExpiration;
use crate::entity::CredentialVerifiedAt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CredentialStatus {
    WaitingForVerification(CredentialSecretWithExpiration),
    Verified(CredentialVerifiedAt),
}
