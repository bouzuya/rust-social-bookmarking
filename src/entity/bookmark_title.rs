#[derive(Clone)]
pub struct BookmarkTitle(String);

impl BookmarkTitle {
    pub fn from_str(s: &str) -> Option<Self> {
        Some(Self(s.into()))
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
