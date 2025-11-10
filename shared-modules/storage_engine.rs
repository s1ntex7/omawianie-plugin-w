// ============================================================================
// STORAGE ENGINE - Shared Module for Aplikacja 3.0/4.0
// ============================================================================
// Used by: Clipboard History 2.0, Sticky Notes, Settings, File Manager AI
// Purpose: Unified SQLite storage with full-text search and migrations
// Tech: rusqlite (SQLite wrapper)
// ============================================================================

use rusqlite::{params, Connection, Result as SqliteResult};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// Database file path
    pub db_path: PathBuf,

    /// Enable full-text search (FTS5)?
    pub enable_fts: bool,

    /// Auto-vacuum on close?
    pub auto_vacuum: bool,
}

/// Storage engine with SQLite backend
pub struct StorageEngine {
    connection: Arc<Mutex<Connection>>,
    config: StorageConfig,
}

impl StorageEngine {
    /// Create new storage engine
    ///
    /// # Example
    /// ```
    /// let db_path = app.path().app_data_dir()?.join("data.db");
    /// let engine = StorageEngine::new(db_path)?;
    /// ```
    pub fn new(db_path: PathBuf) -> Result<Self, String> {
        let config = StorageConfig {
            db_path: db_path.clone(),
            enable_fts: true,
            auto_vacuum: true,
        };

        Self::with_config(config)
    }

    /// Create with custom config
    pub fn with_config(config: StorageConfig) -> Result<Self, String> {
        let conn = Connection::open(&config.db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        // Enable WAL mode for better concurrency
        conn.execute("PRAGMA journal_mode=WAL", [])
            .map_err(|e| format!("Failed to enable WAL: {}", e))?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys=ON", [])
            .map_err(|e| format!("Failed to enable foreign keys: {}", e))?;

        let engine = Self {
            connection: Arc::new(Mutex::new(conn)),
            config,
        };

        // Run migrations
        engine.migrate()?;

        Ok(engine)
    }

    /// Run database migrations
    fn migrate(&self) -> Result<(), String> {
        let conn = self.connection.lock().unwrap();

        // Create migrations table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY,
                version INTEGER NOT NULL UNIQUE,
                applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .map_err(|e| format!("Failed to create migrations table: {}", e))?;

        // Get current version
        let current_version: i32 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM migrations",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        // Apply migrations
        if current_version < 1 {
            self.migrate_v1(&conn)?;
        }

        Ok(())
    }

    /// Migration v1: Create initial tables
    fn migrate_v1(&self, conn: &Connection) -> Result<(), String> {
        // Clipboard History table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS clipboard_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content_type TEXT NOT NULL, -- 'text' | 'image' | 'file'
                content_text TEXT,
                content_blob BLOB,
                timestamp INTEGER NOT NULL,
                pinned INTEGER DEFAULT 0,
                label TEXT,
                source TEXT DEFAULT 'external'
            )",
            [],
        )
        .map_err(|e| format!("Failed to create clipboard_history table: {}", e))?;

        // Sticky Notes table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sticky_notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT,
                content TEXT NOT NULL,
                position_x INTEGER,
                position_y INTEGER,
                width INTEGER DEFAULT 250,
                height INTEGER DEFAULT 200,
                color TEXT DEFAULT '#FFFF88',
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                visible INTEGER DEFAULT 1
            )",
            [],
        )
        .map_err(|e| format!("Failed to create sticky_notes table: {}", e))?;

        // Settings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )
        .map_err(|e| format!("Failed to create settings table: {}", e))?;

        // Full-text search for clipboard history (if enabled)
        if self.config.enable_fts {
            conn.execute(
                "CREATE VIRTUAL TABLE IF NOT EXISTS clipboard_history_fts USING fts5(
                    content_text,
                    label,
                    content=clipboard_history,
                    content_rowid=id
                )",
                [],
            )
            .map_err(|e| format!("Failed to create FTS table: {}", e))?;

            // Triggers to keep FTS in sync
            conn.execute(
                "CREATE TRIGGER IF NOT EXISTS clipboard_history_ai AFTER INSERT ON clipboard_history BEGIN
                    INSERT INTO clipboard_history_fts(rowid, content_text, label)
                    VALUES (new.id, new.content_text, new.label);
                END",
                [],
            )
            .map_err(|e| format!("Failed to create FTS insert trigger: {}", e))?;

            conn.execute(
                "CREATE TRIGGER IF NOT EXISTS clipboard_history_ad AFTER DELETE ON clipboard_history BEGIN
                    DELETE FROM clipboard_history_fts WHERE rowid = old.id;
                END",
                [],
            )
            .map_err(|e| format!("Failed to create FTS delete trigger: {}", e))?;

            conn.execute(
                "CREATE TRIGGER IF NOT EXISTS clipboard_history_au AFTER UPDATE ON clipboard_history BEGIN
                    UPDATE clipboard_history_fts SET content_text = new.content_text, label = new.label
                    WHERE rowid = new.id;
                END",
                [],
            )
            .map_err(|e| format!("Failed to create FTS update trigger: {}", e))?;
        }

        // Record migration
        conn.execute(
            "INSERT INTO migrations (version) VALUES (1)",
            [],
        )
        .map_err(|e| format!("Failed to record migration: {}", e))?;

        Ok(())
    }

    /// Execute raw SQL query
    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize, String> {
        self.connection
            .lock()
            .unwrap()
            .execute(sql, params)
            .map_err(|e| format!("SQL execute error: {}", e))
    }

    /// Query with JSON result
    ///
    /// # Example
    /// ```
    /// let results: Vec<ClipboardEntry> = engine.query(
    ///     "SELECT * FROM clipboard_history WHERE pinned = 1",
    ///     &[]
    /// )?;
    /// ```
    pub fn query<T: for<'de> Deserialize<'de>>(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::ToSql],
    ) -> Result<Vec<T>, String> {
        // TODO: Implement query with serde_json deserialization
        //
        // Steps:
        // 1. Execute query
        // 2. Map rows to JSON objects
        // 3. Deserialize to T
        //
        // This is a helper for complex queries returning structured data

        Err("Not implemented - use execute() for raw SQL".to_string())
    }

    /// Full-text search in clipboard history
    ///
    /// # Example
    /// ```
    /// let results = engine.search_clipboard("meeting notes")?;
    /// ```
    pub fn search_clipboard(&self, query: &str) -> Result<Vec<ClipboardHistoryRow>, String> {
        let conn = self.connection.lock().unwrap();

        let mut stmt = conn
            .prepare(
                "SELECT ch.* FROM clipboard_history ch
                 INNER JOIN clipboard_history_fts fts ON ch.id = fts.rowid
                 WHERE clipboard_history_fts MATCH ?1
                 ORDER BY ch.timestamp DESC
                 LIMIT 100",
            )
            .map_err(|e| format!("Failed to prepare search query: {}", e))?;

        let rows = stmt
            .query_map([query], |row| {
                Ok(ClipboardHistoryRow {
                    id: row.get(0)?,
                    content_type: row.get(1)?,
                    content_text: row.get(2)?,
                    timestamp: row.get(4)?,
                    pinned: row.get(5)?,
                    label: row.get(6)?,
                })
            })
            .map_err(|e| format!("Failed to execute search: {}", e))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| format!("Failed to parse row: {}", e))?);
        }

        Ok(results)
    }

    /// Save setting
    pub fn set_setting(&self, key: &str, value: &str) -> Result<(), String> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)",
            params![key, value, chrono::Utc::now().timestamp()],
        )
        .map_err(|e| format!("Failed to save setting: {}", e))?;
        Ok(())
    }

    /// Get setting
    pub fn get_setting(&self, key: &str) -> Result<Option<String>, String> {
        let conn = self.connection.lock().unwrap();
        let result = conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get(0),
        );

        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get setting: {}", e)),
        }
    }

    /// Vacuum database (cleanup, optimize)
    pub fn vacuum(&self) -> Result<(), String> {
        let conn = self.connection.lock().unwrap();
        conn.execute("VACUUM", [])
            .map_err(|e| format!("Failed to vacuum: {}", e))?;
        Ok(())
    }
}

/// Clipboard history row
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardHistoryRow {
    pub id: i64,
    pub content_type: String,
    pub content_text: Option<String>,
    pub timestamp: i64,
    pub pinned: bool,
    pub label: Option<String>,
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This module should be placed in: src-tauri/src/storage_engine.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod storage_engine;
//    use storage_engine::StorageEngine;
//
//    // In .setup():
//    let db_path = app.path().app_data_dir()?.join("aplikacja_3_0.db");
//    let storage = StorageEngine::new(db_path)?;
//    app.manage(storage);
//    ```
//
// 3. Dependencies to add to Cargo.toml:
//    ```toml
//    rusqlite = { version = "0.32", features = ["bundled", "chrono"] }
//    serde = { version = "1.0", features = ["derive"] }
//    serde_json = "1.0"
//    chrono = "0.4"
//    ```
//
// 4. Usage in plugins:
//    - Clipboard History 2.0: Store all clipboard entries with FTS
//    - Sticky Notes: Persist notes with position, color, visibility
//    - Settings: Global app settings (screenshot folder, hotkeys, etc)
//    - File Manager AI: Track categorization rules
//
// 5. IPC Commands to add:
//    ```rust
//    #[tauri::command]
//    fn storage_set_setting(key: String, value: String, state: tauri::State<StorageEngine>) -> Result<(), String> {
//        state.set_setting(&key, &value)
//    }
//
//    #[tauri::command]
//    fn storage_get_setting(key: String, state: tauri::State<StorageEngine>) -> Result<Option<String>, String> {
//        state.get_setting(&key)
//    }
//
//    #[tauri::command]
//    fn storage_search_clipboard(query: String, state: tauri::State<StorageEngine>) -> Result<Vec<ClipboardHistoryRow>, String> {
//        state.search_clipboard(&query)
//    }
//    ```
//
// 6. Database location:
//    - Windows: `C:\Users\<user>\AppData\Roaming\com.aplikacja.3-0\aplikacja_3_0.db`
//    - macOS: `~/Library/Application Support/com.aplikacja.3-0/aplikacja_3_0.db`
//    - Linux: `~/.local/share/com.aplikacja.3-0/aplikacja_3_0.db`
//
// 7. Migrations:
//    - Add new migrate_vX() functions as needed
//    - Always increment version in migrations table
//    - Never modify existing migrations (append only!)
//
// ============================================================================
