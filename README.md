# TekkenProgres
<h4 align="center">A todo API for learning tekken characters, using sqlite for database </h4>

## Guide
To create our sqlite database, we need to install sqlx-cli
```sh
cargo install sqlx-cli
```
Create a sqlite database, name it todo.db(optional)
```sh
sqlx database create --url "sqlite:todo.db"
```
Creates a new todo task in migrations/<timestamp>-<name>.sql
```sh
sqlx migrate add todo
```
Edit the sql file in migrations directory as my repository suggest
```sh
CREATE TABLE todos (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    movement INTEGER,
    punishment INTEGER,
    mixup INTEGER,
    combo INTEGER
);
```
Runs the migrations
```sh
sqlx migrate run 
```
Because we already set our DATABASE_URL in root directory, now we have a todos table in todo.db.

Now we're all set for the database-side. To run the API server, simply enter below command.
```sh
cargo watch -q -c -w src/ -x run
```
