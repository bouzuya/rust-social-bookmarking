use crate::entity::credential_verification::CredentialVerification;

#[derive(Debug)]
pub enum CredentialStatus {
    WaitingForVerification(CredentialVerification), // (1). -> (2) / -> (3)
    WaitingForDeletion,                             // (2).
    Verified,                                       // (3). -> (4)
    ResettingPassword(CredentialVerification),      // (4). -> (3) / -> (4)
}
