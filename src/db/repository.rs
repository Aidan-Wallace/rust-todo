use super::database;
use crate::data::{self, Data};
use rusqlite::Result;

pub trait Repository<T> {
    fn new(db: database::Db) -> Self;
    fn fetch_first_record(&self) -> Result<T, rusqlite::Error>;
    fn insert(&self, data: data::Data) -> Result<()>;
    fn get_all(&self) -> Result<Vec<Data>>;
    fn remove_by_id(&self, id: u64) -> Result<()>;
}
