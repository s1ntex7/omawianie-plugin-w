// ============================================================================
// SCREENSHOT TOOL - PLAN B (egui Native Rendering)
// ============================================================================
// Priority: P0 🔥 - Core feature
// Tech: egui + eframe + screenshots crate + overlay_manager
// Hotkey: F8
// Features: Multi-monitor capture, region selection, annotations, auto-save
// ============================================================================

use crate::overlay_manager::{OverlayManager, OverlayResult, ScreenRegion};
use crate::clipboard_manager::ClipboardManager;
use crate::storage_engine::StorageEngine;
use screenshots::Screen;
use image::{ImageBuffer, RgbaImage, Rgba};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::Local;

/// Screenshot tool state
pub struct ScreenshotTool {
    overlay_manager: OverlayManager,
    clipboard_manager: ClipboardManager,
    storage: Arc<Mutex<StorageEngine>>,
    last_screenshot_path: Arc<Mutex<Option<PathBuf>>>,
    save_folder: PathBuf,
}

impl ScreenshotTool {
    /// Create new screenshot tool
    pub fn new(
        storage: Arc<Mutex<StorageEngine>>,
        clipboard_manager: ClipboardManager,
        save_folder: PathBuf,
    ) -> Self {
        Self {
            overlay_manager: OverlayManager::new(),
            clipboard_manager,
            storage,
            last_screenshot_path: Arc::new(Mutex::new(None)),
            save_folder,
        }
    }

    /// Main entry point - triggered by F8 hotkey
    ///
    /// Steps:
    /// 1. Spawn overlay for region selection
    /// 2. Capture selected region
    /// 3. Show annotation toolbar (optional)
    /// 4. Save to folder + clipboard
    /// 5. Update /ss text expansion
    pub fn capture(&self) -> Result<String, String> {
        // Step 1: Show overlay and get selected region
        let result = self.overlay_manager.spawn_region_selector()?;

        match result {
            OverlayResult::RegionSelected(region) => {
                // Step 2: Capture the selected region
                let img = self.capture_region(&region)?;

                // Step 3: TODO - Show annotation toolbar (Phase 3)
                // let annotated = self.show_annotation_toolbar(img)?;

                // Step 4: Save to folder
                let file_path = self.save_screenshot(&img)?;

                // Step 5: Copy to clipboard
                self.copy_to_clipboard(&img)?;

                // Step 6: Update /ss text expansion
                self.update_ss_expansion(&file_path)?;

                Ok(file_path.to_string_lossy().to_string())
            }
            OverlayResult::Cancelled => Err("User cancelled screenshot".to_string()),
            _ => Err("Unexpected overlay result".to_string()),
        }
    }

    /// Capture specific region from screen
    ///
    /// IMPORTANT: Handles multi-monitor + DPI scaling correctly
    fn capture_region(&self, region: &ScreenRegion) -> Result<RgbaImage, String> {
        // TODO: Implement capture logic
        //
        // REUSE EXISTING CODE FROM screenshot_new.rs!
        // The multi-monitor capture logic is proven and works perfectly.
        //
        // Steps:
        // 1. Find which monitor(s) the region spans
        // 2. Capture those monitors using screenshots crate
        // 3. Crop to exact region (handle coordinate conversion)
        // 4. Return RGBA image
        //
        // Critical formulas (from SEGMENT-APLIKACJA-3.0.md line 217):
        // - Virtual desktop coordinates can be NEGATIVE (monitor left of primary)
        // - DPI scaling: physical_x = (virtual_x - monitor_x) * scale_factor
        // - Selection: clientX * scaleFactor + windowOffset
        //
        // Example from existing code:
        // ```rust
        // let screens = Screen::all()?;
        // for screen in screens {
        //     let monitor = screen.display_info;
        //     if region overlaps monitor {
        //         let image = screen.capture()?;
        //         // Crop to region...
        //     }
        // }
        // ```

        Err("Not implemented - reuse screenshot_new.rs capture logic".to_string())
    }

    /// Save screenshot to folder
    ///
    /// Filename format: DD-MM-YYYY-HH-MM-SS.png
    fn save_screenshot(&self, img: &RgbaImage) -> Result<PathBuf, String> {
        // Generate filename
        let timestamp = Local::now().format("%d-%m-%Y-%H-%M-%S");
        let filename = format!("{}.png", timestamp);
        let file_path = self.save_folder.join(&filename);

        // Ensure folder exists
        std::fs::create_dir_all(&self.save_folder)
            .map_err(|e| format!("Failed to create screenshot folder: {}", e))?;

        // Save PNG
        img.save(&file_path)
            .map_err(|e| format!("Failed to save screenshot: {}", e))?;

        // Update last screenshot path (for /ss expansion)
        *self.last_screenshot_path.lock().unwrap() = Some(file_path.clone());

        Ok(file_path)
    }

    /// Copy image to clipboard
    fn copy_to_clipboard(&self, img: &RgbaImage) -> Result<(), String> {
        let (width, height) = img.dimensions();
        let rgba_data = img.to_vec();

        self.clipboard_manager.paste_image(width as usize, height as usize, rgba_data)
    }

    /// Update /ss text expansion with latest screenshot path
    fn update_ss_expansion(&self, file_path: &PathBuf) -> Result<(), String> {
        // TODO: Integrate with simple_expansion.rs
        //
        // Steps:
        // 1. Get path as string
        // 2. Call text expansion API to update /ss shortcut
        // 3. This should be hardcoded - user doesn't configure it
        //
        // Example:
        // ```rust
        // let path_str = file_path.to_string_lossy();
        // expansion_manager.update_shortcut("/ss", &path_str)?;
        // ```
        //
        // IMPORTANT: This makes /ss always paste the latest screenshot path!
        // Super useful workflow: F8 → take screenshot → /ss → paste path

        Ok(())
    }

    /// Get last screenshot path (for UI display)
    pub fn get_last_screenshot_path(&self) -> Option<PathBuf> {
        self.last_screenshot_path.lock().unwrap().clone()
    }

    /// Change save folder (user setting)
    pub fn set_save_folder(&mut self, folder: PathBuf) -> Result<(), String> {
        // Validate folder is writable
        if !folder.exists() {
            std::fs::create_dir_all(&folder)
                .map_err(|e| format!("Failed to create folder: {}", e))?;
        }

        self.save_folder = folder.clone();

        // Save to settings
        self.storage
            .lock()
            .unwrap()
            .set_setting("screenshot_folder", &folder.to_string_lossy())?;

        Ok(())
    }
}

// ============================================================================
// ANNOTATION SYSTEM (Phase 3)
// ============================================================================

/// Annotation toolbar (shown after region selection)
///
/// Tools:
/// - Arrow (auto-numbered: 1., 2., 3...)
/// - Text box (click + drag to define, auto-wrap)
/// - Blur (pixelate sensitive areas)
/// - Undo/Redo
///
/// Hotkeys:
/// - Ctrl+C: Accept and save
/// - ESC: Cancel
pub struct AnnotationToolbar {
    image: RgbaImage,
    annotations: Vec<Annotation>,
    active_tool: AnnotationTool,
}

#[derive(Debug, Clone)]
pub enum Annotation {
    Arrow {
        number: u32,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        color: Rgba<u8>,
    },
    Text {
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        text: String,
        font_size: u32,
        color: Rgba<u8>,
    },
    Blur {
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        intensity: u32,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum AnnotationTool {
    Arrow,
    Text,
    Blur,
    Select, // For moving/editing existing annotations
}

impl AnnotationToolbar {
    pub fn new(image: RgbaImage) -> Self {
        Self {
            image,
            annotations: Vec::new(),
            active_tool: AnnotationTool::Arrow,
        }
    }

    /// Show toolbar UI and wait for user input
    ///
    /// Returns: Final annotated image, or None if cancelled
    pub fn show(&mut self) -> Result<Option<RgbaImage>, String> {
        // TODO: Implement egui toolbar UI (Phase 3)
        //
        // Layout:
        // - Top toolbar with tool buttons
        // - Main canvas showing image + annotations
        // - Hotkeys: Ctrl+C (save), ESC (cancel), Ctrl+Z (undo)
        //
        // See SCREENSHOT-MASTER-PLAN.md Phase 3 for detailed design

        Err("Not implemented - Phase 3 feature".to_string())
    }

    /// Add arrow annotation
    pub fn add_arrow(&mut self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) {
        let number = self.annotations.iter().filter(|a| matches!(a, Annotation::Arrow { .. })).count() as u32 + 1;

        self.annotations.push(Annotation::Arrow {
            number,
            start_x,
            start_y,
            end_x,
            end_y,
            color: Rgba([255, 0, 0, 255]), // Red
        });
    }

    /// Render all annotations onto image
    pub fn render(&self) -> RgbaImage {
        let mut img = self.image.clone();

        for annotation in &self.annotations {
            match annotation {
                Annotation::Arrow { number, start_x, start_y, end_x, end_y, color } => {
                    // TODO: Draw arrow with number
                    // Use imageproc crate for drawing primitives
                }
                Annotation::Text { x, y, text, font_size, color, .. } => {
                    // TODO: Draw text with wrapping
                    // Use rusttype or ab_glyph for font rendering
                }
                Annotation::Blur { x, y, width, height, intensity } => {
                    // TODO: Apply pixelation effect
                    // Downsample region then upsample back
                }
            }
        }

        img
    }
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This file should be placed in: src-tauri/src/screenshot_tool_egui.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod screenshot_tool_egui;
//    use screenshot_tool_egui::ScreenshotTool;
//
//    // In .setup():
//    let screenshot_folder = app.path().app_data_dir()?.join("Screenshots");
//    let screenshot_tool = ScreenshotTool::new(
//        storage.clone(),
//        clipboard_mgr.clone(),
//        screenshot_folder
//    );
//    app.manage(screenshot_tool);
//
//    // Register F8 hotkey:
//    gs.on_shortcut("F8", {
//        let app = app.handle().clone();
//        move |_app, _shortcut, event| {
//            if format!("{:?}", event).contains("Pressed") {
//                let _ = app.emit_to("main", "screenshot:capture", ());
//            }
//        }
//    })?;
//    ```
//
// 3. IPC Command:
//    ```rust
//    #[tauri::command]
//    fn screenshot_capture(state: tauri::State<ScreenshotTool>) -> Result<String, String> {
//        state.capture()
//    }
//
//    #[tauri::command]
//    fn screenshot_set_folder(folder: String, state: tauri::State<Mutex<ScreenshotTool>>) -> Result<(), String> {
//        state.lock().unwrap().set_save_folder(folder.into())
//    }
//    ```
//
// 4. Dependencies:
//    ```toml
//    screenshots = "0.10"  # Already in project
//    image = "0.25"        # Already in project
//    chrono = "0.4"
//    imageproc = "0.25"    # For annotation drawing (Phase 3)
//    rusttype = "0.9"      # For text rendering (Phase 3)
//    ```
//
// 5. CRITICAL: Reuse screenshot_new.rs logic!
//    - Multi-monitor capture code is proven (works perfectly)
//    - Coordinate math is solved
//    - DPI scaling handled correctly
//    - Don't rewrite from scratch - copy and adapt!
//
// 6. Implementation phases:
//    - Phase 0: POC with overlay_manager (verify no transform bug)
//    - Phase 1: Region selection + capture
//    - Phase 2: Save + clipboard + /ss expansion
//    - Phase 3: Annotation toolbar (arrows, text, blur)
//    - Phase 4: Polish, settings UI, testing
//
// 7. Testing checklist:
//    - [ ] F8 spawns overlay instantly (< 200ms)
//    - [ ] Region selection pixel-perfect on all monitors
//    - [ ] DPI scaling works (150%, 200%)
//    - [ ] Negative monitor coordinates handled (monitor left of primary)
//    - [ ] Screenshot saved with correct filename format
//    - [ ] Image copied to clipboard
//    - [ ] /ss expansion updated
//    - [ ] Existing features (Text Expansion, VTT) still work
//
// ============================================================================
