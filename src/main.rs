use anyhow::Result;
use chrono::prelude::*;
use chrono_humanize::HumanTime;
use clap::{Parser, ValueEnum};
use rusqlite::{params, Connection, ToSql};
use std::fmt;
use std::fs;

#[derive(Debug, Clone, ValueEnum)]
enum TodoStatus {
    Ready,
    Doing,
    Done,
}

impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoStatus::Ready => write!(f, "READY"),
            TodoStatus::Doing => write!(f, "DOING"),
            TodoStatus::Done => write!(f, "DONE"),
        }
    }
}

impl ToSql for TodoStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

#[derive(Debug)]
struct Todo {
    id: u32,
    created_at: DateTime<Local>,
    title: String,
}

#[derive(Debug, Parser)]
enum Commands {
    Add { message: String },
    List,
    Set { id: u32, status: TodoStatus },
    Prune,
}

#[derive(Debug, Parser)]
#[clap(name = "todo", version)]
struct App {
    #[clap(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let app = App::parse();

    let home = home::home_dir().unwrap();
    let path = home.as_path().join(".todo");

    if !path.exists() {
        fs::create_dir_all(path.as_path())?;
    }

    let db = Connection::open(path.join("todo.db")).expect("failed to open todo database");

    db.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
            title       TEXT NOT NULL,
            status      TEXT NOT NULL DEFAULT 'READY'
        )",
        (),
    )?;

    match app.command {
        Commands::Add { message } => {
            db.execute("INSERT INTO todos (title) VALUES (?1)", params![message])
                .expect("failed to insert todo");
        }
        Commands::List => {
            let mut stmt = db
                .prepare("SELECT id, created_at, title FROM todos WHERE status != 'DONE'")
                .expect("failed to prepare query");

            stmt.query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    created_at: row.get(1)?,
                    title: row.get(2)?,
                })
            })?
            .filter_map(|todo| todo.ok())
            .for_each(|todo| {
                println!(
                    "{}) {} ({})",
                    todo.id,
                    todo.title,
                    HumanTime::from(todo.created_at)
                );
            });
        }
        Commands::Set { id, status } => {
            db.execute(
                "UPDATE todos SET status = ?1 WHERE id = ?2",
                params![status, id],
            )
            .expect("failed to update todo status");
        }
        Commands::Prune => {
            db.execute("DELETE FROM todos WHERE status = 'DONE'", ())
                .expect("failed to delete todos");

            db.execute(
                "UPDATE SQLITE_SEQUENCE SET SEQ = (SELECT MAX(id) FROM todos) WHERE NAME = 'todos'",
                (),
            )
            .expect("failed to reset autoincrement");
        }
    }

    Ok(())
}
