#[derive(Clone)]
pub struct BookmarkComment(String);

impl BookmarkComment {
    pub fn from_str(s: &str) -> Option<Self> {
        Some(Self(s.into()))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
