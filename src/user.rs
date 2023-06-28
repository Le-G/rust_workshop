use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(alias = "_firestore_id")]
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
}
