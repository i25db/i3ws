use serde::Serialize;
use serde::Deserialize;
pub use crate::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub id: i64,
    pub num: i32,
    pub name: String,
    pub visible: bool,
    pub focused: bool,
    pub rect: Rect,
    pub output: String,
    pub urgent: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}



pub fn parse_workspaces(json: &str) -> Vec<Workspace> {
    serde_json::from_str(json).expect("Unable to parse json")
}
