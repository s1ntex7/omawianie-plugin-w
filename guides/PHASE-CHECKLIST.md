# ✅ APLIKACJA 4.0 - IMPLEMENTATION CHECKLIST

**Purpose:** Track progress through all 18 plugins + shared modules.

**Instructions:**
- Mark items with `[x]` when complete
- Test thoroughly before marking as done
- If stuck, check IMPLEMENTATION-GUIDE.md for detailed steps

---

## 📦 PHASE 0: FOUNDATION

### Setup
- [ ] Backup created (`Aplikacja 3.0 - BACKUP YYYY-MM-DD-HH-mm`)
- [ ] Repository cloned/synced on home PC
- [ ] All skeleton files copied to project

### Shared Modules
- [ ] `overlay_manager.rs` copied to `src-tauri/src/`
- [ ] `clipboard_manager.rs` copied to `src-tauri/src/`
- [ ] `storage_engine.rs` copied to `src-tauri/src/`
- [ ] `file_processor.rs` copied to `src-tauri/src/`
- [ ] `browser_bridge.rs` copied to `src-tauri/src/`
- [ ] All modules declared in `main.rs` (mod statements)

### Dependencies
- [ ] `Cargo.toml` updated with dependencies from `Cargo.toml.ADDITIONS`
- [ ] `cargo check` runs without missing crate errors
- [ ] Database path configured (`app_data_dir/aplikacja_4_0.db`)
- [ ] Storage engine initialized in `.setup()`
- [ ] Clipboard manager initialized in `.setup()`

### Existing Features (Sanity Check)
- [ ] ✅ Text Expansion still works (type `/test` → expands)
- [ ] ✅ Voice-to-Text still works (F9 → record → paste)
- [ ] ✅ Screenshot Tool Core still works (F8 → select → save)

---

## 🎯 PHASE 1: TIER S PLUGINS (Must-Have, High Priority)

### Plugin 1: QR Generator (4h)
- [ ] `qr_generator.rs` copied + declared in `main.rs`
- [ ] TODOs implemented (`generate()`, `generate_and_copy()`, `image_to_data_url()`)
- [ ] IPC commands added (`qr_generate`, `qr_save`)
- [ ] React UI created (`QrGeneratorPanel.tsx`)
- [ ] ✅ **TESTED:** Generate QR from URL → copies to clipboard → displays in UI

### Plugin 2: Color Picker (6h)
- [ ] `color_picker.rs` copied + declared
- [ ] TODOs implemented (`get_pixel_color_at()`, `rgb_to_hsl()`)
- [ ] Overlay integration with `overlay_manager.rs`
- [ ] IPC commands added (`colorpicker_pick`, `colorpicker_set_format`)
- [ ] React UI created
- [ ] Global hotkey configured (Ctrl+Shift+C)
- [ ] ✅ **TESTED:** Hotkey → click pixel → color copied (HEX, RGB, HSL)

### Plugin 3: Screenshot Tool PLAN B (20-30h) 🔥
**CRITICAL: This is the highest priority feature!**

#### Phase 0: Proof of Concept (3h)
- [ ] `overlay_egui.rs` created in `src-tauri/bin/`
- [ ] Binary target added to `Cargo.toml`
- [ ] `cargo run --bin overlay_egui` runs successfully
- [ ] ✅ **VERIFIED:** Overlay fullscreen, transparent, ESC closes, **NO TRANSFORM BUG** (watch 10+ seconds)
- [ ] **DECISION:** ✅ GO to Phase 1 | ❌ NO-GO → Implement PLAN C

#### Phase 1: Region Selection (4-6h)
- [ ] `screenshot_tool_egui.rs` copied + declared
- [ ] Reused capture logic from `screenshot_new.rs`
- [ ] Overlay shows captured screens as background
- [ ] Mouse click + drag selects region
- [ ] Selection rectangle drawn with dimensions display
- [ ] Multi-monitor coordinates handled correctly
- [ ] DPI scaling handled correctly
- [ ] ✅ **TESTED:** F8 → select region on any monitor → pixel-perfect selection

#### Phase 2: Save & Clipboard (2-3h)
- [ ] Save to folder with timestamp filename (`DD-MM-YYYY-HH-MM-SS.png`)
- [ ] Copy to clipboard as image
- [ ] `/ss` text expansion updated with latest screenshot path
- [ ] IPC command `screenshot_capture` implemented
- [ ] ✅ **TESTED:** F8 → select → Ctrl+C (saves + clipboard + /ss works)

#### Phase 3: Annotations (6-8h) - **OPTIONAL for v1.0**
- [ ] Annotation toolbar UI (egui)
- [ ] Arrow tool (auto-numbered: 1., 2., 3...)
- [ ] Text tool (click + drag box, auto-wrap)
- [ ] Blur tool (pixelate region)
- [ ] Undo/Redo stack
- [ ] Ctrl+C accepts, ESC cancels
- [ ] ✅ **TESTED:** Annotations render correctly, save to image

#### Phase 4: Polish & Testing (8-12h)
- [ ] Multi-monitor edge cases tested
- [ ] DPI scaling (150%, 200%) tested
- [ ] Negative monitor coordinates tested (monitor left of primary)
- [ ] Performance optimized (< 200ms overlay spawn)
- [ ] Error handling (no crashes on ESC spam, etc.)
- [ ] Settings UI (folder selection, hotkey configuration)
- [ ] ✅ **PRODUCTION READY**

### Plugin 4: Clipboard History 2.0 (25h) 🔥
**KILLER FEATURE - Must have for v1.0!**

- [ ] `clipboard_history.rs` copied + declared
- [ ] TODOs implemented in `clipboard_manager.rs` (`watch_changes()`)
- [ ] Database schema migrations verified (storage_engine.rs)
- [ ] Clipboard watcher started in `.setup()`
- [ ] Background thread saves entries to SQLite
- [ ] Full-text search implemented (FTS5)
- [ ] Pinned entries feature works
- [ ] Ctrl+Shift+V hotkey registered
- [ ] React popup UI created (`ClipboardHistoryPanel.tsx`)
- [ ] Fuzzy search input with live results
- [ ] Click entry → pastes to clipboard
- [ ] ✅ **TESTED:**
  - [ ] Copy text/images → saved to history
  - [ ] Ctrl+Shift+V → popup shows
  - [ ] Search works (fuzzy matching)
  - [ ] Pin entry → persists across restarts
  - [ ] Old entries auto-deleted (30 days default)

### Plugin 5: Sticky Notes (10h)
- [ ] `sticky_notes.rs` copied + declared
- [ ] Database schema for notes verified
- [ ] DECISION: egui windows OR Tauri WebView windows?
- [ ] Notes persist to database
- [ ] Always-on-top windows spawn correctly
- [ ] Position + size + color saved on change
- [ ] React UI created (if using Tauri windows)
- [ ] ✅ **TESTED:** Create note → edit text → drag → persists across restarts

---

## ⚡ PHASE 2: TIER A PLUGINS (High Value, Medium Risk)

### Plugin 6: Desk Pins (5h)
- [ ] `desk_pins.rs` copied + declared
- [ ] Windows API integration (SetWindowPos)
- [ ] List windows function implemented
- [ ] Pin/unpin toggle works
- [ ] IPC commands added
- [ ] React UI created (window list with pin buttons)
- [ ] Global hotkey (Ctrl+Alt+P) to pin focused window
- [ ] ✅ **TESTED:** Pin Notepad → stays on top → unpin → goes back to normal

### Plugin 7: Audio Switcher (5h)
- [ ] `audio_switcher.rs` copied + declared
- [ ] Windows audio device enumeration works
- [ ] `nircmd.exe` bundled in `src-tauri/resources/`
- [ ] Switch device function works
- [ ] System tray icon integration (optional)
- [ ] React UI created (device list with switch buttons)
- [ ] ✅ **TESTED:** Switch from speakers to headphones → audio output changes

### Plugin 8: Image Resizer (8h)
- [ ] `image_resizer.rs` copied + declared
- [ ] `file_processor.rs` integration complete
- [ ] Presets implemented (Thumbnail, Web Small, Web Medium, Web Large)
- [ ] Batch resize function works
- [ ] Output folder configuration
- [ ] React UI created (drag & drop + preset selector)
- [ ] ✅ **TESTED:** Drag 10 images → select preset → resize all → verify output

### Plugin 9: Screen Recorder (25h)
- [ ] `screen_recorder.rs` copied + declared
- [ ] ffmpeg installed OR bundled
- [ ] Region selection (reuses overlay_manager)
- [ ] GIF recording works (fps=15)
- [ ] MP4 recording works (H.264 codec)
- [ ] Optional audio capture
- [ ] Start/stop hotkeys
- [ ] React UI created
- [ ] ✅ **TESTED:** Record 10-second GIF → verify output file plays

### Plugin 10: OCR (15h)
- [ ] `ocr.rs` copied + declared
- [ ] Tesseract installed OR cloud API key configured
- [ ] Extract text from image function works
- [ ] Clipboard integration (copy extracted text)
- [ ] Integration with Screenshot Tool (OCR button in toolbar)
- [ ] React UI created (optional - can be toolbar button only)
- [ ] ✅ **TESTED:** Screenshot text → OCR → text copied to clipboard

---

## 🔧 PHASE 3: TIER B PLUGINS (Nice to Have)

### Plugin 11: URL Shortener (10h)
- [ ] `url_shortener.rs` copied + declared
- [ ] API endpoint configured (YOURLS, Bitly, or self-hosted)
- [ ] API key added to settings
- [ ] Shorten URL function works
- [ ] Clipboard integration (copy shortened URL)
- [ ] React UI created
- [ ] ✅ **TESTED:** Shorten URL → copies to clipboard → opens in browser

### Plugin 12: File Converter (15h)
- [ ] `file_converter.rs` skeleton created (not provided - use `file_processor.rs`)
- [ ] ffmpeg integration for media conversion
- [ ] LibreOffice integration for document conversion (optional)
- [ ] Batch conversion queue
- [ ] React UI created (drag & drop + format selector)
- [ ] ✅ **TESTED:** Convert AVI to MP4 → verify output file plays

### Plugin 13: File Manager AI (12h)
- [ ] `file_manager_ai.rs` skeleton created (not provided)
- [ ] Filesystem watcher implemented (use `notify` crate from `file_processor.rs`)
- [ ] Groq API integration (similar to `voice_to_text.rs`)
- [ ] Auto-categorization rules
- [ ] React UI created (folder watch config + rules)
- [ ] ✅ **TESTED:** Drop file in Downloads → auto-moved to correct folder

### Plugin 14: Browser Extension (15h)
- [ ] `browser_bridge.rs` integration in `.setup()`
- [ ] Native messaging manifest installed
- [ ] `browser-extension/` folder created
- [ ] `manifest.json` created (Chrome + Firefox)
- [ ] `background.js` implemented (full-page screenshot)
- [ ] Extension loaded in Chrome (unpacked)
- [ ] Bidirectional messaging works (extension ↔ app)
- [ ] ✅ **TESTED:** Click extension icon → screenshot sent to app → saved

### Plugin 15: Web Clipper (5h)
- [ ] Integration with `browser_bridge.rs` + `sticky_notes.rs`
- [ ] Text extraction in extension (`content.js`)
- [ ] Create Sticky Note from clipped text
- [ ] ✅ **TESTED:** Clip text from webpage → Sticky Note created

---

## 💪 PHASE 4: TIER C PLUGINS (Power Users, Optional)

### Plugin 16: Window Manager (Tiling) (20h)
**OPTIONAL - Can be skipped for v1.0**

- [ ] `window_manager.rs` skeleton created (not provided)
- [ ] Windows API deep integration
- [ ] Snap zones implemented
- [ ] Custom layouts
- [ ] Hotkeys (Win+Arrow override)
- [ ] Multi-monitor zone sync
- [ ] React UI created (layout editor)
- [ ] ✅ **TESTED:** Win+Left → window snaps to left half

---

## 📦 PRODUCTION BUILD

### Pre-Build Checklist
- [ ] All planned plugins implemented ✅
- [ ] All features tested individually
- [ ] Integration testing (multiple plugins running simultaneously)
- [ ] Performance testing (no memory leaks, < 200ms response)
- [ ] Error handling verified (graceful failures, no crashes)
- [ ] Hotkeys documented (no conflicts)
- [ ] Settings UI complete (all configurable options accessible)

### Version & Metadata
- [ ] Version number updated in `package.json` (e.g., `"version": "1.0.0"`)
- [ ] Version number updated in `src-tauri/tauri.conf.json`
- [ ] App name finalized (`Aplikacja 4.0` or new name)
- [ ] App icon updated (if changed)
- [ ] Bundle identifier confirmed (`com.aplikacja.4-0`)

### Build Commands
```bash
cd "C:\PennyLane\Aplikacja 4.0"
npm run tauri build
```

- [ ] Build completes without errors (~2-5 minutes)
- [ ] Output files verified:
  - [ ] `aplikacja_4_0.exe` (Portable)
  - [ ] `Aplikacja 4.0_1.0.0_x64_en-US.msi` (MSI Installer)
  - [ ] `Aplikacja 4.0_1.0.0_x64-setup.exe` (NSIS Installer)

### Copy to `gotowe instalki/`
- [ ] Folder created: `Aplikacja 4.0/gotowe instalki/`
- [ ] Files copied with clear names:
  - [ ] `Aplikacja_4.0_v1.0.0_PORTABLE.exe`
  - [ ] `Aplikacja_4.0_v1.0.0_INSTALLER_MSI.msi`
  - [ ] `Aplikacja_4.0_v1.0.0_INSTALLER_NSIS.exe`

### Post-Build Testing
- [ ] Portable EXE runs on clean Windows 11 VM (no dependencies)
- [ ] MSI installer installs correctly (Add/Remove Programs entry)
- [ ] NSIS installer installs correctly
- [ ] All plugins work after installation
- [ ] Database created on first run
- [ ] Settings persist across restarts
- [ ] Uninstaller works correctly (removes all files)

---

## 🎉 COMPLETION CHECKLIST

### v1.0 Release Criteria (Minimum)
- [ ] ✅ Screenshot Tool PLAN B (P0)
- [ ] ✅ Clipboard History 2.0 (P0)
- [ ] ✅ QR Generator (P1)
- [ ] ✅ Color Picker (P1)
- [ ] ✅ Sticky Notes (P1)
- [ ] ✅ Desk Pins (P1)
- [ ] ✅ Audio Switcher (P1)
- [ ] ✅ Image Resizer (P1)

### v1.1 Release (Extended Features)
- [ ] Screen Recorder
- [ ] OCR
- [ ] URL Shortener
- [ ] File Converter
- [ ] File Manager AI

### v2.0 Release (Advanced)
- [ ] Browser Extension + Web Clipper
- [ ] Window Manager (Tiling)
- [ ] Additional features (to be determined)

---

## 📊 PROGRESS SUMMARY

**Shared Modules:** `[ ] 0/5 complete`

**TIER S Plugins:** `[ ] 0/5 complete`
- Screenshot Tool, Clipboard History 2.0, QR Generator, Color Picker, Sticky Notes

**TIER A Plugins:** `[ ] 0/5 complete`
- Desk Pins, Audio Switcher, Image Resizer, Screen Recorder, OCR

**TIER B Plugins:** `[ ] 0/5 complete`
- URL Shortener, File Converter, File Manager AI, Browser Extension, Web Clipper

**TIER C Plugins:** `[ ] 0/1 complete`
- Window Manager

**TOTAL PROGRESS:** `[ ] 0/21 complete` (5 modules + 16 plugins minimum for v1.0)

---

**Last Updated:** 2025-11-10
**Next Update:** After each plugin completion, update progress summary above
**Ready For:** Terminal agent to start Phase 0 → Foundation setup
