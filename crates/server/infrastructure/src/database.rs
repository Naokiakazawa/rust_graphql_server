use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::env;

pub async fn establish_db_connection() -> Result<DatabaseConnection, DbErr> {
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt: ConnectOptions = ConnectOptions::new(db_url);
    opt.sqlx_logging(true);

    let db: DatabaseConnection = Database::connect(opt).await?;
    Ok(db)
}
