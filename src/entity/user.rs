use crate::entity::credential::Credential;
use crate::entity::credential_id::CredentialId;
use crate::entity::user_id::UserId;
use crate::entity::user_key::UserKey;

#[derive(Clone, Debug)]
pub struct User {
    id: UserId,
    key: UserKey,
    credential_id: CredentialId,
}

impl User {
    pub fn new(id: UserId, credential: &Credential) -> Self {
        Self {
            id,
            key: UserKey::generate(),
            credential_id: credential.id(),
        }
    }

    pub fn of(id: UserId, key: UserKey, credential_id: CredentialId) -> Self {
        Self {
            id,
            key,
            credential_id,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn key(&self) -> UserKey {
        self.key.clone()
    }

    pub fn credential_id(&self) -> CredentialId {
        self.credential_id
    }
}
