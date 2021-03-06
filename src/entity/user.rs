use crate::entity::user_id::UserId;
use crate::entity::user_key::UserKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    id: UserId,
    key: UserKey,
}

impl User {
    pub fn new(id: &UserId) -> Self {
        Self {
            id: id.clone(),
            key: UserKey::generate(),
        }
    }

    pub fn of(id: UserId, key: UserKey) -> Self {
        Self { id, key }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn key(&self) -> UserKey {
        self.key.clone()
    }
}
