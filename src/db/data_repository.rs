use super::{database, repository::Repository};
use crate::data::{self, Data};
use chrono::{DateTime, TimeZone, Utc};
use rusqlite::Result;

pub struct DataRepo {
    db: database::Db,
}

impl Repository<Data> for DataRepo {
    fn new(db: database::Db) -> Self {
        Self { db }
    }

    fn fetch_first_record(&self) -> Result<Data, rusqlite::Error> {
        todo!();
    }

    fn insert(&self, data: data::Data) -> Result<()> {
        let sql = "insert into todos (name, date_added, due_date) values (?1, ?2, ?3)";

        let date_added_timestamp = data.date_added.timestamp();
        let due_date_timestamp = data.due_date.map(|dt| dt.timestamp());

        self.db.conn.execute(
            sql,
            (&data.name, &date_added_timestamp, &due_date_timestamp),
        )?;

        Ok(())
    }

    fn get_all(&self) -> Result<Vec<Data>> {
        let sql = "select id, name, date_added, due_date from todos";
        let mut stmt = self.db.conn.prepare(sql)?;

        let data_iter = stmt.query_map([], |row| {
            let date_added_timestamp: i64 = row.get(2)?;
            let due_date_timestamp: Option<i64> = row.get(3)?;

            let date_added = match timestamp_to_utc(date_added_timestamp) {
                Some(ts) => ts,
                None => panic!("Cannot convert i32 to utc date time"),
            };

            let due_date = match due_date_timestamp {
                Some(ts) => {
                    Some(timestamp_to_utc(ts).ok_or_else(|| rusqlite::Error::InvalidQuery)?)
                }
                None => None,
            };

            Ok(Data {
                id: row.get(0)?,
                name: row.get(1)?,
                date_added,
                due_date,
            })
        })?;

        let mut events = Vec::new();
        for event in data_iter {
            events.push(event?);
        }

        Ok(events)
    }

    fn remove_by_id(&self, id: u64) -> Result<()> {
        let sql = "delete from todos where id = ?";
        self.db.conn.execute(sql, rusqlite::params![id])?;

        Ok(())
    }
}

fn timestamp_to_utc(ts: i64) -> Option<DateTime<Utc>> {
    Utc.timestamp_opt(ts, 0).single()
}
