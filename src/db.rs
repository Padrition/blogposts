use deadpool_diesel::postgres::Pool;
use dotenvy::dotenv;
use std::env;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn setup_connection_pool()-> Pool{
    dotenv().ok();

    let database_url = env::var("DATABASE_DOCKER_URL").expect("DATABASE_URL must be set");
    let manager = deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    deadpool_diesel::postgres::Pool::builder(manager)
    .build()
    .unwrap()
}

pub async fn run_migrations(pool: &Pool) {
    let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_|()))
        .await
        .unwrap()
        .unwrap();
}