use crate::entity::{
    bookmark_comment::BookmarkComment, bookmark_id::BookmarkId, bookmark_key::BookmarkKey,
    bookmark_title::BookmarkTitle, bookmark_url::BookmarkUrl, user_id::UserId,
};
use anyhow::Result;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bookmark {
    id: BookmarkId,
    key: BookmarkKey,
    user_id: UserId,
    url: BookmarkUrl,
    comment: BookmarkComment,
    title: BookmarkTitle,
}

impl Bookmark {
    pub fn new(
        id: BookmarkId,
        user_id: UserId,
        url: BookmarkUrl,
        title: BookmarkTitle,
        comment: BookmarkComment,
    ) -> Self {
        Self {
            id,
            key: BookmarkKey::generate(),
            user_id,
            url,
            title,
            comment,
        }
    }

    pub fn from_fields(
        id: BookmarkId,
        key: BookmarkKey,
        user_id: UserId,
        url: BookmarkUrl,
        title: BookmarkTitle,
        comment: BookmarkComment,
    ) -> Self {
        Self {
            id,
            key,
            user_id,
            url,
            title,
            comment,
        }
    }

    pub fn id(&self) -> BookmarkId {
        self.id
    }

    pub fn key(&self) -> BookmarkKey {
        self.key.clone()
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn url(&self) -> BookmarkUrl {
        self.url.clone()
    }

    pub fn title(&self) -> BookmarkTitle {
        self.title.clone()
    }

    pub fn comment(&self) -> BookmarkComment {
        self.comment.clone()
    }

    pub fn is_public(&self) -> bool {
        todo!()
    }

    pub fn update(
        &self,
        url: BookmarkUrl,
        title: BookmarkTitle,
        comment: BookmarkComment,
    ) -> Result<Bookmark> {
        Ok(Self::from_fields(
            self.id(),
            self.key(),
            self.user_id(),
            url,
            title,
            comment,
        ))
    }
}
