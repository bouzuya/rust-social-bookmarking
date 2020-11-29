use crate::entity::{Bookmark, BookmarkId, UserId};
use crate::repository::BookmarkRepository;
use crate::schema::bookmark;
use anyhow::Result;
use diesel::{prelude::*, sql_types::*};
use std::convert::TryFrom;
use std::sync::Arc;

sql_function!(fn nextval(x: Text) -> BigInt);

pub struct PgBookmarkRepository {
    connection: Arc<PgConnection>,
}

impl PgBookmarkRepository {
    pub fn new(connection: Arc<PgConnection>) -> Self {
        Self { connection }
    }
}

#[derive(Insertable, Queryable)]
#[table_name = "bookmark"]
struct BookmarkRow {
    id: i32,
    key: String,
    user_id: i32,
    url: String,
    comment: String,
    title: String,
}

impl From<&Bookmark> for BookmarkRow {
    fn from(bookmark: &Bookmark) -> Self {
        Self {
            id: bookmark.id().into(),
            key: bookmark.key().into(),
            user_id: bookmark.user_id().into(),
            url: bookmark.url().into(),
            comment: bookmark.comment().into(),
            title: bookmark.title().into(),
        }
    }
}

impl BookmarkRepository for PgBookmarkRepository {
    fn create(
        &self,
        user_id: UserId,
        url: crate::entity::BookmarkUrl,
        title: crate::entity::BookmarkTitle,
        comment: crate::entity::BookmarkComment,
    ) -> Result<Bookmark> {
        let id =
            diesel::select(nextval("bookmark_id")).get_result::<i64>(self.connection.as_ref())?;
        let bookmark_id = BookmarkId::try_from(id as i32).map_err(anyhow::Error::msg)?;
        let bookmark = Bookmark::new(bookmark_id, user_id, url, title, comment);
        diesel::insert_into(bookmark::table)
            .values(BookmarkRow::from(&bookmark))
            .execute(self.connection.as_ref())
            .map(|_| bookmark)
            .map_err(anyhow::Error::msg)
    }

    fn delete(&self, _: &BookmarkId) -> Result<()> {
        todo!()
    }

    fn find_by_key(&self, _: &crate::entity::BookmarkKey) -> Result<Option<Bookmark>> {
        todo!()
    }

    fn find_by_user_id(&self, _: &UserId) -> Result<Vec<Bookmark>> {
        todo!()
    }

    fn save(&self, _: &Bookmark) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::User;
    use crate::{
        pg::{
            pg_credential_repository::PgCredentialRepository, pg_user_repository::PgUserRepository,
        },
        repository::{CredentialRepository, UserRepository},
    };

    #[test]
    fn test_scenario() {
        transaction(|connection| {
            let user = {
                let user_repository = PgUserRepository::new(connection.clone());
                let user_id = user_repository.create_id()?;
                let user = User::new(&user_id);
                user_repository.create(&user)?;
                let credential_repository = PgCredentialRepository::new(connection.clone());
                let mail_address = "m@bouzuya.net".parse().unwrap();
                let password = "password".parse().unwrap();
                let created = credential_repository.create(user.id(), &mail_address, &password)?;
                let secret = created.verification().unwrap().secret();
                let verified = created.verify(&secret)?;
                credential_repository.save(&verified)?;
                user
            };
            let repository = PgBookmarkRepository::new(connection.clone());

            {
                let url = "https://bouzuya.net".parse().unwrap();
                let title = "bouzuya.net".parse().unwrap();
                let comment = "bouzuya's website".parse().unwrap();
                repository.create(user.id(), url, title, comment)?;
            };

            Ok(())
        });
    }

    fn transaction<F>(f: F)
    where
        F: FnOnce(Arc<PgConnection>) -> Result<()>,
    {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        let connection = Arc::new(connection);
        connection
            .as_ref()
            .test_transaction::<(), anyhow::Error, _>(|| f(connection.clone()))
    }
}
