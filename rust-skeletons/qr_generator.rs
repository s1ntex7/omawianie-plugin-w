// ============================================================================
// QR CODE GENERATOR
// ============================================================================
// Priority: P1 (Quick Win - High Value, Low Complexity)
// Tech: qrcode crate
// Features: Generate QR code from text/URL, copy to clipboard, save to file
// Estimated time: 4 hours
// ============================================================================

use qrcode::QrCode;
use qrcode::render::svg;
use image::{ImageBuffer, Luma};
use crate::clipboard_manager::ClipboardManager;
use std::path::PathBuf;

/// QR Code generator
pub struct QrGenerator {
    clipboard_manager: ClipboardManager,
}

impl QrGenerator {
    pub fn new(clipboard_manager: ClipboardManager) -> Self {
        Self { clipboard_manager }
    }

    /// Generate QR code from text
    ///
    /// # Example
    /// ```
    /// let qr_gen = QrGenerator::new(clipboard_mgr);
    /// let image = qr_gen.generate("https://example.com")?;
    /// ```
    pub fn generate(&self, text: &str) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, String> {
        let code = QrCode::new(text.as_bytes())
            .map_err(|e| format!("Failed to create QR code: {}", e))?;

        let image = code.render::<Luma<u8>>().build();

        Ok(image)
    }

    /// Generate QR code and copy to clipboard
    ///
    /// Returns: Data URL (for display in UI)
    pub fn generate_and_copy(&self, text: &str) -> Result<String, String> {
        let img = self.generate(text)?;

        // Convert to RGBA for clipboard
        let (width, height) = img.dimensions();
        let rgba_data: Vec<u8> = img
            .pixels()
            .flat_map(|p| {
                let gray = p.0[0];
                [gray, gray, gray, 255] // Grayscale to RGBA
            })
            .collect();

        // Copy to clipboard
        self.clipboard_manager
            .paste_image(width as usize, height as usize, rgba_data)?;

        // Generate data URL for UI preview
        let data_url = self.image_to_data_url(&img)?;

        Ok(data_url)
    }

    /// Save QR code to file
    pub fn save_to_file(&self, text: &str, file_path: PathBuf) -> Result<(), String> {
        let img = self.generate(text)?;

        img.save(&file_path)
            .map_err(|e| format!("Failed to save QR code: {}", e))?;

        Ok(())
    }

    /// Generate SVG QR code (scalable, smaller file size)
    pub fn generate_svg(&self, text: &str) -> Result<String, String> {
        let code = QrCode::new(text.as_bytes())
            .map_err(|e| format!("Failed to create QR code: {}", e))?;

        let svg = code
            .render()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();

        Ok(svg)
    }

    /// Convert image to base64 data URL
    fn image_to_data_url(&self, img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Result<String, String> {
        use base64::{Engine as _, engine::general_purpose};

        let mut png_data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder
            .write_image(
                img.as_raw(),
                img.width(),
                img.height(),
                image::ExtendedColorType::L8,
            )
            .map_err(|e| format!("Failed to encode PNG: {}", e))?;

        let base64 = general_purpose::STANDARD.encode(&png_data);
        Ok(format!("data:image/png;base64,{}", base64))
    }
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This file should be placed in: src-tauri/src/qr_generator.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod qr_generator;
//    use qr_generator::QrGenerator;
//
//    let qr_gen = QrGenerator::new(clipboard_mgr.clone());
//    app.manage(qr_gen);
//    ```
//
// 3. Dependencies to add to Cargo.toml:
//    ```toml
//    qrcode = "0.14"
//    base64 = "0.22"
//    ```
//
// 4. IPC Commands:
//    ```rust
//    #[tauri::command]
//    fn qr_generate(text: String, state: tauri::State<QrGenerator>) -> Result<String, String> {
//        state.generate_and_copy(&text)
//    }
//
//    #[tauri::command]
//    fn qr_save(text: String, file_path: String, state: tauri::State<QrGenerator>) -> Result<(), String> {
//        state.save_to_file(&text, file_path.into())
//    }
//
//    #[tauri::command]
//    fn qr_generate_svg(text: String, state: tauri::State<QrGenerator>) -> Result<String, String> {
//        state.generate_svg(&text)
//    }
//    ```
//
// 5. React UI component (example):
//    ```tsx
//    const QrGeneratorPanel = () => {
//      const [text, setText] = useState("");
//      const [qrDataUrl, setQrDataUrl] = useState<string | null>(null);
//
//      const generate = async () => {
//        const dataUrl = await invoke<string>("qr_generate", { text });
//        setQrDataUrl(dataUrl);
//        toast.success("QR code copied to clipboard!");
//      };
//
//      return (
//        <div>
//          <input value={text} onChange={(e) => setText(e.target.value)} placeholder="Enter text or URL" />
//          <button onClick={generate}>Generate QR Code</button>
//          {qrDataUrl && <img src={qrDataUrl} alt="QR Code" />}
//        </div>
//      );
//    };
//    ```
//
// 6. Features to add:
//    - [ ] Input validation (URL format, length limits)
//    - [ ] Size customization (200x200, 500x500, 1000x1000)
//    - [ ] Color customization (dark/light colors)
//    - [ ] Error correction level (L, M, Q, H)
//    - [ ] History of generated QR codes
//    - [ ] Batch generation from CSV
//
// 7. Use cases:
//    - Share URLs quickly (WiFi passwords, links, etc.)
//    - Generate payment QR codes
//    - Create vCard QR codes for contact sharing
//    - Generate QR codes for marketing materials
//
// ============================================================================
