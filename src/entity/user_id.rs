use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserId(i32);

impl TryFrom<i32> for UserId {
    type Error = &'static str;
    fn try_from(i: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        if i >= 1 {
            Ok(UserId(i))
        } else {
            Err("UserId >= 1")
        }
    }
}

impl From<UserId> for i32 {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}
