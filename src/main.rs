use anyhow::Result;
use chrono_humanize::HumanTime;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use std::{fs, io};

mod store;
mod todo;

use crate::store::Store;
use crate::todo::{Todo, TodoStatus};

#[derive(Debug, Parser)]
#[clap(name = "todo", version)]
struct App {
    #[arg(short)]
    fail: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new todo
    Add { message: String },
    /// List open todos
    #[command(visible_alias = "ls")]
    List,
    /// Update todo status
    Set { id: u32, status: TodoStatus },
    /// Remove done todos
    Prune,
    /// Generates completions
    Completions { shell: Shell },
}

fn main() -> Result<()> {
    let app = App::parse();

    let home = home::home_dir().unwrap();
    let path = home.as_path().join(".todo");

    if !path.exists() {
        fs::create_dir_all(path.as_path())?;
    }

    let store = Store::open(path.join("todo.db"))?;

    match app.command {
        Commands::Add { message } => {
            store.insert_todo(message).expect("failed to insert todo");
        }
        Commands::List => {
            let todos = store.find_open_todos().expect("failed to find open todos");

            for todo in todos {
                println!(
                    "{}) {} ({})",
                    todo.id,
                    todo.title,
                    HumanTime::from(todo.created_at)
                );
            }
        }
        Commands::Set { id, status } => {
            store
                .update_todo_status(id, status)
                .expect("failed to update todo status");
        }
        Commands::Prune => {
            store.prune_todos().expect("failed pruning todos");
        }
        Commands::Completions { shell } => {
            generate(shell, &mut App::command(), "todo", &mut io::stdout());
        }
    }

    Ok(())
}
