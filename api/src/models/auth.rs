use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    api_key: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Role {
    DashboardAdmin,
    BotAdmin,
}

impl User {}
