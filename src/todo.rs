use std::fmt;
use chrono::{DateTime, Local};
use clap::ValueEnum;
use rusqlite::ToSql;

#[derive(Debug, Clone, ValueEnum)]
pub enum TodoStatus {
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
pub struct Todo {
    pub id: u32,
    pub created_at: DateTime<Local>,
    pub title: String,
}
