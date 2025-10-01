use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::sql::Datetime;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Option<RecordId>,
    pub(crate) full_name: String,
    pub(crate) username: String,
    pub(crate) email_address: String,
    pub(crate) img: Option<String>,
    pub(crate) password: String,

    pub(crate) email_verified: bool,
    pub(crate) platform_verification: bool,

    pub(crate) created_at: Option<Datetime>,
    pub(crate) updated_at: Option<Datetime>,
}
