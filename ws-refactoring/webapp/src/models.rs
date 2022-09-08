use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherRegisterForm {
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherResponse {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}
