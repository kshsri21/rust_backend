# 📝 Todo App in Rust with Rocket

A simple RESTful Todo application built using the [Rocket](https://rocket.rs) framework in Rust. This app demonstrates CRUD (Create, Read, Update, Delete) operations on tasks stored in a CSV file.

## 🚀 Features

* 📄 Task storage in `tasks.csv`
* ✅ Create, read, update, and delete tasks via HTTP
* 🔄 Auto-reload using `cargo watch`
* 📦 JSON API using Rocket and Serde

## 🧰 Dependencies

The project uses the following Rust crates:

* [`rocket`](https://crates.io/crates/rocket) v0.5.1 with JSON support
* [`serde`](https://crates.io/crates/serde) v1.0.219 with derive
* [`csv`](https://crates.io/crates/csv) v1.3.1

### `Cargo.toml`

```toml
[package]
name = "todo_rust"
version = "0.1.0"
edition = "2024"

[dependencies]
csv = "1.3.1"
rocket = { version = "0.5.1", features = ["json"] }
serde = { version = "1.0.219", features = ["derive"] }
```

## 📂 Project Structure

```
.
├── src/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
└── tasks.csv
```

## 🛠️ Running the Project

Make sure you have `cargo-watch` installed:

```bash
cargo install cargo-watch
```

Then run the project with automatic reload on code change:

```bash
cargo watch -x run
```

## 📬 API Endpoints

| Method | Endpoint      | Description             |
| ------ | ------------- | ----------------------- |
| GET    | `/tasks`      | Get all tasks           |
| POST   | `/tasks`      | Add a new task          |
| PUT    | `/tasks/<id>` | Update an existing task |
| DELETE | `/tasks/<id>` | Delete a task           |

> 🗂️ All data is stored in `tasks.csv`.

## 📄 License

This project is licensed under the MIT License.
