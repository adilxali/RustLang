use diesel::prelude::*;
use serde::{ Serialize, Deserialize };

#[derive(Queryable, Selectable,Serialize, Deserialize)]
#[diesel(table_name = crate::schema::todos)]
pub struct Todo{
    pub id: i32,
    pub task: String,
    pub completed: bool,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::todos)]
pub struct  NewTodo {
    pub task: String,
    pub completed: bool,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::todos)]
pub struct UpdateTodo {
    pub task: Option<String>,
    pub completed: Option<bool>,
}