use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

const DB_PATH           : &str = "./data.sqlite";
const MAX_CONNECTIONS   : u32  = 10;

pub async fn load_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let connect_options = SqliteConnectOptions::new()
        .filename(DB_PATH)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect_with(connect_options)
        .await?;

    Ok(pool)
}
