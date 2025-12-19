use axum_extra::extract::cookie::{Cookie, SameSite};
use time::OffsetDateTime;

pub fn build_session_cookie(token: String, expires_at: OffsetDateTime) -> Cookie<'static> {
    Cookie::build(("session_id", token))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .expires(expires_at)
        .build()
}

pub fn build_expired_session_cookie() -> Cookie<'static> {
    Cookie::build(("session_id", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .expires(OffsetDateTime::UNIX_EPOCH)
        .build()
}
