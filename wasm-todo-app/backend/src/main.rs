// backend/src/main.rs
#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use std::sync::Mutex;

// In-memory todo storage
struct TodoState {
    todos: Mutex<Vec<Todo>>
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    text: String,
    completed: bool
}

#[get("/todos")]
fn get_todos(state: &rocket::State<TodoState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().unwrap();
    Json(todos.clone())
}

#[post("/todos", data = "<todo>")]
fn create_todo(todo: Json<Todo>, state: &rocket::State<TodoState>) -> Json<Todo> {
    let mut todos = state.todos.lock().unwrap();
    let new_todo = todo.into_inner();
    todos.push(new_todo.clone());
    Json(new_todo)
}

#[delete("/todos/<id>")]
fn delete_todo(id: usize, state: &rocket::State<TodoState>) -> Json<bool> {
    let mut todos = state.todos.lock().unwrap();
    let position = todos.iter().position(|t| t.id == id);
    if let Some(pos) = position {
        todos.remove(pos);
        return Json(true);
    }
    Json(false)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![get_todos, create_todo, delete_todo])
        .manage(TodoState {
            todos: Mutex::new(vec![])
        })
}
