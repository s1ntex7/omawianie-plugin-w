# 🚀 APLIKACJA 4.0 - IMPLEMENTATION GUIDE FOR TERMINAL AGENT

**Purpose:** This guide provides step-by-step instructions for implementing all 18 plugins in Aplikacja 4.0 using the provided skeleton files.

**Context:** You are an AI agent working in the terminal on a Windows PC. The user has provided:
- 5 shared modules (`shared-modules/`)
- 11 plugin skeletons (`rust-skeletons/`)
- This implementation guide
- MASTER-PLAN-18-PLUGINS.md (overall strategy)

**Your task:** Integrate these skeletons into the existing Aplikacja 3.0 codebase and implement the TODOs.

---

## 📂 PROJECT STRUCTURE

```
C:\PennyLane\Aplikacja 3.0\  (or Aplikacja 4.0 if starting fresh)
├── src/                          (React frontend)
│   ├── components/
│   │   ├── TextExpansionPanel.tsx       ✅ Already works
│   │   ├── ScreenshotOverlay.tsx        ✅ Already works
│   │   ├── ClipboardHistoryPanel.tsx    ⏳ To be created
│   │   ├── StickyNotesPanel.tsx         ⏳ To be created
│   │   └── ...
│   └── App.tsx
├── src-tauri/                    (Rust backend)
│   └── src/
│       ├── main.rs                      ✅ Exists - needs updates
│       ├── simple_expansion.rs          ✅ Already works
│       ├── voice_to_text.rs             ✅ Already works
│       ├── screenshot_new.rs            ✅ Already works
│       │
│       ├── overlay_manager.rs           ⏳ Copy from shared-modules/
│       ├── clipboard_manager.rs         ⏳ Copy from shared-modules/
│       ├── storage_engine.rs            ⏳ Copy from shared-modules/
│       ├── file_processor.rs            ⏳ Copy from shared-modules/
│       ├── browser_bridge.rs            ⏳ Copy from shared-modules/
│       │
│       ├── screenshot_tool_egui.rs      ⏳ Copy from rust-skeletons/
│       ├── clipboard_history.rs         ⏳ Copy from rust-skeletons/
│       ├── sticky_notes.rs              ⏳ Copy from rust-skeletons/
│       ├── qr_generator.rs              ⏳ Copy from rust-skeletons/
│       ├── color_picker.rs              ⏳ Copy from rust-skeletons/
│       ├── desk_pins.rs                 ⏳ Copy from rust-skeletons/
│       ├── audio_switcher.rs            ⏳ Copy from rust-skeletons/
│       ├── image_resizer.rs             ⏳ Copy from rust-skeletons/
│       ├── url_shortener.rs             ⏳ Copy from rust-skeletons/
│       ├── screen_recorder.rs           ⏳ Copy from rust-skeletons/
│       └── ocr.rs                       ⏳ Copy from rust-skeletons/
│   └── Cargo.toml                       ⏳ Update with dependencies
└── package.json
```

---

## 🔧 PHASE 0: SETUP & FOUNDATION

### Step 0.1: Backup Current Project

```bash
cd "C:\PennyLane"
robocopy "Aplikacja 3.0" "Aplikacja 3.0 - BACKUP $(Get-Date -Format 'yyyy-MM-dd-HH-mm')" /E /NFL /NDL /NJH
```

**Verify:** Check that backup folder exists and contains all files.

### Step 0.2: Copy Shared Modules

```bash
cd "C:\PennyLane\Aplikacja 3.0\src-tauri\src"

# Copy all shared modules
cp /path/to/shared-modules/overlay_manager.rs ./
cp /path/to/shared-modules/clipboard_manager.rs ./
cp /path/to/shared-modules/storage_engine.rs ./
cp /path/to/shared-modules/file_processor.rs ./
cp /path/to/shared-modules/browser_bridge.rs ./
```

### Step 0.3: Add Module Declarations to main.rs

Edit `src-tauri/src/main.rs` and add at the top:

```rust
// Shared modules
mod overlay_manager;
mod clipboard_manager;
mod storage_engine;
mod file_processor;
mod browser_bridge;

// Plugins
mod screenshot_tool_egui;
mod clipboard_history;
mod sticky_notes;
mod qr_generator;
mod color_picker;
mod desk_pins;
mod audio_switcher;
mod image_resizer;
mod url_shortener;
mod screen_recorder;
mod ocr;

// Existing modules (already declared)
// mod simple_expansion;
// mod voice_to_text;
// mod screenshot_new;
```

### Step 0.4: Update Cargo.toml

Copy dependencies from `Cargo.toml.ADDITIONS` (see that file for full list).

**CRITICAL DEPENDENCIES:**

```toml
[dependencies]
# Existing...
tauri = { version = "2.3.1", features = ["tray-icon", "protocol-asset"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# NEW: Shared modules
egui = "0.29.1"
eframe = "0.29.1"
rusqlite = { version = "0.32", features = ["bundled", "chrono"] }
chrono = "0.4"
arboard = "3.4.1"  # Already exists
image = "0.25"      # Already exists
screenshots = "0.10" # Already exists

# NEW: Plugins
qrcode = "0.14"
base64 = "0.22"
notify = "6.1"

[target.'cfg(windows)'.dependencies]
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_Media_Audio",
    "Win32_System_Com",
]}
```

**After updating Cargo.toml:**

```bash
cd "C:\PennyLane\Aplikacja 3.0\src-tauri"
cargo check
```

**Expected:** Compilation errors (TODOs not implemented), but no missing crate errors.

---

## 🎯 PHASE 1: CORE SHARED MODULES (Week 1-2)

### 1.1: Initialize Shared Modules in main.rs

Edit `.setup()` function in `main.rs`:

```rust
.setup(|app| {
    tracing::info!("🔧 setup() start");

    // ========================================
    // SHARED MODULES INITIALIZATION
    // ========================================

    // Storage Engine (SQLite database)
    let db_path = app.path().app_data_dir()?.join("aplikacja_4_0.db");
    let storage = storage_engine::StorageEngine::new(db_path)?;
    let storage = Arc::new(Mutex::new(storage));
    app.manage(storage.clone());

    // Clipboard Manager
    let clipboard_mgr = clipboard_manager::ClipboardManager::new()?;
    let clipboard_mgr = Arc::new(Mutex::new(clipboard_mgr));
    app.manage(clipboard_mgr.clone());

    // Overlay Manager (used by Screenshot, Color Picker, Screen Recorder)
    let overlay_mgr = overlay_manager::OverlayManager::new();
    app.manage(overlay_mgr);

    // File Processor (used by Image Resizer, File Converter)
    let file_processor = file_processor::FileProcessor::new();
    app.manage(Mutex::new(file_processor));

    // ========================================
    // EXISTING FEATURES
    // ========================================

    // Text Expansion (already works)
    simple_expansion::spawn_expansion_listener(app.handle().clone())?;

    // Global shortcuts
    let gs = app.global_shortcut();

    // F9 → Voice-to-Text (already works)
    gs.on_shortcut("F9", {
        let app = app.handle().clone();
        move |_app, _shortcut, event| {
            tracing::info!("🎹 F9 {:?}", event);
            if format!("{:?}", event).contains("Pressed") {
                let _ = app.emit_to("main", "vtt:hotkey", ());
            }
        }
    })?;

    // F8 → Screenshot (upgrade to PLAN B later)
    gs.on_shortcut("F8", {
        let app = app.handle().clone();
        move |_app, _shortcut, event| {
            if format!("{:?}", event).contains("Pressed") {
                let _ = app.emit_to("main", "screenshot:capture", ());
            }
        }
    })?;

    tracing::info!("✅ setup() done");
    Ok(())
})
```

**Test:** Run `cargo build` - should compile without errors (just warnings for unused modules).

### 1.2: Add IPC Commands

Add before `.setup()` in `main.rs`:

```rust
.invoke_handler(tauri::generate_handler![
    // Existing commands...
    // expansion_add_shortcut,
    // expansion_list_shortcuts,
    // ...

    // NEW: Storage
    storage_set_setting,
    storage_get_setting,

    // NEW: Clipboard
    clipboard_paste_text,
    clipboard_paste_image,

    // Plugins (add as you implement them)
    qr_generate,
    qr_save,
    colorpicker_pick,
    screenshot_capture,
    // ...
])
```

Implement commands at bottom of `main.rs`:

```rust
// ============================================================================
// IPC COMMANDS - STORAGE
// ============================================================================

#[tauri::command]
fn storage_set_setting(
    key: String,
    value: String,
    state: tauri::State<Arc<Mutex<storage_engine::StorageEngine>>>
) -> Result<(), String> {
    state.lock().unwrap().set_setting(&key, &value)
}

#[tauri::command]
fn storage_get_setting(
    key: String,
    state: tauri::State<Arc<Mutex<storage_engine::StorageEngine>>>
) -> Result<Option<String>, String> {
    state.lock().unwrap().get_setting(&key)
}

// ============================================================================
// IPC COMMANDS - CLIPBOARD
// ============================================================================

#[tauri::command]
fn clipboard_paste_text(
    text: String,
    state: tauri::State<Arc<Mutex<clipboard_manager::ClipboardManager>>>
) -> Result<(), String> {
    state.lock().unwrap().paste_text(&text)
}

// TODO: Add clipboard_paste_image, clipboard_get_history, etc.
```

**Test:**
```bash
cargo build
npm run dev
```

Check that app starts without crashing.

---

## 📝 PHASE 2: IMPLEMENT PLUGINS ONE-BY-ONE

**Strategy:** Implement plugins in priority order (TIER S → A → B → C).

### 2.1: QR Generator (Quick Win - 4 hours)

**Why first?** Simplest plugin, no complex dependencies, instant gratification.

#### Step 2.1.1: Copy skeleton

```bash
cp /path/to/rust-skeletons/qr_generator.rs "C:\PennyLane\Aplikacja 3.0\src-tauri\src\"
```

#### Step 2.1.2: Add to main.rs .setup()

```rust
// QR Generator
let qr_gen = qr_generator::QrGenerator::new(clipboard_mgr.lock().unwrap().clone());
app.manage(qr_gen);
```

#### Step 2.1.3: Implement TODOs in qr_generator.rs

Open `src-tauri/src/qr_generator.rs` and implement:
- `generate()` - Use `qrcode` crate to generate QR code
- `generate_and_copy()` - Call `generate()` + clipboard copy
- `image_to_data_url()` - Convert to base64 PNG

**Reference:** Look at existing code in `screenshot_new.rs` for image handling.

#### Step 2.1.4: Add IPC commands

```rust
#[tauri::command]
fn qr_generate(text: String, state: tauri::State<qr_generator::QrGenerator>) -> Result<String, String> {
    state.generate_and_copy(&text)
}

#[tauri::command]
fn qr_save(text: String, file_path: String, state: tauri::State<qr_generator::QrGenerator>) -> Result<(), String> {
    state.save_to_file(&text, file_path.into())
}
```

Add to `.invoke_handler()`.

#### Step 2.1.5: Create React UI

Create `src/components/QrGeneratorPanel.tsx`:

```tsx
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'react-hot-toast';

export const QrGeneratorPanel = () => {
  const [text, setText] = useState("");
  const [qrDataUrl, setQrDataUrl] = useState<string | null>(null);

  const generate = async () => {
    try {
      const dataUrl = await invoke<string>("qr_generate", { text });
      setQrDataUrl(dataUrl);
      toast.success("QR code copied to clipboard!");
    } catch (err) {
      toast.error(`Failed: ${err}`);
    }
  };

  return (
    <div className="panel">
      <h2>QR Code Generator</h2>
      <input
        value={text}
        onChange={(e) => setText(e.target.value)}
        placeholder="Enter text or URL"
      />
      <button onClick={generate}>Generate</button>
      {qrDataUrl && <img src={qrDataUrl} alt="QR Code" />}
    </div>
  );
};
```

Add to `App.tsx`:

```tsx
import { QrGeneratorPanel } from './components/QrGeneratorPanel';

// In render:
<QrGeneratorPanel />
```

#### Step 2.1.6: Test

```bash
npm run dev
```

1. Open app
2. Go to QR Generator panel
3. Enter "https://example.com"
4. Click Generate
5. ✅ QR code appears
6. ✅ URL copied to clipboard (test with Ctrl+V in Notepad)

**If working:** ✅ QR Generator complete! Move to next plugin.

**If errors:** Check console logs, fix TODOs, retry.

---

### 2.2: Color Picker (Quick Win - 6 hours)

Follow same pattern as QR Generator:
1. Copy `color_picker.rs`
2. Add to `main.rs`
3. Implement TODOs (pixel grabbing from `screenshot_new.rs`)
4. Add IPC commands
5. Create React UI
6. Test

**CRITICAL:** Reuse coordinate math from `screenshot_new.rs` for pixel color grabbing!

---

### 2.3: Screenshot Tool PLAN B (Priority P0 - 20-30 hours)

**This is the BIG ONE.** Follow SCREENSHOT-MASTER-PLAN.md Phase 0-4.

#### Phase 0: Proof of Concept (3 hours)

Create `src-tauri/bin/overlay_egui.rs`:

```rust
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top(true)
            .with_fullscreen(true),
        ..Default::default()
    };

    eframe::run_simple_native("Overlay POC", options, move |ctx, _| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PROOF OF CONCEPT - ESC to close");
        });

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    })
}
```

**Test:**

```bash
cargo run --bin overlay_egui
```

**VERIFY:**
- ✅ Overlay appears fullscreen
- ✅ Transparent background
- ✅ ESC closes
- ✅ **NO WINDOW TRANSFORM BUG** (watch for 10+ seconds)

**DECISION POINT:**
- ✅ **IF WORKS:** Continue to Phase 1 (region selection)
- ❌ **IF FAILS:** Escalate to PLAN C (see SCREENSHOT-MASTER-PLAN.md)

#### Phase 1-4: Full Implementation

See `SCREENSHOT-MASTER-PLAN.md` for detailed steps (11,000 words of instructions).

**Key integration points:**
- Reuse `screenshot_new.rs` capture logic
- Use `overlay_manager.rs` for overlay spawning
- Integrate with `/ss` text expansion (update `simple_expansion.rs`)

---

### 2.4: Clipboard History 2.0 (Killer Feature - 25 hours)

**Priority:** P0 - This is a MUST-HAVE killer feature.

Follow `clipboard_history.rs` skeleton + `MASTER-PLAN-18-PLUGINS.md` Phase 2.

**Key steps:**
1. Start clipboard watcher in `.setup()`
2. Implement `watch_changes()` in `clipboard_manager.rs`
3. Save entries to SQLite (schema in `storage_engine.rs`)
4. Add Ctrl+Shift+V hotkey to show popup
5. Create React popup UI with fuzzy search
6. Test: Copy text/images → Check history → Search → Paste

---

### 2.5: Sticky Notes (High Value - 10 hours)

Follow `sticky_notes.rs` skeleton.

**Key decision:** Use egui windows or Tauri WebView windows?
- **egui:** Native, lightweight, but complex rendering
- **Tauri WebView:** Easier React UI, but more memory

**Recommendation:** Start with Tauri WebView windows (one window per note).

---

### 2.6: Desk Pins (Quick Win - 5 hours)

Windows-only initially. macOS/Linux can be added later.

**Test:** Pin Notepad window, verify it stays on top when other apps are focused.

---

### 2.7: Audio Switcher (Niche - 5 hours)

Windows implementation with `windows-rs` crate.

**IMPORTANT:** Bundle `nircmd.exe` in installers for device switching.

---

### 2.8: Image Resizer (Useful - 8 hours)

Uses `file_processor.rs` + `image` crate.

**Test:** Drag 10 JPGs, select "Web Medium" preset, resize all → Check output folder.

---

### 2.9: URL Shortener (Nice to Have - 10 hours)

**Prerequisite:** Setup API (YOURLS, Bitly, or self-hosted).

Can be skipped for v1.0 if no API available.

---

### 2.10: Screen Recorder (Advanced - 25 hours)

**Prerequisite:** ffmpeg installed or bundled.

GIF recording is simpler than MP4 (start there).

**Test:** Record 5-second GIF, verify output file.

---

### 2.11: OCR (Killer Feature - 15 hours)

**Prerequisite:** Tesseract installed or cloud API key.

Integrate with Screenshot Tool (add "OCR" button to toolbar).

---

## ✅ TESTING CHECKLIST

After each plugin implementation:

### Functional Tests
- [ ] Plugin activates (hotkey/button works)
- [ ] Core functionality works as expected
- [ ] Error handling graceful (no crashes)
- [ ] IPC commands respond correctly
- [ ] UI updates in real-time

### Integration Tests
- [ ] Doesn't break existing features (Text Expansion, VTT)
- [ ] Hotkeys don't conflict
- [ ] Multiple plugins can run simultaneously
- [ ] Database writes don't block UI

### Cross-Platform Tests (if applicable)
- [ ] Works on Windows 10/11
- [ ] Multi-monitor support
- [ ] DPI scaling handled correctly (150%, 200%)
- [ ] Negative monitor coordinates (monitor left of primary)

### Performance Tests
- [ ] No memory leaks (run for 30+ minutes)
- [ ] Fast response (<200ms for user actions)
- [ ] Database queries optimized (EXPLAIN QUERY PLAN)

---

## 🚨 TROUBLESHOOTING

### "Cannot find crate" errors
- Check `Cargo.toml` has all dependencies
- Run `cargo clean && cargo build`

### "Module not found" errors
- Check `mod xyz;` declaration in `main.rs`
- Verify file exists at `src-tauri/src/xyz.rs`

### Window transform bug (Screenshot Tool)
- Verify egui POC worked first
- Check SCREENSHOT-MASTER-PLAN.md Phase 0 troubleshooting
- If still fails, implement PLAN C (hybrid approach)

### Database errors
- Check `aplikacja_4_0.db` file exists in app_data_dir
- Verify migrations ran (check `migrations` table)
- Run `storage.vacuum()` to repair

### IPC command not found
- Check `.invoke_handler()` includes command
- Verify function signature matches `#[tauri::command]` requirements
- Check TypeScript `invoke()` call matches Rust function name (snake_case)

---

## 📦 PRODUCTION BUILD

When all plugins are implemented and tested:

### Build Process

```bash
cd "C:\PennyLane\Aplikacja 4.0"

# Update version in package.json and tauri.conf.json
# Example: "version": "1.0.0"

# Build
npm run tauri build

# Wait ~2-5 minutes for compilation
```

### Output Files

```
src-tauri/target/release/
├── aplikacja_4_0.exe                                    (Portable)
└── bundle/
    ├── msi/Aplikacja 4.0_1.0.0_x64_en-US.msi          (MSI Installer)
    └── nsis/Aplikacja 4.0_1.0.0_x64-setup.exe         (NSIS Installer)
```

### Copy to `gotowe instalki/`

```bash
mkdir "C:\PennyLane\Aplikacja 4.0\gotowe instalki"

cp "src-tauri/target/release/aplikacja_4_0.exe" `
   "gotowe instalki/Aplikacja_4.0_v1.0.0_PORTABLE.exe"

cp "src-tauri/target/release/bundle/msi/Aplikacja 4.0_1.0.0_x64_en-US.msi" `
   "gotowe instalki/Aplikacja_4.0_v1.0.0_INSTALLER_MSI.msi"

cp "src-tauri/target/release/bundle/nsis/Aplikacja 4.0_1.0.0_x64-setup.exe" `
   "gotowe instalki/Aplikacja_4.0_v1.0.0_INSTALLER_NSIS.exe"
```

---

## 🎯 SUMMARY FOR TERMINAL AGENT

**Your workflow:**

1. **Read this guide carefully** (you are here!)
2. **Phase 0:** Setup shared modules, update `main.rs`, add dependencies
3. **Phase 1:** Implement plugins one-by-one in priority order
4. **For each plugin:**
   - Copy skeleton to `src-tauri/src/`
   - Implement TODOs (search for `// TODO:` comments)
   - Add IPC commands
   - Create React UI
   - Test thoroughly
   - ✅ Mark as complete, move to next
5. **Production build** when all features work
6. **Report back** to user with status + installer paths

**Remember:**
- ✅ Reuse existing code (screenshot_new.rs, simple_expansion.rs)
- ✅ Test after each plugin (don't batch!)
- ✅ Ask user if stuck (don't spin for hours)
- ✅ Commit frequently with clear messages

**You got this!** 🚀

---

**Last Updated:** 2025-11-10 (prepared during browser session)
**Prepared By:** Claude Code + Aleks
**Ready For:** Terminal agent implementation at home PC
