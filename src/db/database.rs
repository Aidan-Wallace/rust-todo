use rusqlite::{Connection, Result};

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn connect(conn_str: &str) -> Result<Self> {
        let conn = Connection::open(conn_str)?;

        Ok(Self { conn })
    }

    pub fn setup(&self, load_fake_data: bool) -> Result<()> {
        self.conn.execute(
            "create table if not exists todos(
                id          integer     primary key,
                name        text                    not null,
                date_added  datetime                not null,
                due_date    datetime                null
            )",
            (),
        )?;

        if load_fake_data {
            self.conn.execute(
                "insert into todos(id, name, date_added, due_date) values
                    (1, 'do laundry',           1718327834, 1720678389),
                    (2, 'take out the trash',   1718327834, 1749297337),
                    (3, 'clean car',            1718327834, 1749297112),
                    (5, 'clean room',           1718327834, 1749297274),
                    (6, 'do the dishes',        1718682009, 1718682009),
                    (8, 'do the thing',         1718328499, 1749297372),
                    (9, 'write essay',          1718329485, 1749297723)",
                (),
            )?;
        }

        Ok(())
    }
}

// #[tokio::main]
// async fn psql() -> Result<(), sqlx::Error> {
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect("postgres://postgres:postgres@localhost/todos")
//         .await?;
//     let users: Vec<Data> = sqlx::query_as::<_, Data>("SELECT * FROM users")
//         .fetch_all(&pool)
//         .await?;
//     println!("Query executed successfully: {:?}", rows[0]);
//     Ok(())
// }
