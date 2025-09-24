use anyhow::Result;
use async_trait::async_trait;
use kernel::{model::book::{event::CreateBook, Book}, repository::book::BookRepository};
use uuid::Uuid;

use crate::database::ConnectionPool;


pub struct BookRepositoryImpl {
    db: ConnectionPool
}

#[async_trait]
impl BookRepository for BookRepositoryImpl{
    async fn create(&self, event: CreateBook) -> Result<()> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<Book>> {
        todo!()
    }

    async fn find_by_id(&self, book_id: Uuid) -> Result<Option<Book>> {
        todo!()
    }

}
