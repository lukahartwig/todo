use crate::{Todo, TodoStatus};
use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::Path;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut conn = Connection::open(path).expect("failed to open todo database");

        embedded::migrations::runner()
            .run(&mut conn)
            .expect("failed to run migrations");

        Ok(Self { conn })
    }

    pub fn insert_todo(&self, msg: String) -> Result<()> {
        self.conn
            .execute("INSERT INTO todos (title) VALUES (?1)", params![msg])
            .expect("failed to insert todo");
        Ok(())
    }

    pub fn find_open_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, created_at, title FROM todos WHERE status != 'DONE'")
            .expect("failed to prepare query");

        let todos = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    created_at: row.get(1)?,
                    title: row.get(2)?,
                })
            })?
            .filter_map(|todo| todo.ok())
            .collect();

        Ok(todos)
    }

    pub fn update_todo_status(&self, id: u32, status: TodoStatus) -> Result<()> {
        self.conn
            .execute(
                "UPDATE todos SET status = ?1 WHERE id = ?2",
                params![status, id],
            )
            .expect("failed to update todo status");
        Ok(())
    }

    pub fn prune_todos(&self) -> Result<()> {
        self.conn
            .execute("DELETE FROM todos WHERE status = 'DONE'", ())
            .expect("failed to delete todos");

        self.conn
            .execute(
                "UPDATE SQLITE_SEQUENCE SET SEQ = (SELECT MAX(id) FROM todos) WHERE NAME = 'todos'",
                (),
            )
            .expect("failed to reset autoincrement");

        Ok(())
    }
}
