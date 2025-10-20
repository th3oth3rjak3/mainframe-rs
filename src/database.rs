use std::ops::DerefMut;

use rocket_db_pools::{Connection, Database};
use sqlx::{PgConnection, PgPool};

#[derive(Database)]
#[database("app_central_db")]
pub struct AppCentralDb(PgPool);

pub trait AsMutPgConnection {
    fn connection(&mut self) -> &mut PgConnection;
}

impl AsMutPgConnection for Connection<AppCentralDb> {
    fn connection(&mut self) -> &mut PgConnection {
        self.deref_mut()
    }
}
