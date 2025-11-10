// ============================================================================
// OCR (Optical Character Recognition)
// ============================================================================
// Priority: P1 (Killer feature)
// Tech: screenshot_tool_egui + tesseract or cloud API
// Features: Copy text from images/screenshots
// Estimated time: 15 hours (Phase 4)
// ============================================================================

use crate::clipboard_manager::ClipboardManager;
use image::DynamicImage;

pub struct OcrEngine {
    clipboard_manager: ClipboardManager,
    use_cloud_api: bool,
}

impl OcrEngine {
    pub fn new(clipboard_manager: ClipboardManager) -> Self {
        Self {
            clipboard_manager,
            use_cloud_api: false,
        }
    }

    /// Extract text from image and copy to clipboard
    pub fn extract_and_copy(&self, image: &DynamicImage) -> Result<String, String> {
        let text = self.extract_text(image)?;
        self.clipboard_manager.paste_text(&text)?;
        Ok(text)
    }

    /// Extract text from image
    fn extract_text(&self, image: &DynamicImage) -> Result<String, String> {
        if self.use_cloud_api {
            self.extract_text_cloud(image)
        } else {
            self.extract_text_tesseract(image)
        }
    }

    /// Extract using local Tesseract
    fn extract_text_tesseract(&self, image: &DynamicImage) -> Result<String, String> {
        // TODO: Use tesseract crate or call tesseract CLI
        //
        // Option 1: tesseract crate
        // ```rust
        // use tesseract::Tesseract;
        // let text = Tesseract::new()
        //     .image(image)
        //     .lang("eng")
        //     .run()?;
        // ```
        //
        // Option 2: CLI (requires tesseract installed)
        // ```bash
        // tesseract input.png output -l eng
        // ```
        //
        // IMPORTANT: Tesseract must be installed!
        // - Windows: Bundle with installer
        // - macOS: brew install tesseract
        // - Linux: apt install tesseract-ocr

        Err("Not implemented - requires tesseract".to_string())
    }

    /// Extract using cloud API (fallback for better accuracy)
    fn extract_text_cloud(&self, image: &DynamicImage) -> Result<String, String> {
        // TODO: Use cloud OCR API
        //
        // Options:
        // 1. Azure Computer Vision API
        // 2. Google Cloud Vision API
        // 3. AWS Textract
        //
        // Pros: Better accuracy, handles handwriting
        // Cons: Requires API key, network dependency, costs money

        Err("Not implemented - requires API key".to_string())
    }
}

// ============================================================================
// INTEGRATION: Integrate with screenshot_tool_egui.rs
// Add "OCR" button to annotation toolbar (Phase 3/4)
// ============================================================================
