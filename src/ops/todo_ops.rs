use crate::args::{CreateTodo, DeleteTodo, UpdateTodo};
use crate::db::establish_connection;
use crate::models::{NewTodo, Todo};
use anyhow::Result;
use diesel::prelude::*;

pub fn create_todo(todo: CreateTodo) -> Result<()> {
    use crate::schema::todos::dsl::*;

    let conn = &mut establish_connection()?;

    let new_todo = NewTodo {
        description: &todo.description,
    };

    diesel::insert_into(todos).values(&new_todo).execute(conn)?;

    Ok(())
}

pub fn update_todo(todo: UpdateTodo) -> Result<()> {
    use crate::schema::todos::dsl::*;

    let conn = &mut establish_connection()?;

    let db_todo = Todo {
        id: todo.id,
        description: todo.description,
        completed: todo.completed,
    };

    diesel::update(todos.find(todo.id))
        .set(&db_todo)
        .execute(conn)?;

    Ok(())
}

pub fn delete_todo(todo: DeleteTodo) -> Result<()> {
    use crate::schema::todos::dsl::*;

    let conn = &mut establish_connection()?;

    diesel::delete(todos.find(todo.id)).execute(conn)?;

    Ok(())
}

pub fn load_todos() -> Result<()> {
    use crate::schema::todos::dsl::*;

    let conn = &mut establish_connection()?;

    let result = todos.select(Todo::as_select()).load(conn)?;

    for (idx, todo) in result.iter().enumerate() {
        println!(
            "{}) ID: {} Description: {} Completed: {}",
            idx as i32 + 1,
            todo.id,
            todo.description,
            todo.completed
        )
    }

    Ok(())
}
