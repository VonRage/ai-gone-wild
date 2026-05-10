use rusqlite::{Connection, Result};

pub fn initialize_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS profiles (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            limits TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
            id INTEGER PRIMARY KEY,
            executable TEXT NOT NULL,
            profile_id INTEGER,
            FOREIGN KEY(profile_id) REFERENCES profiles(id)
        )",
        [],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_initialization() {
        let conn = Connection::open_in_memory().unwrap();

        // This will fail because we haven't implemented initialize_database yet
        initialize_database(&conn).unwrap();

        // Verify tables exist by inserting dummy rows
        conn.execute(
            "INSERT INTO profiles (id, name, limits) VALUES (?1, ?2, ?3)",
            (1, "Default", "{}"),
        ).expect("Failed to insert profile");

        conn.execute(
            "INSERT INTO games (id, executable, profile_id) VALUES (?1, ?2, ?3)",
            (1, "game.exe", 1),
        ).expect("Failed to insert game");
    }
}
