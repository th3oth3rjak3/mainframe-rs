use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: i32,
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
}

impl Session {
    pub fn new(user_id: i32) -> Self {
        Self::new_with_duration(user_id, Duration::days(1))
    }

    pub fn new_with_duration(user_id: i32, duration: Duration) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            created_at: OffsetDateTime::now_utc(),
            expires_at: OffsetDateTime::now_utc().saturating_add(duration),
        }
    }
}
