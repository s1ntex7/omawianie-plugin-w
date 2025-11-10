// ============================================================================
// GLOBAL COLOR PICKER
// ============================================================================
// Priority: P1 (Quick Win - High Value, Low Complexity)
// Tech: overlay_manager + screenshots crate for pixel grabbing
// Features: Click anywhere on screen to get color (HEX, RGB, HSL)
// Estimated time: 6 hours
// ============================================================================

use crate::overlay_manager::{OverlayManager, OverlayResult};
use crate::clipboard_manager::ClipboardManager;
use screenshots::Screen;
use image::Rgba;

/// Color formats
#[derive(Debug, Clone)]
pub enum ColorFormat {
    Hex,      // #FF5733
    Rgb,      // rgb(255, 87, 51)
    Rgba,     // rgba(255, 87, 51, 1.0)
    Hsl,      // hsl(12, 100%, 60%)
    Hsla,     // hsla(12, 100%, 60%, 1.0)
}

/// Color picker result
#[derive(Debug, Clone)]
pub struct ColorResult {
    pub x: i32,
    pub y: i32,
    pub color: Rgba<u8>,
    pub hex: String,
    pub rgb: String,
    pub rgba: String,
    pub hsl: String,
    pub hsla: String,
}

/// Global color picker
pub struct ColorPicker {
    overlay_manager: OverlayManager,
    clipboard_manager: ClipboardManager,
    default_format: ColorFormat,
}

impl ColorPicker {
    pub fn new(clipboard_manager: ClipboardManager) -> Self {
        Self {
            overlay_manager: OverlayManager::new(),
            clipboard_manager,
            default_format: ColorFormat::Hex,
        }
    }

    /// Pick color from screen
    ///
    /// Shows fullscreen overlay, user clicks a point, returns color
    pub fn pick(&self) -> Result<ColorResult, String> {
        // Step 1: Show overlay for point selection
        let result = self.overlay_manager.spawn_point_selector()?;

        match result {
            OverlayResult::PointClicked { x, y } => {
                // Step 2: Get pixel color at clicked position
                let color = self.get_pixel_color_at(x, y)?;

                // Step 3: Convert to all formats
                let color_result = self.color_to_all_formats(x, y, color);

                // Step 4: Copy default format to clipboard
                let default_str = match self.default_format {
                    ColorFormat::Hex => &color_result.hex,
                    ColorFormat::Rgb => &color_result.rgb,
                    ColorFormat::Rgba => &color_result.rgba,
                    ColorFormat::Hsl => &color_result.hsl,
                    ColorFormat::Hsla => &color_result.hsla,
                };
                self.clipboard_manager.paste_text(default_str)?;

                Ok(color_result)
            }
            OverlayResult::Cancelled => Err("User cancelled color picking".to_string()),
            _ => Err("Unexpected overlay result".to_string()),
        }
    }

    /// Get pixel color at specific coordinates
    fn get_pixel_color_at(&self, x: i32, y: i32) -> Result<Rgba<u8>, String> {
        // TODO: Implement pixel color grabbing
        //
        // Steps:
        // 1. Find which monitor contains point (x, y)
        // 2. Capture that monitor using screenshots crate
        // 3. Convert coordinates to monitor-local (x - monitor.x, y - monitor.y)
        // 4. Handle DPI scaling (physical vs logical pixels)
        // 5. Get pixel from image at (local_x, local_y)
        // 6. Return RGBA color
        //
        // Example:
        // ```rust
        // let screens = Screen::all()?;
        // for screen in screens {
        //     let monitor = screen.display_info;
        //     if x >= monitor.x && x < monitor.x + monitor.width as i32 &&
        //        y >= monitor.y && y < monitor.y + monitor.height as i32 {
        //         let image = screen.capture()?;
        //         let local_x = (x - monitor.x) * monitor.scale_factor;
        //         let local_y = (y - monitor.y) * monitor.scale_factor;
        //         let pixel = image.get_pixel(local_x as u32, local_y as u32);
        //         return Ok(*pixel);
        //     }
        // }
        // ```
        //
        // IMPORTANT: Reuse coordinate math from screenshot_new.rs!

        Err("Not implemented - use screenshots crate".to_string())
    }

    /// Convert color to all formats
    fn color_to_all_formats(&self, x: i32, y: i32, color: Rgba<u8>) -> ColorResult {
        let r = color[0];
        let g = color[1];
        let b = color[2];
        let a = color[3] as f32 / 255.0;

        // HEX
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

        // RGB
        let rgb = format!("rgb({}, {}, {})", r, g, b);
        let rgba = format!("rgba({}, {}, {}, {:.2})", r, g, b, a);

        // HSL conversion
        let (h, s, l) = self.rgb_to_hsl(r, g, b);
        let hsl = format!("hsl({}, {}%, {}%)", h, (s * 100.0) as u32, (l * 100.0) as u32);
        let hsla = format!("hsla({}, {}%, {}%, {:.2})", h, (s * 100.0) as u32, (l * 100.0) as u32, a);

        ColorResult {
            x,
            y,
            color,
            hex,
            rgb,
            rgba,
            hsl,
            hsla,
        }
    }

    /// Convert RGB to HSL
    fn rgb_to_hsl(&self, r: u8, g: u8, b: u8) -> (u32, f32, f32) {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        // Lightness
        let l = (max + min) / 2.0;

        // Saturation
        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        // Hue
        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        (h as u32, s, l)
    }

    /// Set default format for clipboard copy
    pub fn set_default_format(&mut self, format: ColorFormat) {
        self.default_format = format;
    }
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This file should be placed in: src-tauri/src/color_picker.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod color_picker;
//    use color_picker::ColorPicker;
//
//    let color_picker = ColorPicker::new(clipboard_mgr.clone());
//    app.manage(color_picker);
//
//    // Optional: Register global hotkey (e.g., Ctrl+Shift+C)
//    gs.on_shortcut("Ctrl+Shift+C", {
//        let app = app.handle().clone();
//        move |_app, _shortcut, event| {
//            if format!("{:?}", event).contains("Pressed") {
//                let _ = app.emit_to("main", "colorpicker:activate", ());
//            }
//        }
//    })?;
//    ```
//
// 3. Dependencies (already in project):
//    ```toml
//    screenshots = "0.10"
//    image = "0.25"
//    ```
//
// 4. IPC Commands:
//    ```rust
//    #[tauri::command]
//    fn colorpicker_pick(state: tauri::State<ColorPicker>) -> Result<ColorResult, String> {
//        state.pick()
//    }
//
//    #[tauri::command]
//    fn colorpicker_set_format(format: String, state: tauri::State<Mutex<ColorPicker>>) -> Result<(), String> {
//        let fmt = match format.as_str() {
//            "hex" => ColorFormat::Hex,
//            "rgb" => ColorFormat::Rgb,
//            "rgba" => ColorFormat::Rgba,
//            "hsl" => ColorFormat::Hsl,
//            "hsla" => ColorFormat::Hsla,
//            _ => return Err("Invalid format".to_string()),
//        };
//        state.lock().unwrap().set_default_format(fmt);
//        Ok(())
//    }
//    ```
//
// 5. React UI component (example):
//    ```tsx
//    const ColorPickerPanel = () => {
//      const [color, setColor] = useState<ColorResult | null>(null);
//      const [format, setFormat] = useState<string>("hex");
//
//      const pick = async () => {
//        try {
//          const result = await invoke<ColorResult>("colorpicker_pick");
//          setColor(result);
//          toast.success(`Copied ${result[format]} to clipboard!`);
//        } catch (err) {
//          toast.error("Color picking cancelled");
//        }
//      };
//
//      return (
//        <div>
//          <button onClick={pick}>Pick Color</button>
//          <select value={format} onChange={(e) => {
//            setFormat(e.target.value);
//            invoke("colorpicker_set_format", { format: e.target.value });
//          }}>
//            <option value="hex">HEX</option>
//            <option value="rgb">RGB</option>
//            <option value="hsl">HSL</option>
//          </select>
//          {color && (
//            <div>
//              <div style={{ background: color.hex, width: 100, height: 100 }} />
//              <p>HEX: {color.hex}</p>
//              <p>RGB: {color.rgb}</p>
//              <p>HSL: {color.hsl}</p>
//            </div>
//          )}
//        </div>
//      );
//    };
//    ```
//
// 6. Enhancement ideas:
//    - [ ] Show magnified preview near cursor during picking
//    - [ ] Color history (last 10 picked colors)
//    - [ ] Color palette export (save multiple colors)
//    - [ ] Color contrast checker (WCAG compliance)
//    - [ ] Eyedropper cursor icon during picking
//
// 7. Testing checklist:
//    - [ ] Works on all monitors
//    - [ ] DPI scaling handled correctly
//    - [ ] Accurate color values (compare with browser DevTools)
//    - [ ] ESC cancels cleanly
//    - [ ] All formats copied correctly
//
// ============================================================================
