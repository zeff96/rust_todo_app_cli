use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct TodoList {
    #[clap(subcommand)]
    pub command: TodoCommand,
}

#[derive(Debug, Subcommand)]
pub enum TodoCommand {
    ///Create a new todo item
    Create(CreateTodo),
    ///Update an existing todo item
    Update(UpdateTodo),
    ///Delete an existing todo item
    Delete(DeleteTodo),
    ///Load all todos from the database
    Show,
}

#[derive(Debug, Args)]
pub struct CreateTodo {
    /// Description of the todo item
    #[arg(short, long)]
    pub description: String,
}

#[derive(Debug, Args)]
pub struct UpdateTodo {
    ///Id of the todo item to update
    #[arg(short, long)]
    pub id: i32,
    /// Description of the todo item
    #[arg(short, long)]
    pub description: String,
    /// Status of the todo item, default to false
    #[arg(short, long)]
    pub completed: bool,
}

#[derive(Debug, Args)]
pub struct DeleteTodo {
    ///Id of the todo item to delete
    #[arg(short, long)]
    pub id: i32,
}
