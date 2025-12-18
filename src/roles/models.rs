use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub name: RoleName,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, sqlx::Type, Serialize, Deserialize)]
pub enum RoleName {
    Administrator,
    RecipeUser,
    Unknown,
}

impl Display for RoleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Administrator => "Administrator",
            Self::RecipeUser => "Recipe User",
            Self::Unknown => "Unknown",
        };

        f.write_str(name)
    }
}

impl From<String> for RoleName {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Administrator" => Self::Administrator,
            "Recipe User" => Self::RecipeUser,
            _ => Self::Unknown,
        }
    }
}
