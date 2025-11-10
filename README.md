# 🚀 APLIKACJA 4.0 - COMPLETE PLUGIN SKELETON PACKAGE

**Created:** 2025-11-10 (Browser session)
**For:** Terminal agent implementation at home PC
**Purpose:** Ready-to-integrate skeletons for all 18 plugins + shared modules

---

## 📦 WHAT'S INSIDE

This repository contains **production-ready skeletons** for Aplikacja 4.0, including:

- ✅ **5 Shared Modules** (foundation for all plugins)
- ✅ **11 Plugin Skeletons** (with complete API signatures + integration notes)
- ✅ **IMPLEMENTATION-GUIDE.md** (step-by-step for terminal agent)
- ✅ **Cargo.toml.ADDITIONS** (all dependencies in one place)
- ✅ **PHASE-CHECKLIST.md** (track progress through implementation)
- ✅ **3 React UI Components** (examples for frontend)
- ✅ **MASTER-PLAN-18-PLUGINS.md** (strategic overview from initial planning)

---

## 🎯 QUICK START FOR TERMINAL AGENT

### 1. Read Documentation First

Start with these files **in order**:

1. **`MASTER-PLAN-18-PLUGINS.md`** (browser planning session - understand strategy)
2. **`guides/IMPLEMENTATION-GUIDE.md`** (your main reference - step-by-step)
3. **`guides/PHASE-CHECKLIST.md`** (track your progress)

### 2. Foundation Setup (Phase 0)

```bash
# Backup existing project
cd "C:\PennyLane"
robocopy "Aplikacja 3.0" "Aplikacja 3.0 - BACKUP $(Get-Date -Format 'yyyy-MM-dd-HH-mm')" /E

# Copy shared modules
cd "Aplikacja 3.0\src-tauri\src"
cp /path/to/omawianie-plugin-w/shared-modules/*.rs ./

# Update main.rs with module declarations (see IMPLEMENTATION-GUIDE.md)
# Update Cargo.toml with dependencies (see Cargo.toml.ADDITIONS)

cargo check  # Verify no missing crates
```

### 3. Implement Plugins (Phase 1+)

Follow priority order from IMPLEMENTATION-GUIDE.md:

**TIER S (Must-Have):**
1. QR Generator (4h) - Quick win!
2. Color Picker (6h)
3. Screenshot Tool PLAN B (20-30h) 🔥 - **TOP PRIORITY**
4. Clipboard History 2.0 (25h) 🔥 - **KILLER FEATURE**
5. Sticky Notes (10h)

**TIER A (High Value):**
6. Desk Pins (5h)
7. Audio Switcher (5h)
8. Image Resizer (8h)
9. Screen Recorder (25h)
10. OCR (15h)

For each plugin:
- Copy skeleton from `rust-skeletons/` to `src-tauri/src/`
- Implement TODOs (search for `// TODO:` comments)
- Add IPC commands to `main.rs`
- Create React UI (see examples in `react-components/`)
- Test thoroughly
- Mark as complete in `PHASE-CHECKLIST.md`

---

## 📂 FILE STRUCTURE

```
omawianie-plugin-w/
├── README.md                              # This file
├── MASTER-PLAN-18-PLUGINS.md              # Strategic overview (from browser planning)
│
├── shared-modules/                        # Core infrastructure
│   ├── overlay_manager.rs                 # egui overlays (Screenshot, Color Picker, Recorder)
│   ├── clipboard_manager.rs               # Unified clipboard API + history tracking
│   ├── storage_engine.rs                  # SQLite + FTS5 (database layer)
│   ├── file_processor.rs                  # Batch file ops (Image Resizer, Converter)
│   └── browser_bridge.rs                  # Native messaging (Browser Extension)
│
├── rust-skeletons/                        # Plugin implementations
│   ├── screenshot_tool_egui.rs            # PLAN B (native egui rendering)
│   ├── clipboard_history.rs               # Clipboard History 2.0 (killer feature)
│   ├── sticky_notes.rs                    # Floating notes (egui windows)
│   ├── qr_generator.rs                    # QR code generation
│   ├── color_picker.rs                    # Global color picker
│   ├── desk_pins.rs                       # Always-on-top windows
│   ├── audio_switcher.rs                  # Quick audio device switching
│   ├── image_resizer.rs                   # Batch image resize
│   ├── url_shortener.rs                   # URL shortening service
│   ├── screen_recorder.rs                 # GIF/MP4 screen recording
│   └── ocr.rs                             # OCR (tesseract/cloud API)
│
├── react-components/                      # UI examples
│   ├── ClipboardHistoryPanel.tsx          # Full popup UI with search
│   ├── QrGeneratorPanel.tsx               # Simple input + preview
│   └── ColorPickerPanel.tsx               # Color display with formats
│
└── guides/                                # Documentation
    ├── IMPLEMENTATION-GUIDE.md            # Main reference (step-by-step)
    ├── Cargo.toml.ADDITIONS               # All dependencies
    └── PHASE-CHECKLIST.md                 # Progress tracker
```

---

## 🔑 KEY FEATURES

### Shared Modules (Reusable Components)

#### `overlay_manager.rs`
- **Purpose:** Unified API for fullscreen native overlays
- **Used by:** Screenshot Tool, Color Picker, Screen Recorder, OCR
- **Tech:** egui v0.29.1, eframe v0.29.1
- **Key functions:**
  - `spawn_region_selector()` - Select screen region
  - `spawn_point_selector()` - Click single point

#### `clipboard_manager.rs`
- **Purpose:** Cross-platform clipboard with history tracking
- **Used by:** All plugins that interact with clipboard
- **Tech:** arboard crate
- **Key functions:**
  - `paste_text()`, `paste_image()`
  - `watch_changes()` - Background monitoring
  - `search()` - Fuzzy search history

#### `storage_engine.rs`
- **Purpose:** SQLite database with migrations + FTS5
- **Used by:** Clipboard History, Sticky Notes, Settings
- **Tech:** rusqlite v0.32 with bundled SQLite
- **Key functions:**
  - `execute()`, `query()`
  - `search_clipboard()` - Full-text search
  - `migrate()` - Schema versioning

#### `file_processor.rs`
- **Purpose:** Batch file operations + filesystem watching
- **Used by:** Image Resizer, File Converter, File Manager AI
- **Tech:** image crate, ffmpeg CLI, notify crate
- **Key functions:**
  - `add_job()`, `process_all()`
  - `convert_image()`, `resize_image()`, `convert_media()`

#### `browser_bridge.rs`
- **Purpose:** Native messaging for browser extensions
- **Used by:** Browser Extension, Web Clipper
- **Tech:** stdin/stdout JSON protocol (Chrome/Firefox)
- **Key functions:**
  - `init()` - Start listening
  - `recv()`, `send()` - Bidirectional messaging
  - `install_manifest()` - Register with browser

---

## 🎨 PLUGIN HIGHLIGHTS

### Screenshot Tool PLAN B 🔥
**File:** `screenshot_tool_egui.rs`

**Why PLAN B?** After 15+ hours debugging, we discovered Tauri WebView has a **window transform bug** on multi-monitor setups. PLAN B uses **native egui rendering** to bypass this limitation entirely.

**Status:** Skeleton complete with integration notes

**Phases:**
- Phase 0: POC (3h) - Verify egui works without transform bug
- Phase 1: Region selection (4-6h)
- Phase 2: Save + clipboard + `/ss` expansion (2-3h)
- Phase 3: Annotations (6-8h) - arrows, text, blur
- Phase 4: Polish + testing (8-12h)

**CRITICAL:** Reuse coordinate math from existing `screenshot_new.rs` (proven to work).

---

### Clipboard History 2.0 🔥
**File:** `clipboard_history.rs`

**Why Killer Feature?** This is the #1 requested feature. Ctrl+Shift+V popup with instant search beats Windows built-in clipboard.

**Features:**
- Background clipboard monitoring
- SQLite storage with FTS5 search
- Text + images
- Pin favorites (never expire)
- Fuzzy search
- Auto-delete old entries (30 days configurable)

**UI:** Full React popup (see `ClipboardHistoryPanel.tsx`)

---

### QR Generator (Quick Win)
**File:** `qr_generator.rs`

**Why First?** Simplest plugin, no complex dependencies, instant gratification.

**Time:** 4 hours total

**Features:**
- Generate QR from text/URL
- Copy to clipboard automatically
- Save to file (PNG)
- SVG export (scalable)

---

### Color Picker
**File:** `color_picker.rs`

**Features:**
- Click anywhere to pick color
- All formats: HEX, RGB, RGBA, HSL, HSLA
- Auto-copy default format to clipboard
- Multi-monitor support
- DPI scaling aware

**Integration:** Uses `overlay_manager.rs` for fullscreen selection

---

### Sticky Notes
**File:** `sticky_notes.rs`

**Features:**
- Always-on-top floating windows
- Persistent (saved to SQLite)
- Position + size + color configurable
- Multiple notes simultaneously

**Decision:** Use egui windows OR Tauri WebView windows (see implementation notes)

---

## ⚠️ IMPORTANT NOTES

### 1. Not All Plugins Have Skeletons

The following plugins need skeletons created (use existing ones as templates):

- **File Converter** (use `file_processor.rs` patterns)
- **File Manager AI** (use `file_processor.rs` + `voice_to_text.rs` Groq API patterns)
- **Window Manager** (complex - TIER C, optional for v1.0)
- **Browser Extension** (use `browser_bridge.rs`)
- **Web Clipper** (combine `browser_bridge.rs` + `sticky_notes.rs`)

### 2. Text Expansion + Voice-to-Text Already Work

These plugins are **complete** in Aplikacja 3.0:
- ✅ `simple_expansion.rs` - Global keyboard listener, auto-save to JSON
- ✅ `voice_to_text.rs` - F9 hotkey, Groq Whisper API

**DO NOT rewrite them!** Just maintain and integrate with new features (e.g., /ss expansion).

### 3. Screenshot Tool Core Exists

`screenshot_new.rs` has **proven multi-monitor capture logic**:
- Handles negative coordinates (monitor left of primary)
- DPI scaling correct
- Pixel-perfect selection

**REUSE THIS CODE** in `screenshot_tool_egui.rs`! Don't rewrite coordinate math from scratch.

### 4. External Dependencies

Some plugins require external tools:

- **Screen Recorder:** ffmpeg (bundle or require installation)
- **OCR:** tesseract OR cloud API key
- **Audio Switcher (Windows):** nircmd.exe (bundle in installers)
- **File Converter:** ffmpeg + optionally LibreOffice

**Plan ahead:** Decide to bundle or document as prerequisite.

---

## 🧪 TESTING STRATEGY

For each plugin, verify:

### Functional Tests
- [ ] Core feature works as expected
- [ ] Error handling graceful (no crashes)
- [ ] IPC commands respond correctly
- [ ] UI updates in real-time

### Integration Tests
- [ ] Doesn't break existing features (Text Expansion, VTT, Screenshot Core)
- [ ] Hotkeys don't conflict
- [ ] Multiple plugins run simultaneously
- [ ] Database writes don't block UI

### Cross-Platform Tests
- [ ] Multi-monitor support
- [ ] DPI scaling (150%, 200%)
- [ ] Negative monitor coordinates
- [ ] Works on Windows 10 + 11

### Performance Tests
- [ ] No memory leaks (30+ minute test)
- [ ] Fast response (< 200ms)
- [ ] Database queries optimized

---

## 📊 IMPLEMENTATION TIMELINE

Based on MASTER-PLAN-18-PLUGINS.md:

### Minimum Viable Product (v1.0)
**Target:** 8 plugins (TIER S + easy TIER A)
**Time:** ~120 hours (~6 weeks @ 20h/week)

1. QR Generator (4h)
2. Color Picker (6h)
3. Desk Pins (5h)
4. Audio Switcher (5h)
5. Sticky Notes (10h)
6. Screenshot Tool PLAN B (25h) 🔥
7. Clipboard History 2.0 (25h) 🔥
8. Image Resizer (8h)

### Extended Release (v1.1)
**Additional:** 5 plugins
**Time:** +70 hours

9. Screen Recorder (25h)
10. OCR (15h)
11. URL Shortener (10h)
12. File Converter (15h)
13. File Manager AI (12h)

### Advanced Release (v2.0)
**Additional:** 3 plugins
**Time:** +35 hours

14. Browser Extension (15h)
15. Web Clipper (5h)
16. Window Manager (20h) - optional

---

## 🤝 HANDOFF TO TERMINAL AGENT

### What You Have
- ✅ 5 shared modules (foundation)
- ✅ 11 plugin skeletons (most important features)
- ✅ Complete implementation guide
- ✅ Dependency list (Cargo.toml)
- ✅ Progress tracker (checklist)
- ✅ 3 React UI examples
- ✅ Strategic plan (MASTER-PLAN)

### What You Need to Do

1. **Read IMPLEMENTATION-GUIDE.md carefully** (main reference)
2. **Phase 0:** Setup foundation (shared modules, dependencies)
3. **Phase 1+:** Implement plugins one by one
4. **For each plugin:**
   - Copy skeleton
   - Implement TODOs
   - Add IPC commands
   - Create UI
   - Test thoroughly
   - ✅ Mark complete in checklist
5. **Production build** when v1.0 features complete

### When You're Stuck

1. Check IMPLEMENTATION-GUIDE.md for detailed steps
2. Look at existing working code:
   - `simple_expansion.rs` - for global listeners
   - `voice_to_text.rs` - for API integration
   - `screenshot_new.rs` - for coordinate math
3. Check integration notes in skeleton files (bottom of each .rs file)
4. Ask user for clarification

### Success Criteria

**v1.0 Release Checklist:**
- ✅ Screenshot Tool PLAN B works (pixel-perfect, multi-monitor)
- ✅ Clipboard History 2.0 works (Ctrl+Shift+V popup, search)
- ✅ At least 6 additional plugins working
- ✅ No regressions (Text Expansion, VTT still work)
- ✅ Production build completes
- ✅ Installers tested on clean Windows 11

---

## 🎉 SUMMARY

This package contains **everything needed** to implement Aplikacja 4.0 with 18 plugins. The foundation is solid, the architecture is proven, and the implementation path is clear.

**Key Success Factors:**
1. ✅ **Reuse existing code** (screenshot_new.rs, simple_expansion.rs)
2. ✅ **Follow priority order** (TIER S first, quick wins for momentum)
3. ✅ **Test after each plugin** (don't batch!)
4. ✅ **Ask when stuck** (don't spin for hours)

**Timeline:**
- v1.0 MVP: 6 weeks (8 plugins)
- v1.1 Extended: +3 weeks (13 plugins)
- v2.0 Advanced: +2 weeks (16 plugins)

**You've got this!** 🚀

---

**Prepared By:** Claude Code (Browser session)
**Date:** 2025-11-10
**For:** Aleks + Terminal Agent at home PC
**Next:** Start with IMPLEMENTATION-GUIDE.md Phase 0
