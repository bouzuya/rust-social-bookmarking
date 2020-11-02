use anyhow::Result;

pub trait UseListBookmarksUseCase {
    type ListBookmarksUseCase: ListBookmarksUseCase;
    fn list_bookmarks_use_case(&self) -> &Self::ListBookmarksUseCase;
}

pub trait ListBookmarksUseCase {
    fn list_bookmarks(&self) -> Result<()> {
        todo!()
    }
}

impl<T> ListBookmarksUseCase for T {}
