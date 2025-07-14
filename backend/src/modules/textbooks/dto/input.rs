use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct CreateTextbookDto {
    pub title: String,
    pub description: Option<String>,
    pub level: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct UpdateTextbookDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub level: Option<String>,
    pub is_active: Option<bool>,
}