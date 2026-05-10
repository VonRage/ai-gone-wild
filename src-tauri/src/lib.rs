use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

pub mod db;

struct DbState {
  conn: Mutex<Connection>,
}

#[derive(serde::Serialize)]
pub struct Game {
    id: i64,
    executable: String,
    profile_id: Option<i64>,
}

#[tauri::command]
fn get_games(state: tauri::State<'_, DbState>) -> Result<Vec<Game>, String> {
    let conn = state.conn.lock().map_err(|_| "Failed to lock database mutex")?;
    let mut stmt = conn.prepare("SELECT id, executable, profile_id FROM games").map_err(|e| e.to_string())?;

    let games_iter = stmt.query_map([], |row| {
        Ok(Game {
            id: row.get(0)?,
            executable: row.get(1)?,
            profile_id: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut games = Vec::new();
    for game in games_iter {
        games.push(game.map_err(|e| e.to_string())?);
    }

    Ok(games)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_games])
    .setup(|app| {
      let data_dir = app.path().app_data_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
      std::fs::create_dir_all(&data_dir).expect("Failed to create app data dir");
      let db_path = data_dir.join("telemetry.sqlite");

      let conn = Connection::open(&db_path).expect("Failed to open local SQLite database");
      db::initialize_database(&conn).expect("Failed to initialize database schema");

      app.manage(DbState {
        conn: Mutex::new(conn),
      });

      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
