use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BadgeColor {
    Brightgreen,
    Green,
    Yellowgreen,
    Yellow,
    Orange,
    Red,
    Lightgrey,
    Blue,
}

#[derive(Serialize, Debug)]
pub struct Badge {
    #[serde(rename(serialize = "schemaVersion"))]
    pub schema_version: u8,
    pub label: String,
    pub message: String,
    pub color: BadgeColor,
}

impl Badge {
    pub fn new(label: String, message: String, color: BadgeColor) -> Self {
        Self {
            schema_version: 1,
            label,
            message,
            color,
        }
    }
}
