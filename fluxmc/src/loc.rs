use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}
