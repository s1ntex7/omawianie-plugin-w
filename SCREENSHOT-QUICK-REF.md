# 🎯 SCREENSHOT TOOL - QUICK REFERENCE

**Status:** PLAN B - Native Rust (egui + eframe)
**Date:** 2025-11-06
**Full Details:** See `SCREENSHOT-MASTER-PLAN.md`

---

## 📋 TLDR

After 15h debugging, JavaScript/Canvas approach (PLAN A) **fundamentally fails** on multi-monitor setups due to Tauri WebView window transform bug. Switching to **native Rust rendering with egui**.

---

## ✅ APPROVED ARCHITECTURE

```
F8 pressed → Spawn separate egui process → Native transparent overlay
→ Capture screens → Select region → Annotate → Ctrl+C saves → Done
```

**Tech Stack:**
- `egui` + `eframe` (immediate-mode GUI)
- `screenshots` crate (multi-monitor capture - already works)
- `image` crate (PNG encoding - already used)
- `rdev` (global keyboard - already used for text expansion)

---

## 🏗️ PHASES

| Phase | Goal | Time | Priority |
|-------|------|------|----------|
| **0** | POC: Verify egui works without transform bug | 3h | P0 🔥 |
| **1** | Screen capture + region selection | 4-6h | P0 |
| **2** | Save to folder + clipboard + `/ss` expansion | 2-3h | P0 |
| **3** | Annotations (arrows, text, blur) | 6-8h | P1 |
| **4** | Polish, settings, edge cases, testing | 8-12h | P1 |

**Total:** 23-32 hours (realistic: 4 weeks part-time)

---

## 🎯 MVP SCOPE (Must-Have for v1.0)

- ✅ F8 → instant overlay (< 200ms)
- ✅ Pixel-perfect selection (multi-monitor, DPI-aware)
- ✅ Ctrl+C saves as `DD-MM-YYYY-HH-MM-SS.png` to user folder
- ✅ Copy to clipboard
- ✅ `/ss` expansion pastes path to last screenshot
- ✅ ESC cancels cleanly
- ✅ Basic annotations: arrows (auto-numbered), text (wrapping), blur
- ✅ Existing features (Text Expansion, VTT) still work

---

## ⚡ NEXT STEP

**START HERE:** Phase 0 Proof of Concept

1. Create `src-tauri/bin/overlay_egui.rs`
2. Add dependencies to `Cargo.toml`:
   ```toml
   egui = "0.29.1"
   eframe = "0.29.1"
   ```
3. Test: `cargo run --bin overlay_egui`
4. **VERIFY:** No window transform bug on multi-monitor

**IF Phase 0 works → Continue to Phase 1**
**IF Phase 0 fails → Escalate to PLAN C (Hybrid)**

---

## 🔄 FALLBACK PLANS

- **PLAN C:** Hybrid (Rust capture + React annotations in decorated window)
- **PLAN D:** External tool integration (ShareX/Greenshot)

---

## 📚 KEY FILES

- `SCREENSHOT-MASTER-PLAN.md` - 11,000 word comprehensive guide
- `STATUS-2025-11-06-PLAN-B-READY.md` - Today's session summary
- `SEGMENT-APLIKACJA-3.0.md` - Overall project status

---

## 🎨 FEATURE HIGHLIGHTS

**Arrow Auto-Numbering:**
- First arrow → "1."
- Second arrow → "2."
- Delete #2 → keep gaps (1, 3, 4...) - less confusing

**Text Box Wrapping:**
- Click + drag to define rectangle
- Type text inside
- Auto-wraps at boundaries
- Narrow box (< 100px) → reduce font size

**Coordinate Systems:**
- Virtual desktop: supports negative coordinates (monitor left of primary)
- DPI scaling: logical pixels (mouse) vs physical pixels (image)
- Conversion: `physical_x = (virtual_x - monitor_x) * scale_factor`

---

**Last Updated:** 2025-11-06
**Quick Access:** This file lives in Segment 6 for fast reference
**Full Details:** Read `SCREENSHOT-MASTER-PLAN.md` before starting implementation
