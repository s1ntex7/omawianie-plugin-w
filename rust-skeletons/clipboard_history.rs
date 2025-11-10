// ============================================================================
// CLIPBOARD HISTORY 2.0 - Killer Feature
// ============================================================================
// Priority: P0 🔥 - Must-have killer feature
// Tech: clipboard_manager + storage_engine + fuzzy search
// Hotkey: Ctrl+Shift+V (show popup)
// Features: History tracking, search, pin favorites, images + text
// Estimated time: 25 hours (Phase 2)
// ============================================================================

use crate::clipboard_manager::{ClipboardManager, ClipboardEntry, ClipboardEvent, ClipboardContent};
use crate::storage_engine::{StorageEngine, ClipboardHistoryRow};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use chrono::Utc;

/// Clipboard history settings
#[derive(Debug, Clone)]
pub struct HistorySettings {
    /// Maximum entries to keep (0 = unlimited)
    pub max_entries: usize,

    /// Enable image history?
    pub track_images: bool,

    /// Enable text history?
    pub track_text: bool,

    /// Auto-delete entries older than N days (0 = never)
    pub auto_delete_days: u32,
}

impl Default for HistorySettings {
    fn default() -> Self {
        Self {
            max_entries: 1000,
            track_images: true,
            track_text: true,
            auto_delete_days: 30,
        }
    }
}

/// Clipboard history manager
pub struct ClipboardHistory {
    clipboard_manager: Arc<Mutex<ClipboardManager>>,
    storage: Arc<Mutex<StorageEngine>>,
    settings: Arc<Mutex<HistorySettings>>,
    watcher_running: Arc<Mutex<bool>>,
}

impl ClipboardHistory {
    pub fn new(
        clipboard_manager: Arc<Mutex<ClipboardManager>>,
        storage: Arc<Mutex<StorageEngine>>,
    ) -> Self {
        Self {
            clipboard_manager,
            storage,
            settings: Arc::new(Mutex::new(HistorySettings::default())),
            watcher_running: Arc::new(Mutex::new(false)),
        }
    }

    /// Start watching clipboard for changes
    ///
    /// This runs in a background thread and saves all clipboard changes to database.
    pub fn start_watching(&self) -> Result<(), String> {
        let mut running = self.watcher_running.lock().unwrap();
        if *running {
            return Err("Clipboard watcher already running".to_string());
        }
        *running = true;

        let (tx, rx) = mpsc::channel::<ClipboardEvent>();

        // Start clipboard watcher
        let clipboard_mgr = self.clipboard_manager.clone();
        clipboard_mgr.lock().unwrap().watch_changes(tx)?;

        // Spawn handler thread
        let storage = self.storage.clone();
        let settings = self.settings.clone();
        let running_clone = self.watcher_running.clone();

        thread::spawn(move || {
            while let Ok(event) = rx.recv() {
                // Check if watcher should stop
                if !*running_clone.lock().unwrap() {
                    break;
                }

                // Save to database
                if let Err(e) = Self::save_entry_to_db(&storage, &settings, &event.entry) {
                    eprintln!("Failed to save clipboard entry: {}", e);
                }

                // TODO: Emit IPC event to UI
                // app.emit_to("main", "clipboard:history-updated", &event.entry)?;
            }
        });

        Ok(())
    }

    /// Stop watching clipboard
    pub fn stop_watching(&self) {
        *self.watcher_running.lock().unwrap() = false;
    }

    /// Save entry to database
    fn save_entry_to_db(
        storage: &Arc<Mutex<StorageEngine>>,
        settings: &Arc<Mutex<HistorySettings>>,
        entry: &ClipboardEntry,
    ) -> Result<(), String> {
        let settings = settings.lock().unwrap();
        let storage = storage.lock().unwrap();

        match &entry.content {
            ClipboardContent::Text(text) if settings.track_text => {
                storage.execute(
                    "INSERT INTO clipboard_history (content_type, content_text, timestamp, pinned, label, source)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    &[
                        &"text" as &dyn rusqlite::ToSql,
                        &text,
                        &entry.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as &dyn rusqlite::ToSql,
                        &(entry.pinned as i32),
                        &entry.label,
                        &"external",
                    ],
                )?;
            }
            ClipboardContent::Image { width, height, rgba_data } if settings.track_images => {
                storage.execute(
                    "INSERT INTO clipboard_history (content_type, content_blob, timestamp, pinned, source)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    &[
                        &"image" as &dyn rusqlite::ToSql,
                        &rgba_data.as_slice(),
                        &entry.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as &dyn rusqlite::ToSql,
                        &(entry.pinned as i32),
                        &"external",
                    ],
                )?;
            }
            _ => {}
        }

        // Cleanup old entries if max_entries exceeded
        if settings.max_entries > 0 {
            storage.execute(
                "DELETE FROM clipboard_history WHERE id NOT IN (
                    SELECT id FROM clipboard_history ORDER BY timestamp DESC LIMIT ?1
                )",
                &[&settings.max_entries as &dyn rusqlite::ToSql],
            )?;
        }

        // Auto-delete old entries
        if settings.auto_delete_days > 0 {
            let cutoff = Utc::now().timestamp() - (settings.auto_delete_days as i64 * 86400);
            storage.execute(
                "DELETE FROM clipboard_history WHERE timestamp < ?1 AND pinned = 0",
                &[&cutoff as &dyn rusqlite::ToSql],
            )?;
        }

        Ok(())
    }

    /// Get recent history (last N entries)
    pub fn get_recent(&self, limit: usize) -> Result<Vec<ClipboardHistoryRow>, String> {
        let storage = self.storage.lock().unwrap();

        // TODO: Implement SQL query with limit
        //
        // ```sql
        // SELECT * FROM clipboard_history
        // ORDER BY timestamp DESC
        // LIMIT ?1
        // ```

        Err("Not implemented".to_string())
    }

    /// Search history (full-text search)
    pub fn search(&self, query: &str) -> Result<Vec<ClipboardHistoryRow>, String> {
        let storage = self.storage.lock().unwrap();
        storage.search_clipboard(query)
    }

    /// Get pinned entries
    pub fn get_pinned(&self) -> Result<Vec<ClipboardHistoryRow>, String> {
        let storage = self.storage.lock().unwrap();

        // TODO: Implement SQL query
        //
        // ```sql
        // SELECT * FROM clipboard_history
        // WHERE pinned = 1
        // ORDER BY timestamp DESC
        // ```

        Err("Not implemented".to_string())
    }

    /// Toggle pin status
    pub fn toggle_pin(&self, entry_id: i64) -> Result<bool, String> {
        let storage = self.storage.lock().unwrap();

        storage.execute(
            "UPDATE clipboard_history SET pinned = 1 - pinned WHERE id = ?1",
            &[&entry_id as &dyn rusqlite::ToSql],
        )?;

        // Return new pin status
        // TODO: Query and return actual value

        Ok(true)
    }

    /// Delete entry
    pub fn delete_entry(&self, entry_id: i64) -> Result<(), String> {
        let storage = self.storage.lock().unwrap();

        storage.execute(
            "DELETE FROM clipboard_history WHERE id = ?1",
            &[&entry_id as &dyn rusqlite::ToSql],
        )?;

        Ok(())
    }

    /// Clear all history (keep pinned if specified)
    pub fn clear_history(&self, keep_pinned: bool) -> Result<(), String> {
        let storage = self.storage.lock().unwrap();

        if keep_pinned {
            storage.execute("DELETE FROM clipboard_history WHERE pinned = 0", &[])?;
        } else {
            storage.execute("DELETE FROM clipboard_history", &[])?;
        }

        Ok(())
    }

    /// Paste entry back to clipboard
    pub fn paste_entry(&self, entry_id: i64) -> Result<(), String> {
        // TODO: Implement
        //
        // Steps:
        // 1. Query entry from database
        // 2. Determine content type (text vs image)
        // 3. Call clipboard_manager.paste_text() or paste_image()
        // 4. Update "last used" timestamp (optional)

        Err("Not implemented".to_string())
    }

    /// Update settings
    pub fn update_settings(&self, new_settings: HistorySettings) {
        *self.settings.lock().unwrap() = new_settings;
    }

    /// Get current settings
    pub fn get_settings(&self) -> HistorySettings {
        self.settings.lock().unwrap().clone()
    }
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This file should be placed in: src-tauri/src/clipboard_history.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod clipboard_history;
//    use clipboard_history::ClipboardHistory;
//
//    let clipboard_history = ClipboardHistory::new(
//        clipboard_mgr.clone(),
//        storage.clone()
//    );
//    clipboard_history.start_watching()?;
//    app.manage(clipboard_history);
//
//    // Register Ctrl+Shift+V hotkey to show popup
//    gs.on_shortcut("Ctrl+Shift+V", {
//        let app = app.handle().clone();
//        move |_app, _shortcut, event| {
//            if format!("{:?}", event).contains("Pressed") {
//                let _ = app.emit_to("main", "clipboard-history:show-popup", ());
//            }
//        }
//    })?;
//    ```
//
// 3. IPC Commands:
//    ```rust
//    #[tauri::command]
//    fn clipboard_history_search(query: String, state: tauri::State<ClipboardHistory>) -> Result<Vec<ClipboardHistoryRow>, String> {
//        state.search(&query)
//    }
//
//    #[tauri::command]
//    fn clipboard_history_recent(limit: usize, state: tauri::State<ClipboardHistory>) -> Result<Vec<ClipboardHistoryRow>, String> {
//        state.get_recent(limit)
//    }
//
//    #[tauri::command]
//    fn clipboard_history_paste(entry_id: i64, state: tauri::State<ClipboardHistory>) -> Result<(), String> {
//        state.paste_entry(entry_id)
//    }
//
//    #[tauri::command]
//    fn clipboard_history_toggle_pin(entry_id: i64, state: tauri::State<ClipboardHistory>) -> Result<bool, String> {
//        state.toggle_pin(entry_id)
//    }
//    ```
//
// 4. React UI Component (Popup):
//    ```tsx
//    const ClipboardHistoryPopup = () => {
//      const [entries, setEntries] = useState<ClipboardHistoryRow[]>([]);
//      const [query, setQuery] = useState("");
//
//      const search = async () => {
//        const results = await invoke<ClipboardHistoryRow[]>(
//          query ? "clipboard_history_search" : "clipboard_history_recent",
//          query ? { query } : { limit: 50 }
//        );
//        setEntries(results);
//      };
//
//      const paste = async (id: number) => {
//        await invoke("clipboard_history_paste", { entryId: id });
//        // Close popup
//      };
//
//      return (
//        <div className="popup">
//          <input
//            value={query}
//            onChange={(e) => { setQuery(e.target.value); search(); }}
//            placeholder="Search clipboard history..."
//            autoFocus
//          />
//          <div className="entries">
//            {entries.map((entry) => (
//              <div key={entry.id} onClick={() => paste(entry.id)}>
//                {entry.content_text || <img src={...} />}
//              </div>
//            ))}
//          </div>
//        </div>
//      );
//    };
//    ```
//
// 5. Testing checklist:
//    - [ ] Clipboard changes are tracked automatically
//    - [ ] Text and images both saved
//    - [ ] Fuzzy search works correctly
//    - [ ] Pinned entries never deleted
//    - [ ] Ctrl+Shift+V shows popup
//    - [ ] Clicking entry pastes to clipboard
//    - [ ] Old entries auto-deleted based on settings
//
// ============================================================================
