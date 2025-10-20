use rocket::{
    Build, Rocket,
    fairing::{self, AdHoc},
    fs::FileServer,
};
use rocket_db_pools::Database;

use crate::database::AppCentralDb;

#[macro_use]
extern crate rocket;

extern crate rocket_db_pools;

mod database;
mod errors;
mod recipes;

#[launch]
async fn rocket() -> _ {
    dotenvy::dotenv().ok();

    let migrations_fairing = AdHoc::try_on_ignite("SQLx Migrations", run_migrations);

    rocket::build()
        .attach(AppCentralDb::init())
        .attach(migrations_fairing)
        .mount("/", FileServer::from("frontend"))
        .mount("/api/recipes", recipes::routes())
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match AppCentralDb::fetch(&rocket) {
        Some(db) => match sqlx::migrate!().run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to run database migrations: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}
