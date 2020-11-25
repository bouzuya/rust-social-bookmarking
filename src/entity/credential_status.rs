use crate::entity::credential_secret_with_expiration::CredentialSecretWithExpiration;

#[derive(Debug, Eq, PartialEq)]
pub enum CredentialStatus {
    WaitingForVerification(CredentialSecretWithExpiration), // (1). -> (2) / -> (3)
    WaitingForDeletion,                                     // (2).
    Verified,                                               // (3). -> (4)
    ResettingPassword(CredentialSecretWithExpiration),      // (4). -> (3) / -> (4)
}
