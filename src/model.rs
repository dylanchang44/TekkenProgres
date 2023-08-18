use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub movement: Option<i8>,
    pub punishment: Option<i8>,
    pub mixup: Option<i8>,
    pub combo: Option<i8>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct UpdateTodoSchema {
    pub movement: Option<i8>,
    pub punishment: Option<i8>,
    pub mixup: Option<i8>,
    pub combo: Option<i8>
}


