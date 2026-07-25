use std::path::PathBuf;

use directories::ProjectDirs;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db() -> &'static DatabaseConnection {
    DB.get_or_init(|| async {
        let conn = Database::connect("sqlite://ferrum_launcher.db?mode=rwc")
            .await
            .expect("Db not created");

        conn.get_schema_registry("ferrum_launcher::db::entity::*")
            .sync(&conn)
            .await
            .expect("schema not registry");

        conn
    })
    .await
}

fn get_db_path() -> PathBuf {
    if let Some(proj) = ProjectDirs::from("com", "ferrum", "FerrumLauncher") {
        let data_dir = proj.data_dir();
        return data_dir.join("profiles.db");
    }
    PathBuf::new()
}
