// ============================================================================
// QUICK AUDIO SWITCHER
// ============================================================================
// Priority: P1 (Niche but useful)
// Tech: Windows Audio API (wasapi), macOS CoreAudio, Linux PulseAudio
// Features: System tray icon, one-click switch between audio devices
// Estimated time: 5 hours
// ============================================================================

#[cfg(target_os = "windows")]
use windows::Win32::Media::Audio::{
    IMMDeviceEnumerator, MMDeviceEnumerator, EDataFlow, eRender, eConsole, DEVICE_STATE_ACTIVE,
};
#[cfg(target_os = "windows")]
use windows::core::ComInterface;

use serde::{Serialize, Deserialize};

/// Audio device info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub device_type: AudioDeviceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioDeviceType {
    Output, // Speakers, headphones
    Input,  // Microphone
}

/// Audio switcher
pub struct AudioSwitcher {
    current_device: Option<String>,
}

impl AudioSwitcher {
    pub fn new() -> Self {
        Self {
            current_device: None,
        }
    }

    /// List all audio output devices
    pub fn list_output_devices(&self) -> Result<Vec<AudioDevice>, String> {
        #[cfg(target_os = "windows")]
        {
            self.list_devices_windows(AudioDeviceType::Output)
        }

        #[cfg(target_os = "macos")]
        {
            self.list_devices_macos(AudioDeviceType::Output)
        }

        #[cfg(target_os = "linux")]
        {
            self.list_devices_linux(AudioDeviceType::Output)
        }
    }

    /// Switch to specific audio device
    pub fn switch_to_device(&mut self, device_id: &str) -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            self.switch_device_windows(device_id)?;
        }

        #[cfg(not(target_os = "windows"))]
        {
            return Err("Not implemented for this platform".to_string());
        }

        self.current_device = Some(device_id.to_string());
        Ok(())
    }

    /// Get current default device
    pub fn get_default_device(&self) -> Result<AudioDevice, String> {
        #[cfg(target_os = "windows")]
        {
            self.get_default_device_windows()
        }

        #[cfg(not(target_os = "windows"))]
        {
            Err("Not implemented for this platform".to_string())
        }
    }

    // ============================================================================
    // WINDOWS IMPLEMENTATION
    // ============================================================================

    #[cfg(target_os = "windows")]
    fn list_devices_windows(&self, device_type: AudioDeviceType) -> Result<Vec<AudioDevice>, String> {
        use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

        unsafe {
            // Initialize COM
            CoInitializeEx(None, COINIT_MULTITHREADED)
                .map_err(|e| format!("Failed to initialize COM: {}", e))?;

            // Create device enumerator
            let enumerator: IMMDeviceEnumerator = windows::core::ComObject::create(&MMDeviceEnumerator)
                .map_err(|e| format!("Failed to create enumerator: {}", e))?;

            // Get device collection
            let collection = enumerator
                .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)
                .map_err(|e| format!("Failed to enumerate devices: {}", e))?;

            let count = collection.GetCount()
                .map_err(|e| format!("Failed to get device count: {}", e))?;

            let mut devices = Vec::new();

            for i in 0..count {
                // TODO: Get device info
                // - ID: device.GetId()
                // - Name: property_store.GetValue(PKEY_Device_FriendlyName)
                // - Is default: Compare with GetDefaultAudioEndpoint()
            }

            CoUninitialize();

            Ok(devices)
        }
    }

    #[cfg(target_os = "windows")]
    fn switch_device_windows(&self, device_id: &str) -> Result<(), String> {
        // TODO: Set default audio device
        //
        // Windows doesn't have a built-in API for this!
        // Options:
        // 1. Use PolicyConfig COM interface (undocumented)
        // 2. Use external tool like nircmd.exe
        // 3. Use Registry manipulation (unreliable)
        // 4. Use AudioDeviceCmdlets PowerShell module
        //
        // Recommended: Bundle nircmd.exe and call:
        // nircmd.exe setdefaultsounddevice "{device_id}"

        Err("Requires nircmd.exe or PolicyConfig".to_string())
    }

    #[cfg(target_os = "windows")]
    fn get_default_device_windows(&self) -> Result<AudioDevice, String> {
        // TODO: Get default device using IMMDeviceEnumerator::GetDefaultAudioEndpoint()
        Err("Not implemented".to_string())
    }

    // ============================================================================
    // MACOS IMPLEMENTATION
    // ============================================================================

    #[cfg(target_os = "macos")]
    fn list_devices_macos(&self, device_type: AudioDeviceType) -> Result<Vec<AudioDevice>, String> {
        // TODO: Use CoreAudio framework
        //
        // Steps:
        // 1. Get device list using AudioObjectGetPropertyData
        // 2. Query kAudioHardwarePropertyDevices
        // 3. Filter by kAudioDevicePropertyStreamFormat
        //
        // Dependencies:
        // - coreaudio-rs = "0.11"

        Err("macOS support not implemented".to_string())
    }

    // ============================================================================
    // LINUX IMPLEMENTATION
    // ============================================================================

    #[cfg(target_os = "linux")]
    fn list_devices_linux(&self, device_type: AudioDeviceType) -> Result<Vec<AudioDevice>, String> {
        // TODO: Use PulseAudio API
        //
        // Steps:
        // 1. Connect to PulseAudio daemon
        // 2. Query sinks (output devices) using pa_context_get_sink_info_list
        // 3. Query sources (input devices) for microphones
        //
        // Dependencies:
        // - libpulse-binding = "2.28"
        //
        // Alternative: Parse `pactl list sinks` command output

        Err("Linux support not implemented".to_string())
    }
}

impl Default for AudioSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SYSTEM TRAY INTEGRATION
// ============================================================================

/// System tray icon with menu for audio switching
///
/// Example:
/// - Icon shows current audio device (speaker/headphone icon)
/// - Click to show menu with all devices
/// - Click device to switch
pub struct AudioSwitcherTray {
    switcher: AudioSwitcher,
}

impl AudioSwitcherTray {
    pub fn new() -> Self {
        Self {
            switcher: AudioSwitcher::new(),
        }
    }

    /// Initialize system tray icon
    pub fn init_tray(&self) -> Result<(), String> {
        // TODO: Use tray-icon crate to create system tray
        //
        // Dependencies:
        // - tray-icon = "0.14"
        //
        // Steps:
        // 1. Create tray icon
        // 2. Build menu with device list
        // 3. Handle menu item clicks
        // 4. Update icon on device switch

        Err("System tray not implemented".to_string())
    }
}

// ============================================================================
// INTEGRATION NOTES FOR TERMINAL AGENT:
// ============================================================================
//
// 1. This file should be placed in: src-tauri/src/audio_switcher.rs
//
// 2. Add to src-tauri/src/main.rs:
//    ```rust
//    mod audio_switcher;
//    use audio_switcher::{AudioSwitcher, AudioSwitcherTray};
//
//    let audio_switcher = AudioSwitcher::new();
//    app.manage(audio_switcher);
//
//    // Optional: Initialize system tray
//    // let tray = AudioSwitcherTray::new();
//    // tray.init_tray()?;
//    ```
//
// 3. Dependencies to add to Cargo.toml:
//    ```toml
//    [target.'cfg(windows)'.dependencies]
//    windows = { version = "0.58", features = [
//        "Win32_Media_Audio",
//        "Win32_System_Com",
//    ]}
//
//    [target.'cfg(target_os = "macos")'.dependencies]
//    coreaudio-rs = "0.11"
//
//    [target.'cfg(target_os = "linux")'.dependencies]
//    libpulse-binding = "2.28"
//
//    # System tray
//    tray-icon = "0.14"
//    ```
//
// 4. External dependencies (Windows):
//    - nircmd.exe (for switching default device)
//    - Bundle in installers: src-tauri/resources/nircmd.exe
//
// 5. IPC Commands:
//    ```rust
//    #[tauri::command]
//    fn audio_list_devices(state: tauri::State<AudioSwitcher>) -> Result<Vec<AudioDevice>, String> {
//        state.list_output_devices()
//    }
//
//    #[tauri::command]
//    fn audio_switch_device(device_id: String, state: tauri::State<Mutex<AudioSwitcher>>) -> Result<(), String> {
//        state.lock().unwrap().switch_to_device(&device_id)
//    }
//    ```
//
// 6. Testing checklist:
//    - [ ] List shows all audio devices
//    - [ ] Switch changes default device system-wide
//    - [ ] Current device is highlighted
//    - [ ] System tray icon updates on switch
//    - [ ] Works with USB devices (hot-plug)
//
// ============================================================================
