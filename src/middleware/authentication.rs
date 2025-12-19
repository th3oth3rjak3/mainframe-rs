use axum::{
    body::Body,
    extract::State,
    http::{HeaderValue, Request, Response, header::SET_COOKIE},
    middleware::Next,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};

use crate::{
    authentication::SessionCookieHandled,
    cookies::{build_expired_session_cookie, build_session_cookie},
    extractors::authenticated_user::AuthenticatedUser,
    services::ServiceContainer,
    token::SessionToken,
};

pub async fn auth_middleware(
    State(container): State<ServiceContainer>,
    jar: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> Response<Body> {
    let eval = evaluate_session(&container, &jar).await;

    if let Some(auth_user) = &eval.auth_user {
        req.extensions_mut().insert(auth_user.clone());
    }

    let mut res = next.run(req).await;

    let session_cookie_handled = res.extensions().get::<SessionCookieHandled>().is_some();

    if session_cookie_handled {
        return res;
    }

    if !session_cookie_handled {
        if let Some(cookie) = eval.cookie {
            append_set_cookie(&mut res, &cookie);
        } else if eval.clear_cookie {
            append_set_cookie(&mut res, &build_expired_session_cookie());
        }
    }

    res
}

struct SessionEvaluation {
    auth_user: Option<AuthenticatedUser>,
    cookie: Option<Cookie<'static>>,
    clear_cookie: bool,
}

async fn evaluate_session(container: &ServiceContainer, jar: &CookieJar) -> SessionEvaluation {
    let Some(cookie) = jar.get("session_id") else {
        return SessionEvaluation {
            auth_user: None,
            cookie: None,
            clear_cookie: false,
        };
    };

    let Ok(token) = SessionToken::parse(cookie.value()) else {
        return SessionEvaluation {
            auth_user: None,
            cookie: None,
            clear_cookie: true,
        };
    };

    match container.auth_service().refresh(token).await {
        Ok(auth_user) => SessionEvaluation {
            cookie: Some(build_session_cookie(
                auth_user.session.token.clone(),
                auth_user.session.expires_at,
            )),
            auth_user: Some(auth_user),
            clear_cookie: false,
        },
        Err(_) => SessionEvaluation {
            auth_user: None,
            cookie: None,
            clear_cookie: true,
        },
    }
}

fn append_set_cookie(res: &mut Response<Body>, cookie: &Cookie<'static>) {
    match HeaderValue::from_str(&cookie.to_string()) {
        Ok(value) => {
            res.headers_mut().append(SET_COOKIE, value);
        }
        Err(_) => {
            tracing::error!("Failed to convert cookie to header value: {:?}", cookie);
        }
    }
}
