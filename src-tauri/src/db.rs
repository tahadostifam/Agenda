use crate::models::models::{NewTodo, Todo};
use crate::schema::todos::dsl::todos;
use crate::schema::todos::id;
use diesel::associations::HasTable;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_todo(conn: &mut SqliteConnection, subj: &str, contx: &str) -> Todo {
    let new_todo = NewTodo {
        subject: subj,
        context: contx,
    };

    diesel::insert_into(todos::table())
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error saving new todo")
}

pub fn get_todos(conn: &mut SqliteConnection) -> Vec<Todo> {
    let results = todos
        .select(Todo::as_select())
        .load(conn)
        .expect("Error loading todos");

    results
}

pub fn delete_todo(conn: &mut SqliteConnection, todo_id: i32) {
    diesel::delete(todos.filter(id.eq(todo_id)))
        .execute(conn)
        .expect("Failed to delete todo");
}
