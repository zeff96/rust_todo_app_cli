use crate::schema::todos;
use diesel::prelude::*;

#[derive(Debug, Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub description: &'a str,
}

#[derive(Debug, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}
