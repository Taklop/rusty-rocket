use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Animal {
    pub id: Option<i32>,
    pub name: String,
    pub age: i32,
    pub leg_count: i32
}