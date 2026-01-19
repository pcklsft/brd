use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: i64,
    pub body: String,
    pub parent: Option<i64>,
    pub board_id: i64,

    pub file_name: Option<String>,
    pub file_type: Option<String>,
    pub file_path: Option<String>,
}
