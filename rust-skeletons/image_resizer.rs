// ============================================================================
// QUICK IMAGE RESIZER
// ============================================================================
// Priority: P1
// Tech: image crate + file_processor (batch operations)
// Features: Drag & drop multiple images, batch resize, quality adjustment
// Estimated time: 8 hours
// ============================================================================

use crate::file_processor::{FileProcessor, FileOperation};
use std::path::PathBuf;
use image::ImageFormat;

/// Image resize preset
#[derive(Debug, Clone)]
pub struct ResizePreset {
    pub name: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub preserve_aspect: bool,
    pub quality: u8, // 0-100
}

impl ResizePreset {
    pub fn thumbnail() -> Self {
        Self {
            name: "Thumbnail".to_string(),
            width: Some(150),
            height: Some(150),
            preserve_aspect: true,
            quality: 85,
        }
    }

    pub fn web_small() -> Self {
        Self {
            name: "Web Small".to_string(),
            width: Some(800),
            height: None,
            preserve_aspect: true,
            quality: 80,
        }
    }

    pub fn web_medium() -> Self {
        Self {
            name: "Web Medium".to_string(),
            width: Some(1280),
            height: None,
            preserve_aspect: true,
            quality: 85,
        }
    }

    pub fn web_large() -> Self {
        Self {
            name: "Web Large".to_string(),
            width: Some(1920),
            height: None,
            preserve_aspect: true,
            quality: 90,
        }
    }
}

/// Image resizer
pub struct ImageResizer {
    processor: FileProcessor,
    output_folder: PathBuf,
    presets: Vec<ResizePreset>,
}

impl ImageResizer {
    pub fn new(output_folder: PathBuf) -> Self {
        Self {
            processor: FileProcessor::new(),
            output_folder,
            presets: vec![
                ResizePreset::thumbnail(),
                ResizePreset::web_small(),
                ResizePreset::web_medium(),
                ResizePreset::web_large(),
            ],
        }
    }

    /// Resize multiple images with preset
    pub fn batch_resize(&mut self, input_paths: Vec<PathBuf>, preset: &ResizePreset) -> Result<Vec<PathBuf>, String> {
        let mut output_paths = Vec::new();

        for input_path in input_paths {
            // Generate output filename
            let filename = input_path
                .file_stem()
                .ok_or("Invalid filename")?
                .to_string_lossy()
                .to_string();

            let extension = input_path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("jpg");

            let output_filename = format!("{}_{}.{}", filename, preset.name, extension);
            let output_path = self.output_folder.join(output_filename);

            // Add to processor queue
            let operation = FileOperation::ImageResize {
                width: preset.width,
                height: preset.height,
                preserve_aspect: preset.preserve_aspect,
            };

            self.processor.add_job(input_path, output_path.clone(), operation);
            output_paths.push(output_path);
        }

        // Process all jobs
        self.processor.process_all()?;

        Ok(output_paths)
    }

    /// Resize with custom dimensions
    pub fn resize_custom(
        &mut self,
        input_paths: Vec<PathBuf>,
        width: Option<u32>,
        height: Option<u32>,
        preserve_aspect: bool,
        quality: u8,
    ) -> Result<Vec<PathBuf>, String> {
        let custom_preset = ResizePreset {
            name: "Custom".to_string(),
            width,
            height,
            preserve_aspect,
            quality,
        };

        self.batch_resize(input_paths, &custom_preset)
    }

    /// Get available presets
    pub fn get_presets(&self) -> &[ResizePreset] {
        &self.presets
    }

    /// Add custom preset
    pub fn add_preset(&mut self, preset: ResizePreset) {
        self.presets.push(preset);
    }

    /// Set output folder
    pub fn set_output_folder(&mut self, folder: PathBuf) -> Result<(), String> {
        if !folder.exists() {
            std::fs::create_dir_all(&folder)
                .map_err(|e| format!("Failed to create output folder: {}", e))?;
        }

        self.output_folder = folder;
        Ok(())
    }
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This file should be placed in: src-tauri/src/image_resizer.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod image_resizer;
//    use image_resizer::ImageResizer;
//
//    let output_folder = app.path().app_data_dir()?.join("Resized");
//    let resizer = ImageResizer::new(output_folder);
//    app.manage(Mutex::new(resizer));
//    ```
//
// 3. Dependencies (already in project via file_processor):
//    ```toml
//    image = "0.25"
//    ```
//
// 4. IPC Commands:
//    ```rust
//    #[tauri::command]
//    fn image_resize_batch(
//        input_paths: Vec<String>,
//        preset_name: String,
//        state: tauri::State<Mutex<ImageResizer>>
//    ) -> Result<Vec<String>, String> {
//        let mut resizer = state.lock().unwrap();
//        let preset = resizer.get_presets().iter()
//            .find(|p| p.name == preset_name)
//            .ok_or("Preset not found")?
//            .clone();
//        let inputs: Vec<PathBuf> = input_paths.into_iter().map(PathBuf::from).collect();
//        let outputs = resizer.batch_resize(inputs, &preset)?;
//        Ok(outputs.into_iter().map(|p| p.to_string_lossy().to_string()).collect())
//    }
//    ```
//
// 5. React Component (Drag & Drop):
//    ```tsx
//    const ImageResizerPanel = () => {
//      const [files, setFiles] = useState<File[]>([]);
//      const [preset, setPreset] = useState("Web Medium");
//
//      const onDrop = (e: DragEvent) => {
//        e.preventDefault();
//        const droppedFiles = Array.from(e.dataTransfer.files).filter(
//          f => f.type.startsWith("image/")
//        );
//        setFiles(droppedFiles);
//      };
//
//      const resize = async () => {
//        const paths = files.map(f => f.path); // Electron/Tauri exposes file paths
//        const outputs = await invoke<string[]>("image_resize_batch", {
//          inputPaths: paths,
//          presetName: preset
//        });
//        toast.success(`Resized ${outputs.length} images!`);
//      };
//
//      return (
//        <div onDrop={onDrop} onDragOver={(e) => e.preventDefault()}>
//          <p>Drag & drop images here</p>
//          <select value={preset} onChange={(e) => setPreset(e.target.value)}>
//            <option>Thumbnail</option>
//            <option>Web Small</option>
//            <option>Web Medium</option>
//            <option>Web Large</option>
//          </select>
//          <button onClick={resize}>Resize {files.length} images</button>
//        </div>
//      );
//    };
//    ```
//
// 6. Testing checklist:
//    - [ ] Batch resize works for multiple images
//    - [ ] Aspect ratio preserved when enabled
//    - [ ] Quality setting applied correctly
//    - [ ] Output folder created if missing
//    - [ ] Works with various formats (JPG, PNG, WebP, GIF)
//
// ============================================================================
