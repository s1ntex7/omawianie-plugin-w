// ============================================================================
// OVERLAY MANAGER - Shared Module for Aplikacja 3.0/4.0
// ============================================================================
// Used by: Screenshot Tool, Color Picker, Screen Recorder, OCR
// Purpose: Unified API for creating native transparent overlays with egui
// Tech: egui v0.29.1, eframe v0.29.1
// ============================================================================

use eframe::egui;
use std::sync::{Arc, Mutex};

/// Configuration for overlay behavior
#[derive(Debug, Clone)]
pub struct OverlayConfig {
    /// Background color (RGBA, 0.0-1.0)
    pub background_color: [f32; 4],

    /// Should overlay span all monitors?
    pub fullscreen_all_monitors: bool,

    /// Is overlay transparent?
    pub transparent: bool,

    /// Always on top?
    pub always_on_top: bool,

    /// Show window decorations?
    pub decorations: bool,
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self {
            background_color: [0.0, 0.0, 0.0, 0.8], // Dark semi-transparent
            fullscreen_all_monitors: true,
            transparent: true,
            always_on_top: true,
            decorations: false,
        }
    }
}

/// Selected region on screen (pixel coordinates)
#[derive(Debug, Clone, Copy, Default)]
pub struct ScreenRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Result of overlay interaction
#[derive(Debug, Clone)]
pub enum OverlayResult {
    /// User selected a region
    RegionSelected(ScreenRegion),

    /// User clicked a point (for color picker)
    PointClicked { x: i32, y: i32 },

    /// User cancelled (ESC pressed)
    Cancelled,

    /// Overlay closed without action
    Closed,
}

/// Main overlay manager
pub struct OverlayManager {
    config: OverlayConfig,
    result: Arc<Mutex<Option<OverlayResult>>>,
}

impl OverlayManager {
    /// Create new overlay manager with default config
    pub fn new() -> Self {
        Self::with_config(OverlayConfig::default())
    }

    /// Create with custom config
    pub fn with_config(config: OverlayConfig) -> Self {
        Self {
            config,
            result: Arc::new(Mutex::new(None)),
        }
    }

    /// Spawn fullscreen overlay for region selection
    ///
    /// This is the main entry point for Screenshot Tool.
    ///
    /// # Example
    /// ```
    /// let manager = OverlayManager::new();
    /// let result = manager.spawn_region_selector()?;
    /// match result {
    ///     OverlayResult::RegionSelected(region) => {
    ///         println!("Selected: {}x{} at ({}, {})",
    ///                  region.width, region.height, region.x, region.y);
    ///     }
    ///     OverlayResult::Cancelled => println!("User cancelled"),
    ///     _ => {}
    /// }
    /// ```
    pub fn spawn_region_selector(&self) -> Result<OverlayResult, String> {
        // TODO: Implement egui overlay
        //
        // Steps:
        // 1. Capture all screens using screenshots crate (reuse screenshot_new.rs logic)
        // 2. Create eframe::NativeOptions with transparent window
        // 3. Run egui app with:
        //    - Display captured images as background
        //    - Track mouse click + drag for selection
        //    - Draw selection rectangle
        //    - ESC to cancel
        //    - Release mouse to confirm
        // 4. Return OverlayResult with selected region
        //
        // CRITICAL: Handle multi-monitor coordinate conversion
        // - Virtual desktop coordinates (can be negative!)
        // - DPI scaling (physical vs logical pixels)
        // - Formula: physical_x = (virtual_x - monitor_x) * scale_factor
        //
        // See: SCREENSHOT-MASTER-PLAN.md Phase 1 for detailed implementation

        Err("Not implemented - integrate with screenshot_new.rs capture logic".to_string())
    }

    /// Spawn overlay for single point selection (Color Picker)
    ///
    /// # Example
    /// ```
    /// let manager = OverlayManager::new();
    /// if let OverlayResult::PointClicked { x, y } = manager.spawn_point_selector()? {
    ///     let color = get_pixel_color_at(x, y);
    ///     println!("Color at ({}, {}): {:?}", x, y, color);
    /// }
    /// ```
    pub fn spawn_point_selector(&self) -> Result<OverlayResult, String> {
        // TODO: Similar to spawn_region_selector but simpler
        // - Single click instead of drag
        // - Optionally show magnified preview near cursor
        // - Return clicked coordinates

        Err("Not implemented - see spawn_region_selector for reference".to_string())
    }

    /// Get current overlay result (non-blocking)
    pub fn poll_result(&self) -> Option<OverlayResult> {
        self.result.lock().unwrap().clone()
    }

    /// Set overlay result (called internally by egui app)
    pub fn set_result(&self, result: OverlayResult) {
        *self.result.lock().unwrap() = Some(result);
    }
}

impl Default for OverlayManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This module should be placed in: src-tauri/src/overlay_manager.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod overlay_manager;
//    use overlay_manager::OverlayManager;
//    ```
//
// 3. Dependencies to add to Cargo.toml:
//    ```toml
//    egui = "0.29.1"
//    eframe = "0.29.1"
//    ```
//
// 4. Reuse existing capture logic from screenshot_new.rs:
//    - The multi-monitor capture code is proven and works perfectly
//    - Coordinate math is already solved (see SEGMENT-APLIKACJA-3.0.md line 217)
//    - DPI scaling is handled correctly
//
// 5. Testing checklist:
//    - [ ] Overlay appears on all monitors
//    - [ ] No window transform bug (verify for 5+ seconds)
//    - [ ] ESC closes cleanly
//    - [ ] Selection coordinates are pixel-perfect
//    - [ ] Works with negative monitor coordinates (monitor left of primary)
//    - [ ] DPI scaling handled correctly (150%, 200% scaling)
//
// 6. Reference files:
//    - SCREENSHOT-MASTER-PLAN.md - Phase 0-1 for egui implementation
//    - STATUS-2025-11-06-PLAN-B-READY.md - Architecture decision details
//    - screenshot_new.rs - Existing capture logic to reuse
//
// ============================================================================
