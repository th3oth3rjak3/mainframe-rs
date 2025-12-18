use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::users::UserBaseResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: OffsetDateTime,
}

impl Session {
    pub fn new(user_id: Uuid) -> Self {
        Self::new_with_duration(user_id, Duration::hours(2))
    }

    pub fn new_with_duration(user_id: Uuid, duration: Duration) -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id,
            expires_at: OffsetDateTime::now_utc().saturating_add(duration),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SessionSummary {
    pub user: UserBaseResponse,
    pub active_sessions: i64,
}
