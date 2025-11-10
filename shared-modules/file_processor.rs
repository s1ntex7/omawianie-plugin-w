// ============================================================================
// FILE PROCESSOR - Shared Module for Aplikacja 3.0/4.0
// ============================================================================
// Used by: File Converter, Image Resizer, File Manager AI
// Purpose: Batch file operations, format conversion, filesystem watching
// Tech: std::fs, image crate, ffmpeg integration
// ============================================================================

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

/// File processing job
#[derive(Debug, Clone)]
pub struct ProcessingJob {
    pub id: u64,
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub operation: FileOperation,
    pub status: JobStatus,
}

/// File operations
#[derive(Debug, Clone)]
pub enum FileOperation {
    /// Convert image format
    ImageConvert {
        target_format: String, // "png", "jpg", "webp", etc.
        quality: u8,           // 0-100
    },

    /// Resize image
    ImageResize {
        width: Option<u32>,
        height: Option<u32>,
        preserve_aspect: bool,
    },

    /// Convert video/audio format
    MediaConvert {
        target_format: String, // "mp4", "mp3", "gif", etc.
        codec: Option<String>,
        bitrate: Option<String>,
    },

    /// Convert document
    DocumentConvert {
        target_format: String, // "pdf", "docx", etc.
    },

    /// Custom command
    Custom {
        command: String,
        args: Vec<String>,
    },
}

/// Job status
#[derive(Debug, Clone)]
pub enum JobStatus {
    Pending,
    Running,
    Completed { duration_ms: u64 },
    Failed { error: String },
}

/// File processor with batch queue
pub struct FileProcessor {
    jobs: Vec<ProcessingJob>,
    next_id: u64,
}

impl FileProcessor {
    /// Create new file processor
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            next_id: 0,
        }
    }

    /// Add job to queue
    pub fn add_job(&mut self, input: PathBuf, output: PathBuf, operation: FileOperation) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let job = ProcessingJob {
            id,
            input_path: input,
            output_path: output,
            operation,
            status: JobStatus::Pending,
        };

        self.jobs.push(job);
        id
    }

    /// Process all jobs in queue
    pub fn process_all(&mut self) -> Result<(), String> {
        for job in &mut self.jobs {
            if matches!(job.status, JobStatus::Pending) {
                job.status = JobStatus::Running;
                let result = self.process_job(job);
                job.status = result;
            }
        }
        Ok(())
    }

    /// Process single job
    fn process_job(&self, job: &ProcessingJob) -> JobStatus {
        let start = std::time::Instant::now();

        let result = match &job.operation {
            FileOperation::ImageConvert { target_format, quality } => {
                self.convert_image(&job.input_path, &job.output_path, target_format, *quality)
            }
            FileOperation::ImageResize {
                width,
                height,
                preserve_aspect,
            } => self.resize_image(&job.input_path, &job.output_path, *width, *height, *preserve_aspect),
            FileOperation::MediaConvert {
                target_format,
                codec,
                bitrate,
            } => self.convert_media(
                &job.input_path,
                &job.output_path,
                target_format,
                codec.as_deref(),
                bitrate.as_deref(),
            ),
            FileOperation::DocumentConvert { target_format } => {
                self.convert_document(&job.input_path, &job.output_path, target_format)
            }
            FileOperation::Custom { command, args } => self.execute_custom(command, args),
        };

        match result {
            Ok(_) => JobStatus::Completed {
                duration_ms: start.elapsed().as_millis() as u64,
            },
            Err(e) => JobStatus::Failed { error: e },
        }
    }

    /// Convert image format using `image` crate
    fn convert_image(
        &self,
        input: &Path,
        output: &Path,
        _format: &str,
        _quality: u8,
    ) -> Result<(), String> {
        // TODO: Implement image conversion
        //
        // Steps:
        // 1. Load image: image::open(input)?
        // 2. Convert format: img.save_with_format(output, format)?
        // 3. Handle quality for JPEG/WebP
        //
        // Dependencies:
        // - image = "0.25"
        // - image-webp = "0.1" (for WebP support)

        Err("Not implemented - use image crate".to_string())
    }

    /// Resize image
    fn resize_image(
        &self,
        input: &Path,
        output: &Path,
        width: Option<u32>,
        height: Option<u32>,
        preserve_aspect: bool,
    ) -> Result<(), String> {
        // TODO: Implement image resizing
        //
        // Steps:
        // 1. Load image: image::open(input)?
        // 2. Calculate dimensions (preserve_aspect logic)
        // 3. Resize: img.resize(w, h, FilterType::Lanczos3)
        // 4. Save: resized.save(output)?
        //
        // Example:
        // ```
        // let img = image::open(input)?;
        // let (w, h) = if preserve_aspect {
        //     let aspect = img.width() as f32 / img.height() as f32;
        //     if let Some(target_w) = width {
        //         (target_w, (target_w as f32 / aspect) as u32)
        //     } else if let Some(target_h) = height {
        //         ((target_h as f32 * aspect) as u32, target_h)
        //     } else {
        //         (img.width(), img.height())
        //     }
        // } else {
        //     (width.unwrap_or(img.width()), height.unwrap_or(img.height()))
        // };
        // let resized = img.resize(w, h, image::imageops::FilterType::Lanczos3);
        // resized.save(output)?;
        // ```

        Err("Not implemented - use image crate".to_string())
    }

    /// Convert media using ffmpeg
    fn convert_media(
        &self,
        input: &Path,
        output: &Path,
        format: &str,
        codec: Option<&str>,
        bitrate: Option<&str>,
    ) -> Result<(), String> {
        // TODO: Implement ffmpeg integration
        //
        // Steps:
        // 1. Build ffmpeg command:
        //    ffmpeg -i input.mp4 -c:v libx264 -b:v 2M output.mp4
        // 2. Execute: Command::new("ffmpeg").args(...).output()?
        // 3. Check exit status
        //
        // IMPORTANT: ffmpeg must be installed on user's system!
        // - Windows: Bundled in installers or via PATH
        // - macOS: brew install ffmpeg
        // - Linux: apt install ffmpeg
        //
        // For GIF conversion (Screen Recorder plugin):
        // ffmpeg -i input.mp4 -vf "fps=15,scale=320:-1:flags=lanczos" output.gif
        //
        // For MP4 with H.264:
        // ffmpeg -i input.avi -c:v libx264 -preset fast -crf 22 output.mp4

        Err("Not implemented - requires ffmpeg binary".to_string())
    }

    /// Convert document
    fn convert_document(&self, input: &Path, output: &Path, format: &str) -> Result<(), String> {
        // TODO: Implement document conversion
        //
        // Options:
        // 1. LibreOffice CLI:
        //    soffice --headless --convert-to pdf input.docx --outdir /output
        // 2. pandoc (for markdown/HTML):
        //    pandoc input.md -o output.pdf
        // 3. Cloud API (Cloudmersive, etc.)
        //
        // This is the most complex converter (many formats)
        // Consider shipping v1.0 without this, or with limited support

        Err("Not implemented - consider external tool or API".to_string())
    }

    /// Execute custom command
    fn execute_custom(&self, command: &str, args: &[String]) -> Result<(), String> {
        let output = Command::new(command)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute {}: {}", command, e))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Command failed: {}", stderr))
        }
    }

    /// Get all jobs
    pub fn get_jobs(&self) -> &[ProcessingJob] {
        &self.jobs
    }

    /// Get job by ID
    pub fn get_job(&self, id: u64) -> Option<&ProcessingJob> {
        self.jobs.iter().find(|j| j.id == id)
    }

    /// Clear completed jobs
    pub fn clear_completed(&mut self) {
        self.jobs
            .retain(|j| !matches!(j.status, JobStatus::Completed { .. }));
    }
}

impl Default for FileProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Filesystem watcher for File Manager AI
pub struct FilesystemWatcher {
    watch_path: PathBuf,
    sender: Sender<PathBuf>,
}

impl FilesystemWatcher {
    /// Start watching a folder for new files
    ///
    /// # Example
    /// ```
    /// let (tx, rx) = std::sync::mpsc::channel();
    /// let watcher = FilesystemWatcher::watch("/Users/alex/Downloads", tx)?;
    ///
    /// // In background thread:
    /// while let Ok(new_file) = rx.recv() {
    ///     println!("New file detected: {:?}", new_file);
    ///     categorize_with_ai(&new_file)?;
    /// }
    /// ```
    pub fn watch(path: PathBuf, sender: Sender<PathBuf>) -> Result<Self, String> {
        // TODO: Implement filesystem watching
        //
        // Options:
        // 1. notify crate (cross-platform):
        //    use notify::{Watcher, RecursiveMode, watcher};
        //    let mut watcher = watcher(tx, Duration::from_secs(2))?;
        //    watcher.watch(path, RecursiveMode::NonRecursive)?;
        //
        // 2. Manual polling (simpler, less efficient):
        //    - Read directory every N seconds
        //    - Track known files in HashSet
        //    - Send events for new files
        //
        // For File Manager AI plugin, we want:
        // - Watch "Downloads" folder
        // - Detect new files (Created event)
        // - Send to AI for categorization
        // - Move to appropriate folder

        Err("Not implemented - use notify crate".to_string())
    }

    /// Stop watching
    pub fn stop(self) {
        // TODO: Cleanup watcher
    }
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This module should be placed in: src-tauri/src/file_processor.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod file_processor;
//    use file_processor::FileProcessor;
//
//    let processor = FileProcessor::new();
//    app.manage(processor);
//    ```
//
// 3. Dependencies to add to Cargo.toml:
//    ```toml
//    image = "0.25"
//    notify = "6.1"  # For filesystem watching
//    ```
//
// 4. External dependencies (user must install):
//    - ffmpeg: For media conversion
//    - LibreOffice: For document conversion (optional)
//
// 5. Usage in plugins:
//    - File Converter: Add job, process with appropriate operation
//    - Image Resizer: Batch resize with drag-drop
//    - File Manager AI: Watch Downloads, categorize with Groq API
//
// 6. IPC Commands:
//    ```rust
//    #[tauri::command]
//    fn file_convert(
//        input: String,
//        output: String,
//        format: String,
//        state: tauri::State<Mutex<FileProcessor>>
//    ) -> Result<u64, String> {
//        let mut processor = state.lock().unwrap();
//        let op = FileOperation::ImageConvert { target_format: format, quality: 90 };
//        Ok(processor.add_job(input.into(), output.into(), op))
//    }
//
//    #[tauri::command]
//    fn file_process_queue(state: tauri::State<Mutex<FileProcessor>>) -> Result<(), String> {
//        let mut processor = state.lock().unwrap();
//        processor.process_all()
//    }
//    ```
//
// 7. Error handling:
//    - Check if ffmpeg is available before media conversion
//    - Gracefully fail if external tool not found
//    - Show user-friendly error messages
//
// ============================================================================
