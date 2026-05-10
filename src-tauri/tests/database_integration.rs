use app_lib::database; // Jules will need to create this module

#[tokio::test]
async fn test_database_initialization_and_schema() {
    // 1. Arrange: Use an in-memory database to prevent test pollution
    // Jules MUST design init_db to accept a connection string or path, not an AppHandle.
    let db_url = "sqlite::memory:";

    // 2. Act: Call the target function
    let pool = database::init_db(db_url).await.expect("Database initialization failed");

    // 3. Assert: Verify the schema matches ADR requirements
    
    // Check 'games' table
    let games_table = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='games'")
        .fetch_optional(&pool)
        .await
        .expect("Failed to query schema");
    assert!(games_table.is_some(), "Table 'games' must exist");

    // Check 'reports' table
    let reports_table = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='reports'")
        .fetch_optional(&pool)
        .await
        .expect("Failed to query schema");
    assert!(reports_table.is_some(), "Table 'reports' must exist");

    // Check 'summaries' table
    let summaries_table = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='summaries'")
        .fetch_optional(&pool)
        .await
        .expect("Failed to query schema");
    assert!(summaries_table.is_some(), "Table 'summaries' must exist");
}
