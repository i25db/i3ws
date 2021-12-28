pub(crate) use crate::json;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    pub id: i64,
    pub num: i32,
    pub name: String,
    pub visible: bool,
    pub focused: bool,
    pub rect: Rect,
    pub output: String,
    pub urgent: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub fn parse_workspaces(json: &str) -> Vec<Workspace> {
    // TODO: Cache this command
    serde_json::from_str(json).expect("Unable to parse json")
}
