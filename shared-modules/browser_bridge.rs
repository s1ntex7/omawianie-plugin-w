// ============================================================================
// BROWSER BRIDGE - Shared Module for Aplikacja 3.0/4.0
// ============================================================================
// Used by: Browser Extension (GoFullPage v2), Web Clipper
// Purpose: Native messaging bridge between browser extensions and Tauri app
// Tech: Chrome Native Messaging Protocol, Firefox WebExtensions
// ============================================================================

use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

/// Message from browser extension to app
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExtensionMessage {
    /// Screenshot full page
    ScreenshotFullPage {
        url: String,
        data_url: String, // base64 image
        width: u32,
        height: u32,
    },

    /// Clip text from webpage
    ClipText {
        url: String,
        title: String,
        text: String,
        html: Option<String>,
    },

    /// Get app settings
    GetSettings,

    /// Ping (health check)
    Ping,
}

/// Message from app to browser extension
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AppMessage {
    /// Screenshot saved successfully
    ScreenshotSaved {
        file_path: String,
    },

    /// Text clipped successfully (created Sticky Note)
    TextClipped {
        note_id: u64,
    },

    /// App settings
    Settings {
        screenshot_folder: String,
        auto_copy_path: bool,
    },

    /// Pong response
    Pong,

    /// Error occurred
    Error {
        message: String,
    },
}

/// Browser bridge for native messaging
pub struct BrowserBridge {
    message_sender: Sender<AppMessage>,
    message_receiver: Receiver<ExtensionMessage>,
}

impl BrowserBridge {
    /// Initialize native messaging bridge
    ///
    /// This starts a background thread that listens for messages from stdin
    /// (Chrome/Firefox native messaging protocol).
    ///
    /// # Example
    /// ```
    /// let bridge = BrowserBridge::init()?;
    ///
    /// // In main loop:
    /// while let Ok(msg) = bridge.recv() {
    ///     match msg {
    ///         ExtensionMessage::ScreenshotFullPage { data_url, .. } => {
    ///             save_screenshot(data_url)?;
    ///             bridge.send(AppMessage::ScreenshotSaved { file_path: "...".to_string() })?;
    ///         }
    ///         _ => {}
    ///     }
    /// }
    /// ```
    pub fn init() -> Result<Self, String> {
        let (ext_tx, ext_rx) = channel::<ExtensionMessage>();
        let (app_tx, app_rx) = channel::<AppMessage>();

        // Spawn stdin reader thread
        let ext_tx_clone = ext_tx.clone();
        thread::spawn(move || {
            if let Err(e) = Self::read_messages(ext_tx_clone) {
                eprintln!("Native messaging reader error: {}", e);
            }
        });

        // Spawn stdout writer thread
        thread::spawn(move || {
            while let Ok(msg) = app_rx.recv() {
                if let Err(e) = Self::write_message(&msg) {
                    eprintln!("Native messaging writer error: {}", e);
                }
            }
        });

        Ok(Self {
            message_sender: app_tx,
            message_receiver: ext_rx,
        })
    }

    /// Receive message from extension (non-blocking)
    pub fn recv(&self) -> Result<ExtensionMessage, String> {
        self.message_receiver
            .recv()
            .map_err(|e| format!("Failed to receive message: {}", e))
    }

    /// Send message to extension
    pub fn send(&self, message: AppMessage) -> Result<(), String> {
        self.message_sender
            .send(message)
            .map_err(|e| format!("Failed to send message: {}", e))
    }

    /// Read messages from stdin (Chrome Native Messaging Protocol)
    ///
    /// Format:
    /// - First 4 bytes: message length (u32, little-endian)
    /// - Next N bytes: JSON message
    fn read_messages(sender: Sender<ExtensionMessage>) -> Result<(), String> {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        loop {
            // Read message length (4 bytes, little-endian)
            let mut length_bytes = [0u8; 4];
            if let Err(e) = handle.read_exact(&mut length_bytes) {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    break; // Extension closed connection
                }
                return Err(format!("Failed to read message length: {}", e));
            }

            let length = u32::from_le_bytes(length_bytes) as usize;

            // Read message JSON
            let mut message_bytes = vec![0u8; length];
            handle
                .read_exact(&mut message_bytes)
                .map_err(|e| format!("Failed to read message: {}", e))?;

            // Parse JSON
            let message: ExtensionMessage = serde_json::from_slice(&message_bytes)
                .map_err(|e| format!("Failed to parse message: {}", e))?;

            // Send to main thread
            sender
                .send(message)
                .map_err(|e| format!("Failed to forward message: {}", e))?;
        }

        Ok(())
    }

    /// Write message to stdout (Chrome Native Messaging Protocol)
    fn write_message(message: &AppMessage) -> Result<(), String> {
        let json = serde_json::to_string(message)
            .map_err(|e| format!("Failed to serialize message: {}", e))?;

        let bytes = json.as_bytes();
        let length = bytes.len() as u32;

        let stdout = io::stdout();
        let mut handle = stdout.lock();

        // Write length (4 bytes, little-endian)
        handle
            .write_all(&length.to_le_bytes())
            .map_err(|e| format!("Failed to write length: {}", e))?;

        // Write JSON
        handle
            .write_all(bytes)
            .map_err(|e| format!("Failed to write message: {}", e))?;

        handle
            .flush()
            .map_err(|e| format!("Failed to flush stdout: {}", e))?;

        Ok(())
    }

    /// Install native messaging host manifest
    ///
    /// This registers the app with Chrome/Firefox so extensions can communicate.
    ///
    /// # Example
    /// ```
    /// BrowserBridge::install_manifest(
    ///     "com.aplikacja.3-0",
    ///     "C:\\Program Files\\Aplikacja 3.0\\aplikacja_3_0.exe",
    ///     &["chrome-extension://abcdef123456/"]
    /// )?;
    /// ```
    pub fn install_manifest(
        app_id: &str,
        executable_path: &str,
        allowed_origins: &[&str],
    ) -> Result<(), String> {
        // TODO: Implement manifest installation
        //
        // Steps:
        // 1. Create JSON manifest:
        //    {
        //      "name": "com.aplikacja.3-0",
        //      "description": "Aplikacja 3.0 Native Messaging Host",
        //      "path": "C:\\...\\aplikacja_3_0.exe",
        //      "type": "stdio",
        //      "allowed_origins": ["chrome-extension://..."]
        //    }
        //
        // 2. Write to platform-specific location:
        //    - Windows: HKEY_CURRENT_USER\Software\Google\Chrome\NativeMessagingHosts\com.aplikacja.3-0
        //    - macOS: ~/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.aplikacja.3-0.json
        //    - Linux: ~/.config/google-chrome/NativeMessagingHosts/com.aplikacja.3-0.json
        //
        // 3. For Firefox, use different registry key / path
        //
        // IMPORTANT: Run this during app installation or first launch
        // - Installer should call this
        // - Or app checks on startup and installs if missing

        Err("Not implemented - see install_manifest TODO".to_string())
    }
}

// ============================================================================
// BROWSER EXTENSION SKELETON (JavaScript)
// ============================================================================
//
// Place this in browser-extension/ folder for reference:
//
// manifest.json (Chrome):
// ```json
// {
//   "manifest_version": 3,
//   "name": "Aplikacja 3.0 Extension",
//   "version": "1.0.0",
//   "permissions": ["nativeMessaging", "activeTab", "tabs"],
//   "background": {
//     "service_worker": "background.js"
//   },
//   "action": {
//     "default_popup": "popup.html"
//   }
// }
// ```
//
// background.js:
// ```javascript
// let port = chrome.runtime.connectNative("com.aplikacja.3-0");
//
// port.onMessage.addListener((msg) => {
//   console.log("Received from app:", msg);
//   if (msg.type === "ScreenshotSaved") {
//     chrome.notifications.create({
//       type: "basic",
//       title: "Screenshot Saved",
//       message: `Saved to ${msg.file_path}`
//     });
//   }
// });
//
// // Screenshot full page
// chrome.action.onClicked.addListener(async (tab) => {
//   const dataUrl = await captureFullPage(tab.id);
//   port.postMessage({
//     type: "ScreenshotFullPage",
//     url: tab.url,
//     data_url: dataUrl,
//     width: 1920,
//     height: 10800
//   });
// });
//
// async function captureFullPage(tabId) {
//   // TODO: Implement full page capture
//   // Options:
//   // 1. Scroll + stitch multiple captures
//   // 2. Use chrome.debugger API
//   // 3. Inject content script that expands page
// }
// ```
//
// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This module should be placed in: src-tauri/src/browser_bridge.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod browser_bridge;
//    use browser_bridge::BrowserBridge;
//
//    // In .setup():
//    let bridge = BrowserBridge::init()?;
//    app.manage(bridge);
//
//    // Spawn handler thread
//    thread::spawn(move || {
//        while let Ok(msg) = bridge.recv() {
//            handle_extension_message(msg, &app_handle);
//        }
//    });
//    ```
//
// 3. Dependencies (already in project):
//    ```toml
//    serde = { version = "1.0", features = ["derive"] }
//    serde_json = "1.0"
//    ```
//
// 4. Browser extension development:
//    - Create browser-extension/ folder in project root
//    - Implement manifest.json, background.js, popup.html
//    - Test with chrome://extensions (Load unpacked)
//    - Submit to Chrome Web Store / Firefox Add-ons
//
// 5. Native messaging manifest:
//    - Call install_manifest() during app installation
//    - Or on first launch (check if manifest exists)
//    - Update allowed_origins with real extension ID after publishing
//
// 6. Testing workflow:
//    - Load unpacked extension in Chrome
//    - Get temporary extension ID
//    - Update manifest with that ID
//    - Run app, click extension icon
//    - Verify bidirectional communication works
//
// 7. Features to implement:
//    - Screenshot full page → save to folder + clipboard + /ss expansion
//    - Web Clipper → extract text → create Sticky Note
//    - Sync settings between app and extension
//
// ============================================================================
