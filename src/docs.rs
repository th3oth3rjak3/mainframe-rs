//! OpenAPI documentation configuration and aggregation.
//!
//! This module defines the main [`ApiDoc`] struct that serves as the root of the
//! OpenAPI specification for the Mainframe API. It merges documentation from all
//! feature-specific modules (users, authentication, recipes, etc.) into a single
//! unified API specification.
//!
//! # Configuration
//!
//! API metadata (title, version, description, contact info) is loaded from environment
//! variables at runtime. To customize the API documentation, add the following variables
//! to your `.env` file:
//!
//! - `API_TITLE` - The name of your API (default: "Mainframe API")
//! - `API_VERSION` - The API version string (default: "1.0.0")
//! - `API_DESCRIPTION` - A brief description of what the API does
//!   (default: "Self-hosted personal productivity platform API")
//! - `API_CONTACT_NAME` - The name of the API maintainer or support contact
//!   (default: "Administrator")
//! - `API_CONTACT_EMAIL` - Email address for API support (optional, no default)
//!
//! # Example .env Configuration
//!
//! ```env
//! API_TITLE=Mainframe API
//! API_VERSION=1.0.0
//! API_DESCRIPTION=Self-hosted personal productivity platform API
//! API_CONTACT_NAME=Your Name
//! API_CONTACT_EMAIL=support@example.com
//! ```
//!
//! # Usage
//!
//! The [`ApiDoc::merge_modules()`] method is called when serving the OpenAPI specification
//! to Scalar or other documentation tools. It automatically combines all module-level
//! documentation into a single spec.

use crate::{authentication::AuthApiDoc, sessions::SessionApiDoc, users::UsersApiDoc};
use std::env;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(), components(), tags())]
pub struct ApiDoc;

impl ApiDoc {
    pub fn merge_modules() -> utoipa::openapi::OpenApi {
        let mut api_docs = ApiDoc::openapi();

        // Update info from environment variables
        api_docs.info.title = env::var("API_TITLE").unwrap_or_else(|_| "Mainframe API".to_string());
        api_docs.info.version = env::var("API_VERSION").unwrap_or_else(|_| "1.0.0".to_string());
        api_docs.info.description = Some(
            env::var("API_DESCRIPTION")
                .unwrap_or_else(|_| "Self-hosted personal productivity platform API".to_string()),
        );

        if let Some(contact) = &mut api_docs.info.contact {
            contact.name =
                Some(env::var("API_CONTACT_NAME").unwrap_or_else(|_| "Administrator".to_string()));
            contact.email = env::var("API_CONTACT_EMAIL").ok();
        } else {
            use utoipa::openapi::ContactBuilder;

            let mut contact = ContactBuilder::new();
            contact = contact.name(Some(
                env::var("API_CONTACT_NAME").unwrap_or_else(|_| "Administrator".to_string()),
            ));
            if let Ok(email) = env::var("API_CONTACT_EMAIL") {
                contact = contact.email(Some(email));
            }
            api_docs.info.contact = Some(contact.build());
        }

        // Merge module docs
        api_docs.merge(UsersApiDoc::openapi());
        api_docs.merge(AuthApiDoc::openapi());
        api_docs.merge(SessionApiDoc::openapi());

        api_docs
    }
}
