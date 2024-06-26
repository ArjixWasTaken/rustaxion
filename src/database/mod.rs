use std::env;
use sea_orm::{ ConnectOptions, Database, DatabaseConnection };

pub mod entities;
pub mod migrations;
pub mod helpers;

pub async fn establish_connection() -> anyhow::Result<DatabaseConnection> {
    let connection_uri = env
        ::var("DATABASE_URL")
        .unwrap_or("sqlite://sqlite.db?mode=rwc".to_string());

    let mut opt = ConnectOptions::new(connection_uri);
    opt.sqlx_logging(false);

    let db = Database::connect(opt).await?;

    use migrations::MigratorTrait;
    migrations::Migrator::up(&db, None).await?;

    Ok(db)
}
