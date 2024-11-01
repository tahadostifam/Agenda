use std::sync::{Arc, Mutex};
use db::{create_todo, delete_todo, establish_connection, get_todos};
use diesel::SqliteConnection;
use models::models::Todo;
use tauri::Manager;

mod db;
mod models;
mod schema;

struct AppState {
    conn: Arc<Mutex<SqliteConnection>>,
}

#[tauri::command]
fn load_todos(state: tauri::State<'_, Mutex<AppState>>) -> Vec<Todo> {
    let state = state.lock().unwrap();
    let mut conn = &mut *state.conn.lock().unwrap();

    get_todos(&mut conn)
}

#[tauri::command]
fn add_todo(state: tauri::State<'_, Mutex<AppState>>, subj: &str, contx: &str) -> Todo {
    let state = state.lock().unwrap();
    let mut conn = &mut *state.conn.lock().unwrap();

    create_todo(&mut conn, subj, contx)
}

#[tauri::command]
fn remove_todo(state: tauri::State<'_, Mutex<AppState>>, id: i32) {
    let state = state.lock().unwrap();
    let mut conn = &mut *state.conn.lock().unwrap();

    delete_todo(&mut conn, id);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let conn = Arc::new(Mutex::new(establish_connection()));

    tauri::Builder::default()
        .setup(move |app| {
            let app_state = AppState { conn: conn.clone() };

            let state = Mutex::new(app_state);
            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_todos, add_todo, remove_todo])
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
