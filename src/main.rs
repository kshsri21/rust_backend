use rocket::*;
mod task;

use task::*;

use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/tasks")]
fn fetch_tasks() -> Json<Vec<Task>> {
    let tasks = load_tasks();
    Json(tasks)
}

#[post("/create-task", format = "json", data = "<task>")]
fn create_task(task: Json<Task>) -> Status {
    println!("client task:{:?}", task);
    let mut tasks = load_tasks();

    if let Some(_index) = tasks
        .iter()
        .position(|item| item.task_name == task.0.task_name)
    {
        return Status::Conflict;
    }
    tasks.push(task.0);
    save_tasks(&tasks);
    Status::Created
}

#[put("/update-task", format = "json", data = "<task>")]
fn update_task(task: Json<Task>) -> Status {
    let mut tasks = load_tasks();

    if let Some(index) = tasks
        .iter()
        .position(|item| item.task_name == task.0.task_name)
    {
        tasks.remove(index);
        tasks.insert(index, task.0);
        save_tasks(&tasks);
        return Status::NoContent;
    } else {
        return Status::NotFound;
    }
}

#[delete("/delete-task", format = "json", data = "<task>")]
fn delete_task(task: Json<Task>) -> Status {
    let mut tasks = load_tasks();

    if let Some(index) = tasks
        .iter()
        .position(|item| item.task_name == task.0.task_name)
    {
        tasks.remove(index);
        save_tasks(&tasks);
        return Status::NoContent;
    } else {
        return Status::NotFound;
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![fetch_tasks, create_task, update_task, delete_task],
    )
}

// #[get("/")]
// fn hello_user() -> &'static str {
//     "Hello User"
// }
// #[get("/home/<name>")]
// fn hello_user(name: String) -> String {
//     format!("Hello {}", name)
// }
