// ============================================================================
// STICKY NOTES - Floating Notes
// ============================================================================
// Priority: P1 (High Value)
// Tech: egui + storage_engine for persistence
// Features: Create, edit, delete, position, color notes - always visible
// Estimated time: 10 hours
// ============================================================================

use crate::storage_engine::StorageEngine;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use chrono::Utc;

/// Sticky note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickyNote {
    pub id: i64,
    pub title: Option<String>,
    pub content: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: u32,
    pub height: u32,
    pub color: String, // Hex color, e.g., "#FFFF88"
    pub created_at: i64,
    pub updated_at: i64,
    pub visible: bool,
}

impl Default for StickyNote {
    fn default() -> Self {
        Self {
            id: 0,
            title: None,
            content: String::new(),
            position_x: 100,
            position_y: 100,
            width: 250,
            height: 200,
            color: "#FFFF88".to_string(), // Classic yellow
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
            visible: true,
        }
    }
}

/// Sticky notes manager
pub struct StickyNotesManager {
    storage: Arc<Mutex<StorageEngine>>,
    notes: Arc<Mutex<Vec<StickyNote>>>,
}

impl StickyNotesManager {
    pub fn new(storage: Arc<Mutex<StorageEngine>>) -> Result<Self, String> {
        let manager = Self {
            storage: storage.clone(),
            notes: Arc::new(Mutex::new(Vec::new())),
        };

        // Load notes from database
        manager.load_notes()?;

        Ok(manager)
    }

    /// Load notes from database
    fn load_notes(&self) -> Result<(), String> {
        let storage = self.storage.lock().unwrap();

        // TODO: Implement SQL query to load all notes
        //
        // ```sql
        // SELECT * FROM sticky_notes WHERE visible = 1
        // ORDER BY created_at ASC
        // ```
        //
        // Then parse into Vec<StickyNote> and store in self.notes

        Ok(())
    }

    /// Create new note
    pub fn create_note(&self, note: StickyNote) -> Result<i64, String> {
        let storage = self.storage.lock().unwrap();

        storage.execute(
            "INSERT INTO sticky_notes (title, content, position_x, position_y, width, height, color, created_at, updated_at, visible)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            &[
                &note.title as &dyn rusqlite::ToSql,
                &note.content,
                &note.position_x,
                &note.position_y,
                &note.width,
                &note.height,
                &note.color,
                &note.created_at,
                &note.updated_at,
                &(note.visible as i32),
            ],
        )?;

        // Get last inserted ID
        // TODO: Query last_insert_rowid()

        let note_id = 1; // Placeholder

        // Add to in-memory list
        let mut notes = self.notes.lock().unwrap();
        let mut new_note = note;
        new_note.id = note_id;
        notes.push(new_note);

        Ok(note_id)
    }

    /// Update note
    pub fn update_note(&self, note: StickyNote) -> Result<(), String> {
        let storage = self.storage.lock().unwrap();

        let updated_at = Utc::now().timestamp();

        storage.execute(
            "UPDATE sticky_notes
             SET title = ?1, content = ?2, position_x = ?3, position_y = ?4,
                 width = ?5, height = ?6, color = ?7, updated_at = ?8, visible = ?9
             WHERE id = ?10",
            &[
                &note.title as &dyn rusqlite::ToSql,
                &note.content,
                &note.position_x,
                &note.position_y,
                &note.width,
                &note.height,
                &note.color,
                &updated_at,
                &(note.visible as i32),
                &note.id,
            ],
        )?;

        // Update in-memory list
        let mut notes = self.notes.lock().unwrap();
        if let Some(existing) = notes.iter_mut().find(|n| n.id == note.id) {
            *existing = note;
            existing.updated_at = updated_at;
        }

        Ok(())
    }

    /// Delete note
    pub fn delete_note(&self, note_id: i64) -> Result<(), String> {
        let storage = self.storage.lock().unwrap();

        storage.execute(
            "DELETE FROM sticky_notes WHERE id = ?1",
            &[&note_id as &dyn rusqlite::ToSql],
        )?;

        // Remove from in-memory list
        let mut notes = self.notes.lock().unwrap();
        notes.retain(|n| n.id != note_id);

        Ok(())
    }

    /// Get all visible notes
    pub fn get_visible_notes(&self) -> Vec<StickyNote> {
        self.notes
            .lock()
            .unwrap()
            .iter()
            .filter(|n| n.visible)
            .cloned()
            .collect()
    }

    /// Get note by ID
    pub fn get_note(&self, note_id: i64) -> Option<StickyNote> {
        self.notes
            .lock()
            .unwrap()
            .iter()
            .find(|n| n.id == note_id)
            .cloned()
    }

    /// Hide/show note
    pub fn toggle_visibility(&self, note_id: i64) -> Result<bool, String> {
        let mut notes = self.notes.lock().unwrap();
        if let Some(note) = notes.iter_mut().find(|n| n.id == note_id) {
            note.visible = !note.visible;
            drop(notes); // Release lock before calling update_note

            self.update_note(note.clone())?;
            Ok(note.visible)
        } else {
            Err(format!("Note {} not found", note_id))
        }
    }
}

// ============================================================================
// EGUI RENDERING (for native floating windows)
// ============================================================================

/// This is how you'd render Sticky Notes as egui floating windows
///
/// Example usage:
/// ```rust
/// let mut notes_renderer = StickyNotesRenderer::new(notes_manager);
/// notes_renderer.spawn_all_notes();
/// ```
pub struct StickyNotesRenderer {
    manager: Arc<Mutex<StickyNotesManager>>,
}

impl StickyNotesRenderer {
    pub fn new(manager: Arc<Mutex<StickyNotesManager>>) -> Self {
        Self { manager }
    }

    /// Spawn egui window for each visible note
    pub fn spawn_all_notes(&self) -> Result<(), String> {
        // TODO: Implement egui windows for sticky notes
        //
        // Steps:
        // 1. Get all visible notes from manager
        // 2. For each note, create eframe window with:
        //    - Position at note.position_x, position_y
        //    - Size: note.width x note.height
        //    - Background color: note.color
        //    - Always on top
        //    - Decorations = false (frameless)
        //    - Resizable = true (user can resize)
        //    - Draggable title bar
        // 3. Render note content as editable text area
        // 4. On edit, call manager.update_note()
        // 5. On close button, call manager.toggle_visibility()
        //
        // IMPORTANT: Each note runs in its own eframe::run_native() call
        // - Spawn thread for each note window
        // - Handle window events (move, resize, close)
        // - Save position/size changes back to database
        //
        // Alternative: Use Tauri WebView windows (one window per note)
        // - Create WebView window with note content as HTML
        // - Use React component for editing
        // - This is simpler than egui but uses more memory

        Err("Not implemented - choose egui or Tauri windows approach".to_string())
    }
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This file should be placed in: src-tauri/src/sticky_notes.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod sticky_notes;
//    use sticky_notes::StickyNotesManager;
//
//    let sticky_notes = StickyNotesManager::new(storage.clone())?;
//    app.manage(sticky_notes);
//
//    // Optionally spawn renderer
//    // let renderer = StickyNotesRenderer::new(Arc::new(Mutex::new(sticky_notes)));
//    // renderer.spawn_all_notes()?;
//    ```
//
// 3. IPC Commands:
//    ```rust
//    #[tauri::command]
//    fn sticky_notes_create(note: StickyNote, state: tauri::State<StickyNotesManager>) -> Result<i64, String> {
//        state.create_note(note)
//    }
//
//    #[tauri::command]
//    fn sticky_notes_update(note: StickyNote, state: tauri::State<StickyNotesManager>) -> Result<(), String> {
//        state.update_note(note)
//    }
//
//    #[tauri::command]
//    fn sticky_notes_delete(note_id: i64, state: tauri::State<StickyNotesManager>) -> Result<(), String> {
//        state.delete_note(note_id)
//    }
//
//    #[tauri::command]
//    fn sticky_notes_list(state: tauri::State<StickyNotesManager>) -> Vec<StickyNote> {
//        state.get_visible_notes()
//    }
//    ```
//
// 4. React Component (if using Tauri windows):
//    ```tsx
//    const StickyNoteWindow = ({ noteId }: { noteId: number }) => {
//      const [note, setNote] = useState<StickyNote | null>(null);
//
//      useEffect(() => {
//        // Load note
//        // Set up auto-save on changes
//      }, [noteId]);
//
//      return (
//        <div style={{ background: note?.color, padding: 10 }}>
//          <div className="title-bar">
//            {note?.title || "Untitled"}
//            <button onClick={() => invoke("sticky_notes_delete", { noteId })}>×</button>
//          </div>
//          <textarea
//            value={note?.content}
//            onChange={(e) => {
//              setNote({ ...note!, content: e.target.value });
//              invoke("sticky_notes_update", { note: { ...note, content: e.target.value } });
//            }}
//          />
//        </div>
//      );
//    };
//    ```
//
// 5. Enhancement ideas:
//    - [ ] Color picker for background
//    - [ ] Font size adjustment
//    - [ ] Bold/italic/underline formatting
//    - [ ] Markdown support
//    - [ ] Reminders (show notification at specific time)
//    - [ ] Note templates (checklists, meeting notes, etc.)
//    - [ ] Sync between devices (cloud storage)
//
// 6. Testing checklist:
//    - [ ] Notes persist across app restarts
//    - [ ] Position and size saved correctly
//    - [ ] Always on top works
//    - [ ] Multiple notes can be open simultaneously
//    - [ ] Editing is smooth (no lag)
//    - [ ] Delete removes note immediately
//
// ============================================================================
