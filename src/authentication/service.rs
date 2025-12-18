use std::sync::Arc;

use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::{
    authentication::{AuthenticatedUser, IAuthenticationRepository, LoginRequest},
    errors::{RepositoryError, ServiceError},
    roles::IRoleRepository,
    sessions::{ISessionRepository, Session},
    token::{self, SessionToken},
    users::{IUserRepository, Password, User},
};

const MAX_FAILED_LOGIN_ATTEMPTS: i64 = 5;

#[async_trait::async_trait]
pub trait IAuthenticationService: Send + Sync {
    async fn login(&self, request: LoginRequest) -> Result<AuthenticatedUser, ServiceError>;
    async fn logout(&self, session_id: Uuid) -> Result<(), ServiceError>;
    async fn refresh(&self, session_token: SessionToken)
    -> Result<AuthenticatedUser, ServiceError>;
}

#[derive(Clone)]
pub struct AuthenticationService {
    authentication: Arc<dyn IAuthenticationRepository>,
    users: Arc<dyn IUserRepository>,
    roles: Arc<dyn IRoleRepository>,
    sessions: Arc<dyn ISessionRepository>,
}

impl AuthenticationService {
    pub fn new(
        auth_repo: Arc<dyn IAuthenticationRepository>,
        user_repo: Arc<dyn IUserRepository>,
        role_repo: Arc<dyn IRoleRepository>,
        session_repo: Arc<dyn ISessionRepository>,
    ) -> Self {
        Self {
            authentication: auth_repo,
            users: user_repo,
            roles: role_repo,
            sessions: session_repo,
        }
    }
}

#[async_trait::async_trait]
impl IAuthenticationService for AuthenticationService {
    async fn login(&self, request: LoginRequest) -> Result<AuthenticatedUser, ServiceError> {
        let mut user_base = self
            .users
            .get_by_username(&request.username)
            .await
            .map_err(|err| {
                if let RepositoryError::NotFound { .. } = err {
                    // generate random password hash and verify against input to prevent timing attacks.
                    let fake_pw = Password::new("fake password");
                    if let Ok(pw) = fake_pw {
                        _ = pw.verify(request.password.as_bytes());
                    }

                    return ServiceError::InvalidUsernameOrPassword;
                }

                err.into()
            })?;

        let is_valid = user_base.password_hash.verify(request.password.as_bytes());
        if is_valid {
            user_base.last_login = Some(OffsetDateTime::now_utc());
            user_base.failed_login_attempts = 0;
            user_base.last_failed_login_attempt = None;
            user_base.updated_at = OffsetDateTime::now_utc();

            let session_id = Uuid::now_v7();
            let session_token = token::generate_session_token(session_id);

            // Get HMAC key and hash the token for storage
            let hmac_key_hex = std::env::var("SESSION_HMAC_KEY")
                .map_err(|_| ServiceError::Internal(anyhow::anyhow!("HMAC key not configured")))?;
            let hmac_key = hex::decode(hmac_key_hex)
                .map_err(|_| ServiceError::Internal(anyhow::anyhow!("Invalid HMAC key format")))?;

            let token_hash = session_token
                .hash_token(&hmac_key)
                .map_err(|_| ServiceError::Internal(anyhow::anyhow!("Failed to hash token")))?;

            let session = Session::new(session_id, user_base.id, token_hash);

            let user = self.authentication.login(user_base, session).await?;
            let mut auth_user: AuthenticatedUser = user.into();
            auth_user.session.token = session_token.encode();
            return Ok(auth_user);
        }

        user_base.failed_login_attempts += 1;
        user_base.last_failed_login_attempt = Some(OffsetDateTime::now_utc());
        user_base.updated_at = OffsetDateTime::now_utc();

        if user_base.failed_login_attempts >= MAX_FAILED_LOGIN_ATTEMPTS {
            user_base.is_disabled = true;
            self.sessions.delete_all_for_user(user_base.id).await?;
        }

        self.users.update_base(&user_base).await?;

        if user_base.is_disabled {
            return Err(ServiceError::AccountLocked);
        }

        return Err(ServiceError::InvalidUsernameOrPassword);
    }

    async fn logout(&self, session_id: Uuid) -> Result<(), ServiceError> {
        self.sessions.delete(session_id).await?;
        Ok(())
    }

    async fn refresh(
        &self,
        session_token: SessionToken,
    ) -> Result<AuthenticatedUser, ServiceError> {
        // Get the HMAC key from environment
        let hmac_key_hex = std::env::var("SESSION_HMAC_KEY")
            .map_err(|_| ServiceError::Internal(anyhow::anyhow!("HMAC key not configured")))?;
        let hmac_key = hex::decode(hmac_key_hex)
            .map_err(|_| ServiceError::Internal(anyhow::anyhow!("Invalid HMAC key format")))?;

        // get a session
        let mut session = self
            .sessions
            .get_by_id(session_token.session_id)
            .await
            .map_err(|err| {
                if let RepositoryError::NotFound { .. } = err {
                    return ServiceError::Unauthorized("no valid session".into());
                }

                err.into()
            })?;

        // Verify the token hash matches what's stored
        let is_valid = crate::token::verify_token(
            &session_token.raw_token,
            &session.token, // You'll need to add this field to your Session struct
            &hmac_key,
        )
        .map_err(|_| ServiceError::Unauthorized("token verification failed".into()))?;

        if !is_valid {
            return Err(ServiceError::Unauthorized("invalid session token".into()));
        }

        // validate it's not expired
        if session.expires_at < OffsetDateTime::now_utc() {
            return Err(ServiceError::Unauthorized("session is expired".into()));
        }

        // find the user
        let user_base = self.users.get_by_id(session.user_id).await.map_err(|err| {
            if let RepositoryError::NotFound { .. } = err {
                return ServiceError::Unauthorized(
                    "user id from session token was not found".into(),
                );
            }

            err.into()
        })?;

        if user_base.is_disabled {
            return Err(ServiceError::AccountLocked);
        }

        // update session expires_at
        session.expires_at = OffsetDateTime::now_utc().saturating_add(Duration::hours(2));

        // tell repo to save the details
        self.sessions.update(&session).await?;

        let mut user: User = user_base.into();
        let roles = self.roles.get_by_user_id(user.id).await.map_err(|err| {
            if let RepositoryError::NotFound { .. } = err {
                return ServiceError::Unauthorized("error getting roles for user".into());
            }

            err.into()
        })?;

        user.roles = roles;

        let mut auth_user = AuthenticatedUser {
            user: user.into(),
            session,
        };

        auth_user.session.token = session_token.encode();
        // return user and session details
        Ok(auth_user)
    }
}
