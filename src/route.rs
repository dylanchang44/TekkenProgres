use axum::{
    routing::{get, post},
    Router,
    extract::Extension
};
use sqlx::SqlitePool;
use crate::handler::{
        create_todo_handler, delete_todo_handler, edit_todo_handler,
        todos_list_handler, get_todo_handler, delete_all
    };

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        // create, read
        .route(
            "/api/todos",
            post(create_todo_handler)
                .get(todos_list_handler)
                .delete(delete_all)
        )
        // update, delete
        .route(
            "/api/todos/:id",  
                get(get_todo_handler).patch(edit_todo_handler)
                .delete(delete_todo_handler),
        )
        .layer(Extension(pool))
}
