use chrono::{DateTime, Local, Utc};

#[derive(Debug)]
pub struct Data {
    pub id: u64,
    pub name: String,
    pub date_added: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
}

impl Data {
    pub fn new(id: u64, name: String, due_date: Option<DateTime<Utc>>) -> Self {
        Self {
            id,
            name,
            date_added: Utc::now(),
            due_date,
        }
    }

    pub fn date_added_local(&self) -> DateTime<Local> {
        self.date_added.with_timezone(&Local)
    }

    pub fn due_date_local(&self, format: &str) -> Option<String> {
        self.due_date.map(|utc_date_time| {
            let local_date_time: DateTime<Local> = utc_date_time.with_timezone(&Local);
            local_date_time.format(format).to_string()
        })
    }

    pub fn set_due_date_from_local(&mut self, due_date: DateTime<Local>) {
        self.due_date = Some(due_date.with_timezone(&Utc));
    }
}

// impl<'r> FromRow<'r, PgRow> for Data {
//     fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
//         Ok(Data {
//             id: row.try_get("id")?,
//             name: row.try_get("name")?,
//             date_added: row.try_get("date_added")?,
//             due_date: row.try_get("due_date")?,
//         })
//     }
// }
