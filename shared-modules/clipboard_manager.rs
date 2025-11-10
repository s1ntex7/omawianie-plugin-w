// ============================================================================
// CLIPBOARD MANAGER - Shared Module for Aplikacja 3.0/4.0
// ============================================================================
// Used by: Clipboard History 2.0, QR Generator, URL Shortener, Screenshot Tool
// Purpose: Unified clipboard API with history tracking and cross-platform support
// Tech: arboard crate (already used in voice_to_text.rs)
// ============================================================================

use arboard::{Clipboard, ImageData};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

/// Clipboard content types
#[derive(Debug, Clone)]
pub enum ClipboardContent {
    /// Plain text
    Text(String),

    /// Image (PNG/JPEG/etc)
    Image {
        width: usize,
        height: usize,
        rgba_data: Vec<u8>,
    },

    /// File path (for file operations)
    FilePath(String),

    /// Unsupported/Empty
    Empty,
}

/// Clipboard history entry
#[derive(Debug, Clone)]
pub struct ClipboardEntry {
    /// Unique ID
    pub id: u64,

    /// Content
    pub content: ClipboardContent,

    /// Timestamp
    pub timestamp: SystemTime,

    /// Is pinned (favorite)?
    pub pinned: bool,

    /// Optional label
    pub label: Option<String>,
}

/// Clipboard change event
#[derive(Debug, Clone)]
pub struct ClipboardEvent {
    pub entry: ClipboardEntry,
    pub source: ClipboardSource,
}

/// Source of clipboard change
#[derive(Debug, Clone, Copy)]
pub enum ClipboardSource {
    /// Changed by our app
    Internal,

    /// Changed by external app
    External,

    /// Unknown
    Unknown,
}

/// Clipboard manager with history tracking
pub struct ClipboardManager {
    clipboard: Arc<Mutex<Clipboard>>,
    history: Arc<Mutex<Vec<ClipboardEntry>>>,
    next_id: Arc<Mutex<u64>>,
}

impl ClipboardManager {
    /// Create new clipboard manager
    pub fn new() -> Result<Self, String> {
        let clipboard = Clipboard::new()
            .map_err(|e| format!("Failed to initialize clipboard: {}", e))?;

        Ok(Self {
            clipboard: Arc::new(Mutex::new(clipboard)),
            history: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(0)),
        })
    }

    /// Paste text to clipboard
    ///
    /// # Example
    /// ```
    /// manager.paste_text("Hello World!")?;
    /// ```
    pub fn paste_text(&self, text: &str) -> Result<(), String> {
        self.clipboard
            .lock()
            .unwrap()
            .set_text(text)
            .map_err(|e| format!("Failed to paste text: {}", e))?;

        // Add to history
        self.add_to_history(ClipboardContent::Text(text.to_string()), ClipboardSource::Internal);

        Ok(())
    }

    /// Paste image to clipboard
    ///
    /// # Example
    /// ```
    /// let img = image::open("screenshot.png")?;
    /// manager.paste_image(&img)?;
    /// ```
    pub fn paste_image(&self, width: usize, height: usize, rgba_data: Vec<u8>) -> Result<(), String> {
        let img_data = ImageData {
            width,
            height,
            bytes: rgba_data.clone().into(),
        };

        self.clipboard
            .lock()
            .unwrap()
            .set_image(img_data)
            .map_err(|e| format!("Failed to paste image: {}", e))?;

        // Add to history
        self.add_to_history(
            ClipboardContent::Image {
                width,
                height,
                rgba_data,
            },
            ClipboardSource::Internal,
        );

        Ok(())
    }

    /// Get current clipboard text
    pub fn get_text(&self) -> Result<String, String> {
        self.clipboard
            .lock()
            .unwrap()
            .get_text()
            .map_err(|e| format!("Failed to get clipboard text: {}", e))
    }

    /// Get current clipboard image
    pub fn get_image(&self) -> Result<(usize, usize, Vec<u8>), String> {
        let img = self
            .clipboard
            .lock()
            .unwrap()
            .get_image()
            .map_err(|e| format!("Failed to get clipboard image: {}", e))?;

        Ok((img.width, img.height, img.bytes.to_vec()))
    }

    /// Start watching clipboard for changes (for Clipboard History 2.0)
    ///
    /// Returns a channel receiver for clipboard events.
    ///
    /// # Example
    /// ```
    /// let (tx, rx) = std::sync::mpsc::channel();
    /// manager.watch_changes(tx)?;
    ///
    /// // In background thread:
    /// while let Ok(event) = rx.recv() {
    ///     println!("Clipboard changed: {:?}", event.entry.content);
    ///     save_to_database(&event.entry)?;
    /// }
    /// ```
    pub fn watch_changes(
        &self,
        sender: std::sync::mpsc::Sender<ClipboardEvent>,
    ) -> Result<(), String> {
        // TODO: Implement background watcher
        //
        // Steps:
        // 1. Spawn background thread
        // 2. Poll clipboard every 200ms
        // 3. Compare with previous content (hash)
        // 4. If changed, send ClipboardEvent
        // 5. Handle text + images
        //
        // IMPORTANT: Don't send events for our own paste_text/paste_image calls
        // - Track last internal change timestamp
        // - Ignore changes within 500ms of internal paste
        //
        // See: MASTER-PLAN Phase 2 for Clipboard History 2.0 implementation

        Err("Not implemented - see watch_changes TODO".to_string())
    }

    /// Get clipboard history (all entries)
    pub fn get_history(&self) -> Vec<ClipboardEntry> {
        self.history.lock().unwrap().clone()
    }

    /// Get pinned entries only
    pub fn get_pinned(&self) -> Vec<ClipboardEntry> {
        self.history
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.pinned)
            .cloned()
            .collect()
    }

    /// Pin/unpin entry
    pub fn toggle_pin(&self, entry_id: u64) -> Result<(), String> {
        let mut history = self.history.lock().unwrap();
        if let Some(entry) = history.iter_mut().find(|e| e.id == entry_id) {
            entry.pinned = !entry.pinned;
            Ok(())
        } else {
            Err(format!("Entry {} not found", entry_id))
        }
    }

    /// Search history (fuzzy text search)
    pub fn search(&self, query: &str) -> Vec<ClipboardEntry> {
        let history = self.history.lock().unwrap();
        let query_lower = query.to_lowercase();

        history
            .iter()
            .filter(|entry| {
                if let ClipboardContent::Text(text) = &entry.content {
                    text.to_lowercase().contains(&query_lower)
                } else if let Some(label) = &entry.label {
                    label.to_lowercase().contains(&query_lower)
                } else {
                    false
                }
            })
            .cloned()
            .collect()
    }

    /// Clear history (keep pinned entries)
    pub fn clear_history(&self, keep_pinned: bool) {
        let mut history = self.history.lock().unwrap();
        if keep_pinned {
            history.retain(|e| e.pinned);
        } else {
            history.clear();
        }
    }

    /// Add entry to history (internal helper)
    fn add_to_history(&self, content: ClipboardContent, source: ClipboardSource) {
        let mut history = self.history.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

        let entry = ClipboardEntry {
            id: *next_id,
            content,
            timestamp: SystemTime::now(),
            pinned: false,
            label: None,
        };

        *next_id += 1;
        history.push(entry.clone());

        // TODO: Emit event to IPC for UI update
        // app.emit_to("main", "clipboard:history-updated", &entry)?;
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new().expect("Failed to create ClipboardManager")
    }
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This module should be placed in: src-tauri/src/clipboard_manager.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod clipboard_manager;
//    use clipboard_manager::ClipboardManager;
//
//    // Create global instance
//    let clipboard_mgr = ClipboardManager::new()?;
//    app.manage(clipboard_mgr); // Tauri state management
//    ```
//
// 3. Dependencies (already in project):
//    ```toml
//    arboard = "3.4.1"  # Already used in voice_to_text.rs
//    ```
//
// 4. IPC Commands to add:
//    ```rust
//    #[tauri::command]
//    fn clipboard_paste_text(text: String, state: tauri::State<ClipboardManager>) -> Result<(), String> {
//        state.paste_text(&text)
//    }
//
//    #[tauri::command]
//    fn clipboard_get_history(state: tauri::State<ClipboardManager>) -> Vec<ClipboardEntry> {
//        state.get_history()
//    }
//
//    #[tauri::command]
//    fn clipboard_search(query: String, state: tauri::State<ClipboardManager>) -> Vec<ClipboardEntry> {
//        state.search(&query)
//    }
//
//    #[tauri::command]
//    fn clipboard_toggle_pin(id: u64, state: tauri::State<ClipboardManager>) -> Result<(), String> {
//        state.toggle_pin(id)
//    }
//    ```
//
// 5. For Clipboard History 2.0 plugin:
//    - Implement watch_changes() in background thread
//    - Store entries in SQLite (see storage_engine.rs)
//    - Add global hotkey (Ctrl+Shift+V) to show popup
//    - Create React UI with fuzzy search
//
// 6. Usage in other plugins:
//    - QR Generator: paste_text() to copy QR code data URL
//    - URL Shortener: paste_text() to copy shortened URL
//    - Screenshot Tool: paste_image() after saving screenshot
//
// ============================================================================
