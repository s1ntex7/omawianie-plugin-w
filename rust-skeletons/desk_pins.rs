// ============================================================================
// DESK PINS - Always on Top Window Manager
// ============================================================================
// Priority: P1 (Quick Win)
// Tech: Windows API (SetWindowPos), macOS/Linux equivalents
// Features: Pin any window to stay always on top
// Estimated time: 5 hours
// ============================================================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextW, SetWindowPos, HWND_TOPMOST, HWND_NOTOPMOST,
    SWP_NOMOVE, SWP_NOSIZE, IsWindowVisible, GetWindowLongW, GWL_EXSTYLE, WS_EX_TOPMOST,
};

/// Window information
#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub hwnd: isize, // HWND as integer
    pub title: String,
    pub is_pinned: bool,
}

/// Desk Pins manager
pub struct DeskPins {
    pinned_windows: Arc<Mutex<HashMap<isize, WindowInfo>>>,
}

impl DeskPins {
    pub fn new() -> Self {
        Self {
            pinned_windows: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get list of all windows
    pub fn list_windows(&self) -> Result<Vec<WindowInfo>, String> {
        #[cfg(target_os = "windows")]
        {
            self.list_windows_windows()
        }

        #[cfg(target_os = "macos")]
        {
            self.list_windows_macos()
        }

        #[cfg(target_os = "linux")]
        {
            self.list_windows_linux()
        }
    }

    /// Pin window to always on top
    pub fn pin_window(&self, hwnd: isize) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            unsafe {
                let hwnd = HWND(hwnd as *mut _);
                SetWindowPos(
                    hwnd,
                    HWND_TOPMOST,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE,
                )
                .map_err(|e| format!("Failed to pin window: {}", e))?;
            }

            // Track pinned window
            let mut pinned = self.pinned_windows.lock().unwrap();
            if let Some(window) = pinned.get_mut(&hwnd) {
                window.is_pinned = true;
            }

            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err("Not implemented for this platform".to_string())
        }
    }

    /// Unpin window (remove always on top)
    pub fn unpin_window(&self, hwnd: isize) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            unsafe {
                let hwnd = HWND(hwnd as *mut _);
                SetWindowPos(
                    hwnd,
                    HWND_NOTOPMOST,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE,
                )
                .map_err(|e| format!("Failed to unpin window: {}", e))?;
            }

            // Update tracking
            let mut pinned = self.pinned_windows.lock().unwrap();
            if let Some(window) = pinned.get_mut(&hwnd) {
                window.is_pinned = false;
            }

            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err("Not implemented for this platform".to_string())
        }
    }

    /// Toggle pin status
    pub fn toggle_pin(&self, hwnd: isize) -> Result<bool, String> {
        let is_pinned = self.is_window_pinned(hwnd)?;

        if is_pinned {
            self.unpin_window(hwnd)?;
            Ok(false)
        } else {
            self.pin_window(hwnd)?;
            Ok(true)
        }
    }

    /// Check if window is pinned
    pub fn is_window_pinned(&self, hwnd: isize) -> Result<bool, String> {
        #[cfg(target_os = "windows")]
        {
            unsafe {
                let hwnd = HWND(hwnd as *mut _);
                let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                Ok((ex_style & WS_EX_TOPMOST.0 as i32) != 0)
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err("Not implemented for this platform".to_string())
        }
    }

    /// Get list of pinned windows
    pub fn get_pinned_windows(&self) -> Vec<WindowInfo> {
        self.pinned_windows
            .lock()
            .unwrap()
            .values()
            .filter(|w| w.is_pinned)
            .cloned()
            .collect()
    }

    // ============================================================================
    // PLATFORM-SPECIFIC IMPLEMENTATIONS
    // ============================================================================

    #[cfg(target_os = "windows")]
    fn list_windows_windows(&self) -> Result<Vec<WindowInfo>, String> {
        let windows: Arc<Mutex<Vec<WindowInfo>>> = Arc::new(Mutex::new(Vec::new()));
        let windows_clone = windows.clone();

        unsafe {
            let result = EnumWindows(
                Some(Self::enum_windows_callback),
                LPARAM(&*windows_clone as *const _ as isize),
            );

            if !result.as_bool() {
                return Err("Failed to enumerate windows".to_string());
            }
        }

        Ok(windows.lock().unwrap().clone())
    }

    #[cfg(target_os = "windows")]
    unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> windows::Win32::Foundation::BOOL {
        // Skip invisible windows
        if !IsWindowVisible(hwnd).as_bool() {
            return windows::Win32::Foundation::BOOL(1);
        }

        // Get window title
        let mut title_buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut title_buf);
        if len == 0 {
            return windows::Win32::Foundation::BOOL(1);
        }

        let title = String::from_utf16_lossy(&title_buf[..len as usize]);

        // Skip empty titles
        if title.trim().is_empty() {
            return windows::Win32::Foundation::BOOL(1);
        }

        // Check if pinned
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        let is_pinned = (ex_style & WS_EX_TOPMOST.0 as i32) != 0;

        // Add to list
        let windows = &mut *(lparam.0 as *mut Mutex<Vec<WindowInfo>>);
        windows.lock().unwrap().push(WindowInfo {
            hwnd: hwnd.0 as isize,
            title,
            is_pinned,
        });

        windows::Win32::Foundation::BOOL(1) // Continue enumeration
    }

    #[cfg(target_os = "macos")]
    fn list_windows_macos(&self) -> Result<Vec<WindowInfo>, String> {
        // TODO: Implement using macOS Core Graphics API
        //
        // Steps:
        // 1. Use CGWindowListCopyWindowInfo to get all windows
        // 2. Filter visible windows with titles
        // 3. Check kCGWindowLayer for always-on-top status
        //
        // Dependencies:
        // - core-graphics = "0.23"
        // - cocoa = "0.25"

        Err("macOS support not implemented yet".to_string())
    }

    #[cfg(target_os = "linux")]
    fn list_windows_linux(&self) -> Result<Vec<WindowInfo>, String> {
        // TODO: Implement using X11/Wayland
        //
        // X11 approach:
        // 1. Use x11rb crate
        // 2. Query _NET_CLIENT_LIST for window IDs
        // 3. Get _NET_WM_NAME for titles
        // 4. Check _NET_WM_STATE_ABOVE for always-on-top
        //
        // Wayland approach:
        // - More complex, compositor-dependent
        // - May require wlr-foreign-toplevel-management protocol
        //
        // Dependencies:
        // - x11rb = "0.13" (for X11)

        Err("Linux support not implemented yet".to_string())
    }
}

impl Default for DeskPins {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This file should be placed in: src-tauri/src/desk_pins.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod desk_pins;
//    use desk_pins::DeskPins;
//
//    let desk_pins = DeskPins::new();
//    app.manage(desk_pins);
//    ```
//
// 3. Dependencies to add to Cargo.toml:
//    ```toml
//    [target.'cfg(windows)'.dependencies]
//    windows = { version = "0.58", features = [
//        "Win32_Foundation",
//        "Win32_UI_WindowsAndMessaging",
//    ]}
//
//    [target.'cfg(target_os = "macos")'.dependencies]
//    core-graphics = "0.23"
//    cocoa = "0.25"
//
//    [target.'cfg(target_os = "linux")'.dependencies]
//    x11rb = "0.13"
//    ```
//
// 4. IPC Commands:
//    ```rust
//    #[tauri::command]
//    fn deskpins_list_windows(state: tauri::State<DeskPins>) -> Result<Vec<WindowInfo>, String> {
//        state.list_windows()
//    }
//
//    #[tauri::command]
//    fn deskpins_toggle_pin(hwnd: isize, state: tauri::State<DeskPins>) -> Result<bool, String> {
//        state.toggle_pin(hwnd)
//    }
//
//    #[tauri::command]
//    fn deskpins_get_pinned(state: tauri::State<DeskPins>) -> Vec<WindowInfo> {
//        state.get_pinned_windows()
//    }
//    ```
//
// 5. React UI component (example):
//    ```tsx
//    const DeskPinsPanel = () => {
//      const [windows, setWindows] = useState<WindowInfo[]>([]);
//
//      const loadWindows = async () => {
//        const result = await invoke<WindowInfo[]>("deskpins_list_windows");
//        setWindows(result);
//      };
//
//      const togglePin = async (hwnd: number) => {
//        const isPinned = await invoke<boolean>("deskpins_toggle_pin", { hwnd });
//        toast.success(isPinned ? "Window pinned" : "Window unpinned");
//        loadWindows();
//      };
//
//      return (
//        <div>
//          <button onClick={loadWindows}>Refresh Windows</button>
//          {windows.map((win) => (
//            <div key={win.hwnd}>
//              <span>{win.title}</span>
//              <button onClick={() => togglePin(win.hwnd)}>
//                {win.is_pinned ? "Unpin" : "Pin"}
//              </button>
//            </div>
//          ))}
//        </div>
//      );
//    };
//    ```
//
// 6. Enhancement ideas:
//    - [ ] Global hotkey to pin focused window (e.g., Ctrl+Alt+P)
//    - [ ] System tray icon showing pinned windows count
//    - [ ] Persist pinned windows across app restarts
//    - [ ] Pin profiles (save/restore sets of pinned windows)
//    - [ ] Auto-pin specific applications (e.g., calculator, sticky notes)
//
// 7. Testing checklist:
//    - [ ] List windows shows all visible windows
//    - [ ] Pin/unpin works correctly
//    - [ ] Pinned windows stay on top even when other apps are focused
//    - [ ] Works with multiple monitors
//    - [ ] No crashes when window is closed while pinned
//
// ============================================================================
