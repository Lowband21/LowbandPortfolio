use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Skill {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Bio {
    pub id: i32,
    pub bio_content: String,
}
