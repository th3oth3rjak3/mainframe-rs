use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::users::UserBaseResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: OffsetDateTime,
}

impl Session {
    pub fn new(id: Uuid, user_id: Uuid, token: String) -> Self {
        Self::new_with_duration(id, user_id, Duration::hours(2), token)
    }

    pub fn new_with_duration(id: Uuid, user_id: Uuid, duration: Duration, token: String) -> Self {
        Self {
            id,
            user_id,
            token,
            expires_at: OffsetDateTime::now_utc().saturating_add(duration),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SessionSummary {
    pub user: UserBaseResponse,
    pub active_sessions: i64,
}
