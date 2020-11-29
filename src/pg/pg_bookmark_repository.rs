use crate::entity::{Bookmark, BookmarkId, BookmarkKey, UserId};
use crate::repository::BookmarkRepository;
use crate::schema::bookmark;
use anyhow::Result;
use diesel::{prelude::*, sql_types::*};
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;

sql_function!(fn nextval(x: Text) -> BigInt);

pub struct PgBookmarkRepository {
    connection: Arc<PgConnection>,
}

impl PgBookmarkRepository {
    pub fn new(connection: Arc<PgConnection>) -> Self {
        Self { connection }
    }

    fn columns() -> (
        bookmark::columns::id,
        bookmark::columns::key,
        bookmark::columns::user_id,
        bookmark::columns::url,
        bookmark::columns::comment,
        bookmark::columns::title,
    ) {
        (
            bookmark::columns::id,
            bookmark::columns::key,
            bookmark::columns::user_id,
            bookmark::columns::url,
            bookmark::columns::comment,
            bookmark::columns::title,
        )
    }

    fn from_row(row: (i32, String, i32, String, String, String)) -> Result<Bookmark> {
        let (id, key, user_id, url, comment, title) = row;
        Ok(Bookmark::from_fields(
            id.try_into().map_err(anyhow::Error::msg)?,
            key.parse().map_err(anyhow::Error::msg)?,
            user_id.try_into().map_err(anyhow::Error::msg)?,
            url.parse().map_err(anyhow::Error::msg)?,
            title.parse().map_err(anyhow::Error::msg)?,
            comment.parse().map_err(anyhow::Error::msg)?,
        ))
    }
}

#[derive(AsChangeset, Insertable, Queryable)]
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

    fn find_by_key(&self, key: &BookmarkKey) -> Result<Option<Bookmark>> {
        let found = bookmark::table
            .select(Self::columns())
            .filter(bookmark::columns::key.eq(String::from(key.clone())))
            .first(self.connection.as_ref())
            .optional()
            .map_err(anyhow::Error::msg)?;
        found.map(Self::from_row).transpose()
    }

    fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Bookmark>> {
        bookmark::table
            .select(Self::columns())
            .filter(bookmark::columns::user_id.eq(i32::from(user_id.clone())))
            .get_results(self.connection.as_ref())
            .map(|rows| {
                rows.into_iter()
                    .filter_map(|row| Self::from_row(row).ok())
                    .collect()
            })
            .map_err(anyhow::Error::msg)
    }

    fn save(&self, b: &Bookmark) -> Result<()> {
        diesel::update(bookmark::table)
            .set(BookmarkRow::from(b))
            .filter(bookmark::columns::id.eq(i32::from(b.id())))
            .execute(self.connection.as_ref())
            .map(|_| ())
            .map_err(anyhow::Error::msg)
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

            let created = {
                let url = "https://bouzuya.net".parse().unwrap();
                let title = "bouzuya.net".parse().unwrap();
                let comment = "bouzuya's website".parse().unwrap();
                repository.create(user.id(), url, title, comment)?
            };

            let found = {
                let found = repository.find_by_key(&created.key())?;
                assert_eq!(found, Some(created.clone()));

                found.unwrap()
            };

            let updated = {
                let url = "https://blog.bouzuya.net".parse().unwrap();
                let title = "blog.bouzuya.net".parse().unwrap();
                let comment = "bouzuya's weblog".parse().unwrap();
                let updated = found.update(url, title, comment)?;
                repository.save(&updated)?;

                assert_eq!(repository.find_by_key(&found.key())?, Some(updated.clone()));
                updated
            };

            {
                let found = repository.find_by_user_id(&updated.user_id())?;
                assert_eq!(found, vec![updated.clone()]);
            }

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
