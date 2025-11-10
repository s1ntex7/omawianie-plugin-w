// ============================================================================
// SCREEN RECORDER (GIF/MP4)
// ============================================================================
// Priority: P1 (Advanced feature)
// Tech: overlay_manager (for selection) + ffmpeg (for recording)
// Features: Record screen region to GIF or MP4, optional audio
// Estimated time: 25 hours (Phase 4)
// ============================================================================

use crate::overlay_manager::{OverlayManager, OverlayResult, ScreenRegion};
use std::path::PathBuf;
use std::process::{Command, Child};

/// Recording format
#[derive(Debug, Clone)]
pub enum RecordingFormat {
    Gif { fps: u32 },
    Mp4 { fps: u32, codec: String, bitrate: String },
}

/// Screen recorder
pub struct ScreenRecorder {
    overlay_manager: OverlayManager,
    recording_process: Option<Child>,
    output_folder: PathBuf,
}

impl ScreenRecorder {
    pub fn new(output_folder: PathBuf) -> Self {
        Self {
            overlay_manager: OverlayManager::new(),
            recording_process: None,
            output_folder,
        }
    }

    /// Start recording (select region first)
    pub fn start_recording(&mut self, format: RecordingFormat) -> Result<(), String> {
        // Step 1: Show overlay to select region
        let result = self.overlay_manager.spawn_region_selector()?;

        match result {
            OverlayResult::RegionSelected(region) => {
                // Step 2: Start ffmpeg recording process
                self.start_ffmpeg_recording(&region, format)?;
                Ok(())
            }
            OverlayResult::Cancelled => Err("Recording cancelled".to_string()),
            _ => Err("Unexpected overlay result".to_string()),
        }
    }

    /// Stop recording
    pub fn stop_recording(&mut self) -> Result<PathBuf, String> {
        if let Some(mut process) = self.recording_process.take() {
            // Send quit signal to ffmpeg (q key)
            // TODO: Implement graceful termination
            process.kill()
                .map_err(|e| format!("Failed to stop recording: {}", e))?;

            // Wait for process to finish
            process.wait()
                .map_err(|e| format!("Failed to wait for process: {}", e))?;

            // Return output path
            let output_path = self.output_folder.join("recording.mp4");
            Ok(output_path)
        } else {
            Err("No recording in progress".to_string())
        }
    }

    /// Start ffmpeg recording process
    fn start_ffmpeg_recording(&mut self, region: &ScreenRegion, format: RecordingFormat) -> Result<(), String> {
        // TODO: Implement ffmpeg integration
        //
        // GIF recording:
        // ```bash
        // ffmpeg -f gdigrab -framerate 15 -offset_x {x} -offset_y {y} -video_size {w}x{h}
        //        -i desktop -vf "fps=15,scale=320:-1:flags=lanczos"
        //        output.gif
        // ```
        //
        // MP4 recording:
        // ```bash
        // ffmpeg -f gdigrab -framerate 30 -offset_x {x} -offset_y {y} -video_size {w}x{h}
        //        -i desktop -c:v libx264 -preset ultrafast -crf 22
        //        output.mp4
        // ```
        //
        // With audio (optional):
        // Add: -f dshow -i audio="Microphone"
        //
        // Cross-platform capture:
        // - Windows: -f gdigrab
        // - macOS: -f avfoundation
        // - Linux: -f x11grab
        //
        // IMPORTANT: ffmpeg must be installed or bundled!

        Err("Not implemented - requires ffmpeg".to_string())
    }
}

// ============================================================================
// INTEGRATION: See screenshot_tool_egui.rs for overlay patterns
// See file_processor.rs for ffmpeg integration
// ============================================================================
