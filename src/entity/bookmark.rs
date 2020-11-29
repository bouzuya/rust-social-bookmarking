use crate::entity::bookmark_comment::BookmarkComment;
use crate::entity::bookmark_id::BookmarkId;
use crate::entity::bookmark_title::BookmarkTitle;
use crate::entity::bookmark_url::BookmarkUrl;
use crate::entity::user_id::UserId;
use anyhow::Result;

#[derive(Debug)]
pub struct Bookmark {
    id: BookmarkId,
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
            user_id,
            url,
            title,
            comment,
        }
    }

    pub fn id(&self) -> BookmarkId {
        self.id
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
        Ok(Self::new(self.id, self.user_id, url, title, comment))
    }
}
