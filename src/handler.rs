use axum::{
    extract::{Path, Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
    debug_handler
};
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;
use crate::model::{QueryOptions, Todo, UpdateTodoSchema};
use crate::error::CustomError;
//table name: todos

//Fetch data: get
pub async fn todos_list_handler(
    opts: Option<Query<QueryOptions>>,
    Extension(pool): Extension<SqlitePool>,
) -> impl IntoResponse {
    let axum::extract::Query(QueryOptions { page, limit }) = opts.unwrap_or_default();

     let limit = limit.unwrap_or(10);
     let offset= (page.unwrap_or(1) - 1) * limit;

    let sql = "SELECT * FROM todos ORDER BY id";

    let todovec: Vec<Todo> = sqlx::query_as::<_, Todo>(sql)
        .fetch_all(&pool)
        .await.unwrap();

    if todovec.is_empty() {
        return (StatusCode::OK, Json(todovec));
    }

    // Calculate the start and end index of the data for the desired page
    let start_index = offset;
    let end_index = (start_index + limit).min(todovec.len());

    // Slice the data to extract the desired page
    let paged_data: Vec<Todo> = todovec[start_index..end_index].to_vec();
    (StatusCode::OK,Json(paged_data))
}

//Fetch a item from id: get 
pub async fn get_todo_handler(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<SqlitePool>,
) -> Result <(StatusCode, Json<Todo>), CustomError>{

    let id = id.to_string();
    let sql = "SELECT * FROM todos where id=$1";
    
    let todo: Todo= sqlx::query_as(sql)
        .bind(id)
        .fetch_one(&pool)
        .await.map_err(|_|{CustomError::TodoNotFound})?;

    Ok((StatusCode::OK, Json(todo)))
}

//create a item: post
#[debug_handler]
pub async fn create_todo_handler(
    Extension(pool): Extension<SqlitePool>,
    Json(body): Json<Todo>
) -> Result <(StatusCode, Json<Todo>), CustomError> {
// Check if title is empty and return BadRequest error if true
    if body.title.is_empty() {
        return Err(CustomError::BadRequest)
    }
// Check if a todo with the same title already exists
    let todo_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM todos WHERE title = ?)",
    )
    .bind(&body.title)
    .fetch_one(&pool)
    .await.map_err(|_| CustomError::InternalServerError)?;

    if todo_exists {
        return Err(CustomError::TodoDuplicate);
    }
    
    let sql ="INSERT INTO todos (id, title, movement, punishment, mixup, combo) VALUES (?, ?, ?, ?, ?, ?)";
    let uuid_id = Uuid::new_v4().to_string();
    let new_todo=Todo {
        id: Some(uuid_id),
        title: body.title,
        movement: Some(body.movement.unwrap_or(0)),
        punishment: Some(body.punishment.unwrap_or(0)),
        mixup: Some(body.mixup.unwrap_or(0)),
        combo: Some(body.combo.unwrap_or(0)),
    };
    let new_todo_return = new_todo.clone();

    sqlx::query(sql)
        .bind(new_todo.id.unwrap())
        .bind(new_todo.title)
        .bind(new_todo.movement.unwrap())
        .bind(new_todo.punishment.unwrap())
        .bind(new_todo.mixup.unwrap())
        .bind(new_todo.combo.unwrap()) 
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(new_todo_return)))
}

//update item: patch
#[debug_handler]
pub async fn edit_todo_handler(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTodoSchema>,
) -> Result <(StatusCode, Json<Todo>), CustomError> {
    let id = id.to_string();
    // Check if the todo with the provided ID exists in the database
    let sql = "SELECT * FROM todos where id=$1";
    let todo: Todo= sqlx::query_as(sql)
        .bind(&id)
        .fetch_one(&pool)
        .await.map_err(|_|{CustomError::TodoNotFound})?;

    let update_todo = Todo {
        id : Some(id.clone()),
        title : todo.title,
        movement : Some(body.movement.unwrap_or(todo.movement.unwrap())),
        punishment: Some(body.punishment.unwrap_or(todo.punishment.unwrap())),
        mixup: Some(body.mixup.unwrap_or(todo.mixup.unwrap())),
        combo: Some(body.combo.unwrap_or(todo.combo.unwrap())),
    };

    let sql = "UPDATE todos SET movement = ?, punishment = ?, mixup = ?, combo = ? WHERE id = ?;";
        // Execute the UPDATE query
    sqlx::query(sql)
        .bind(update_todo.movement)
        .bind(update_todo.punishment)
        .bind(update_todo.mixup)
        .bind(update_todo.combo)
        .bind(&id) // unwrap() is safe here since we just set it above
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

        // Fetch the updated todo from the database and return it in the response
        let updated_todo: Todo = sqlx::query_as("SELECT * FROM todos WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
            .map_err(|_| CustomError::InternalServerError)?;

        Ok((StatusCode::OK, Json(updated_todo)))
}

//delete item: delete
pub async fn delete_todo_handler(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<SqlitePool>
) -> Result<impl IntoResponse, CustomError> {
    let id = id.to_string();
    
    // Check if the todo with the provided ID exists in the database
    let todo_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM todos WHERE id = ?)",
    )
    .bind(&id)
    .fetch_one(&pool)
    .await
    .map_err(|_| CustomError::InternalServerError)?;
    
    if todo_exists {
        // Execute the DELETE query
        sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(&id)
            .execute(&pool)
            .await
            .map_err(|_| CustomError::InternalServerError)?;
        
        // Return a NO_CONTENT response
        Ok((StatusCode::NO_CONTENT, Json("")))
    } else {
        // Return a NOT_FOUND response
        Err(CustomError::TodoNotFound)
    }
}

pub async fn delete_all(Extension(pool): Extension<SqlitePool>) -> Result<impl IntoResponse, CustomError> {
    // Execute the DELETE query to delete all items from the table
    let sql = "DELETE FROM todos";
    sqlx::query(sql)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok(StatusCode::NO_CONTENT)
}
