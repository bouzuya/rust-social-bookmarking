#[derive(Clone)]
pub struct UserId(i32);

impl UserId {
    pub fn from_i32(i: i32) -> Option<Self> {
        if i > 0 {
            Some(UserId(i))
        } else {
            None
        }
    }

    pub fn to_i32(&self) -> i32 {
        self.0
    }
}
