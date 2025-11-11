# Tauri v2 Integration Guide

This document shows how to integrate the plugin system with Tauri v2.

## Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│                 React Frontend                      │
│  (Configuration UI, Plugin Dashboard)               │
└────────────────┬────────────────────────────────────┘
                 │ Tauri Commands & Events
                 │
┌────────────────▼────────────────────────────────────┐
│              Tauri Backend (Rust)                   │
│  ┌──────────────────────────────────────────┐      │
│  │         Plugin Registry                  │      │
│  │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐│      │
│  │  │Echo  │  │Clip  │  │Text  │  │Shot  ││      │
│  │  │Plugin│  │Plugin│  │Expand│  │Plugin││      │
│  │  └───┬──┘  └───┬──┘  └───┬──┘  └───┬──┘│      │
│  │      │         │          │          │   │      │
│  │      └─────────┴──────────┴──────────┘   │      │
│  │                   │                       │      │
│  │        ┌──────────▼──────────┐           │      │
│  │        │   Core Services     │           │      │
│  │        │  • EventBus         │           │      │
│  │        │  • Clipboard        │           │      │
│  │        │  • Storage          │           │      │
│  │        │  • Notifications    │           │      │
│  │        └─────────────────────┘           │      │
│  └──────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────┘
         │                              │
         │ gRPC                         │ System APIs
         ▼                              ▼
┌──────────────────┐        ┌──────────────────────┐
│  Screenshot      │        │  OS (Clipboard,      │
│  Overlay (egui)  │        │  Filesystem, etc.)   │
└──────────────────┘        └──────────────────────┘
```

## Tauri Backend Setup

### 1. Main Application

```rust
// src-tauri/src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aplikacja_core::*;
use std::sync::Arc;
use tokio::sync::RwLock;

mod commands;
mod plugins;

use commands::*;
use plugins::*;

// Shared state
pub struct AppState {
    registry: Arc<RwLock<PluginRegistry>>,
    core: CoreServices,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Create core services
    let core = CoreServices::new().await;

    // Create and initialize plugin registry
    let mut registry = PluginRegistry::new();

    // Register all plugins
    register_plugins(&mut registry).await;

    // Initialize all plugins
    registry.initialize_all(core.clone()).await
        .expect("Failed to initialize plugins");

    let state = AppState {
        registry: Arc::new(RwLock::new(registry)),
        core: core.clone(),
    };

    // Build Tauri app
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            // Plugin commands
            commands::list_plugins,
            commands::plugin_execute,
            commands::plugin_enable,
            commands::plugin_disable,
            // Settings commands
            commands::get_settings,
            commands::update_settings,
        ])
        .setup(|app| {
            // Setup system tray
            setup_system_tray(app)?;

            // Setup global hotkeys
            setup_hotkeys(app)?;

            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                // Hide window instead of closing (keep running in tray)
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                // Cleanup on exit
                api.prevent_exit();
                tokio::spawn(async move {
                    shutdown_plugins(app_handle).await;
                    std::process::exit(0);
                });
            }
        });
}

async fn register_plugins(registry: &mut PluginRegistry) {
    // Core plugins
    registry.register(Box::new(ClipboardManagerPlugin::new())).unwrap();
    registry.register(Box::new(ScreenshotPlugin::new())).unwrap();
    registry.register(Box::new(TextExpansionPlugin::new())).unwrap();
    registry.register(Box::new(WindowTilingPlugin::new())).unwrap();

    // More plugins...
}

async fn shutdown_plugins(app_handle: &tauri::AppHandle) {
    if let Some(state) = app_handle.try_state::<AppState>() {
        let registry = state.registry.read().await;
        registry.shutdown_all().await.ok();
    }
}
```

### 2. Tauri Commands

```rust
// src-tauri/src/commands.rs

use tauri::State;
use serde::{Deserialize, Serialize};
use aplikacja_core::*;
use crate::AppState;

#[derive(Debug, Clone, Serialize)]
pub struct PluginListItem {
    pub id: String,
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub description: String,
}

/// List all registered plugins
#[tauri::command]
pub async fn list_plugins(state: State<'_, AppState>) -> Result<Vec<PluginListItem>, String> {
    let registry = state.registry.read().await;

    let plugins = registry.list_plugins().await
        .into_iter()
        .map(|info| PluginListItem {
            id: info.id,
            name: info.name,
            version: info.version,
            enabled: true, // TODO: Track enabled state
            description: "".into(), // TODO: Add description to PluginInfo
        })
        .collect();

    Ok(plugins)
}

/// Execute a plugin command
#[tauri::command]
pub async fn plugin_execute(
    state: State<'_, AppState>,
    plugin_id: String,
    command: String,
    args: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let registry = state.registry.read().await;

    registry.execute(&plugin_id, &command, args)
        .await
        .map_err(|e| e.to_string())
}

/// Enable a plugin
#[tauri::command]
pub async fn plugin_enable(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    plugin_id: String,
) -> Result<(), String> {
    // TODO: Implement enable logic

    // Emit event to frontend
    app.emit_all("plugin:enabled", plugin_id).ok();

    Ok(())
}

/// Disable a plugin
#[tauri::command]
pub async fn plugin_disable(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    plugin_id: String,
) -> Result<(), String> {
    // TODO: Implement disable logic

    // Emit event to frontend
    app.emit_all("plugin:disabled", plugin_id).ok();

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct SettingsUpdate {
    pub key: String,
    pub value: serde_json::Value,
}

/// Get all settings
#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let settings = state.core.settings.get_all().await;
    Ok(serde_json::to_value(settings).unwrap())
}

/// Update a setting
#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    update: SettingsUpdate,
) -> Result<(), String> {
    state.core.settings.set(&update.key, update.value.clone())
        .await
        .map_err(|e| e.to_string())?;

    // Emit event to all windows
    app.emit_all("settings:changed", &update).ok();

    Ok(())
}
```

### 3. Plugin with Tauri Integration

```rust
// plugins/screenshot/src/lib.rs

use aplikacja_core::*;
use async_trait::async_trait;
use serde_json::json;
use tauri::{AppHandle, Manager};

pub struct ScreenshotPlugin {
    info: PluginInfo,
    core: Option<CoreServices>,
    app_handle: Option<AppHandle>,
}

impl ScreenshotPlugin {
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                id: "screenshot".into(),
                name: "Screenshot Tool".into(),
                version: "1.0.0".into(),
            },
            core: None,
            app_handle: None,
        }
    }

    pub fn set_app_handle(&mut self, handle: AppHandle) {
        self.app_handle = Some(handle);
    }
}

#[async_trait]
impl Plugin for ScreenshotPlugin {
    fn info(&self) -> PluginInfo {
        self.info.clone()
    }

    async fn initialize(&mut self, core: CoreServices) -> anyhow::Result<()> {
        println!("✓ Screenshot plugin initialized");

        // Subscribe to hotkey events
        let mut events = core.events.subscribe();
        let app_handle = self.app_handle.clone();

        tokio::spawn(async move {
            while let Ok(event) = events.recv().await {
                if event == "hotkey:screenshot:triggered" {
                    // Trigger screenshot
                    if let Some(app) = &app_handle {
                        app.emit_all("screenshot:start", ()).ok();
                    }
                }
            }
        });

        self.core = Some(core);
        Ok(())
    }

    async fn execute(&self, command: &str, args: serde_json::Value) -> anyhow::Result<serde_json::Value> {
        match command {
            "capture" => {
                let capture_mode = args.get("mode")
                    .and_then(|v| v.as_str())
                    .unwrap_or("fullscreen");

                // Spawn overlay process for selection
                if capture_mode == "selection" {
                    std::process::Command::new("./screenshot_overlay")
                        .spawn()?;
                }

                // Emit event to frontend
                if let Some(app) = &self.app_handle {
                    app.emit_all("screenshot:capturing", capture_mode).ok();
                }

                Ok(json!({
                    "status": "capturing",
                    "mode": capture_mode
                }))
            }
            "save" => {
                let path = args.get("path")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Path required"))?;

                let image_data = args.get("data")
                    .ok_or_else(|| anyhow::anyhow!("Image data required"))?;

                // Save screenshot
                // ... save logic ...

                // Emit success event
                if let Some(app) = &self.app_handle {
                    app.emit_all("screenshot:saved", json!({ "path": path })).ok();
                }

                // Publish to event bus
                if let Some(core) = &self.core {
                    core.events.publish(format!("screenshot:saved:{}", path));
                }

                Ok(json!({
                    "status": "saved",
                    "path": path
                }))
            }
            _ => Err(anyhow::anyhow!("Unknown command: {}", command))
        }
    }
}
```

### 4. System Tray Setup

```rust
// src-tauri/src/main.rs (continued)

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};

fn setup_system_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("show".to_string(), "Show Window");
    let screenshot = CustomMenuItem::new("screenshot".to_string(), "Take Screenshot");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(screenshot)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    "screenshot" => {
                        // Trigger screenshot
                        app.emit_all("screenshot:start", ()).ok();
                    }
                    _ => {}
                }
            }
            _ => {}
        });

    Ok(())
}
```

### 5. Global Hotkeys

```rust
// src-tauri/src/main.rs (continued)

use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

fn setup_hotkeys(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();

    // Register Ctrl+Shift+S for screenshot
    app.global_shortcut().register("Ctrl+Shift+S", move || {
        app_handle.emit_all("hotkey:screenshot:triggered", ()).ok();
    })?;

    // Register Ctrl+Shift+V for clipboard history
    let app_handle2 = app.handle();
    app.global_shortcut().register("Ctrl+Shift+V", move || {
        app_handle2.emit_all("hotkey:clipboard:triggered", ()).ok();
    })?;

    Ok(())
}
```

## Frontend Integration (React)

### 1. Plugin List Component

```typescript
// src/components/PluginList.tsx

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

interface Plugin {
  id: string;
  name: string;
  version: string;
  enabled: boolean;
  description: string;
}

export function PluginList() {
  const [plugins, setPlugins] = useState<Plugin[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadPlugins();

    // Listen for plugin events
    const unlistenEnabled = listen('plugin:enabled', (event) => {
      console.log('Plugin enabled:', event.payload);
      loadPlugins();
    });

    const unlistenDisabled = listen('plugin:disabled', (event) => {
      console.log('Plugin disabled:', event.payload);
      loadPlugins();
    });

    return () => {
      unlistenEnabled.then(fn => fn());
      unlistenDisabled.then(fn => fn());
    };
  }, []);

  const loadPlugins = async () => {
    try {
      const result = await invoke<Plugin[]>('list_plugins');
      setPlugins(result);
    } catch (error) {
      console.error('Failed to load plugins:', error);
    } finally {
      setLoading(false);
    }
  };

  const togglePlugin = async (pluginId: string, enabled: boolean) => {
    try {
      if (enabled) {
        await invoke('plugin_disable', { pluginId });
      } else {
        await invoke('plugin_enable', { pluginId });
      }
    } catch (error) {
      console.error('Failed to toggle plugin:', error);
    }
  };

  if (loading) {
    return <div>Loading plugins...</div>;
  }

  return (
    <div className="plugin-list">
      <h2>Installed Plugins</h2>
      <div className="plugins">
        {plugins.map(plugin => (
          <div key={plugin.id} className="plugin-card">
            <div className="plugin-info">
              <h3>{plugin.name}</h3>
              <p className="version">v{plugin.version}</p>
              <p className="description">{plugin.description}</p>
            </div>
            <label className="toggle">
              <input
                type="checkbox"
                checked={plugin.enabled}
                onChange={() => togglePlugin(plugin.id, plugin.enabled)}
              />
              <span className="slider"></span>
            </label>
          </div>
        ))}
      </div>
    </div>
  );
}
```

### 2. Screenshot Component

```typescript
// src/components/ScreenshotTool.tsx

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

export function ScreenshotTool() {
  const [capturing, setCapturing] = useState(false);
  const [lastScreenshot, setLastScreenshot] = useState<string | null>(null);

  useEffect(() => {
    // Listen for screenshot events
    const unlistenStart = listen('screenshot:start', () => {
      setCapturing(true);
    });

    const unlistenSaved = listen<{ path: string }>('screenshot:saved', (event) => {
      setCapturing(false);
      setLastScreenshot(event.payload.path);
    });

    // Listen for hotkey
    const unlistenHotkey = listen('hotkey:screenshot:triggered', () => {
      handleCapture('selection');
    });

    return () => {
      unlistenStart.then(fn => fn());
      unlistenSaved.then(fn => fn());
      unlistenHotkey.then(fn => fn());
    };
  }, []);

  const handleCapture = async (mode: 'fullscreen' | 'selection' | 'window') => {
    try {
      const result = await invoke('plugin_execute', {
        pluginId: 'screenshot',
        command: 'capture',
        args: { mode }
      });
      console.log('Screenshot result:', result);
    } catch (error) {
      console.error('Screenshot failed:', error);
      setCapturing(false);
    }
  };

  return (
    <div className="screenshot-tool">
      <h2>Screenshot Tool</h2>

      <div className="controls">
        <button
          onClick={() => handleCapture('fullscreen')}
          disabled={capturing}
        >
          {capturing ? 'Capturing...' : 'Capture Fullscreen'}
        </button>

        <button
          onClick={() => handleCapture('selection')}
          disabled={capturing}
        >
          {capturing ? 'Capturing...' : 'Capture Selection'}
        </button>

        <button
          onClick={() => handleCapture('window')}
          disabled={capturing}
        >
          {capturing ? 'Capturing...' : 'Capture Window'}
        </button>
      </div>

      {lastScreenshot && (
        <div className="last-screenshot">
          <h3>Last Screenshot</h3>
          <p>{lastScreenshot}</p>
        </div>
      )}

      <div className="hotkey-hint">
        Press <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>S</kbd> to capture
      </div>
    </div>
  );
}
```

### 3. Settings Component

```typescript
// src/components/Settings.tsx

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

export function Settings() {
  const [settings, setSettings] = useState<Record<string, any>>({});

  useEffect(() => {
    loadSettings();

    // Listen for settings changes from other sources
    const unlisten = listen('settings:changed', (event: any) => {
      setSettings(prev => ({
        ...prev,
        [event.payload.key]: event.payload.value
      }));
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const loadSettings = async () => {
    try {
      const result = await invoke<Record<string, any>>('get_settings');
      setSettings(result);
    } catch (error) {
      console.error('Failed to load settings:', error);
    }
  };

  const updateSetting = async (key: string, value: any) => {
    try {
      await invoke('update_settings', {
        update: { key, value }
      });
      // State will be updated via event listener
    } catch (error) {
      console.error('Failed to update setting:', error);
    }
  };

  return (
    <div className="settings">
      <h2>Settings</h2>

      <div className="setting-group">
        <label>
          Screenshot Save Location
          <input
            type="text"
            value={settings.screenshot_path || '~/Pictures/Screenshots'}
            onChange={(e) => updateSetting('screenshot_path', e.target.value)}
          />
        </label>
      </div>

      <div className="setting-group">
        <label>
          <input
            type="checkbox"
            checked={settings.auto_copy_screenshot || false}
            onChange={(e) => updateSetting('auto_copy_screenshot', e.target.checked)}
          />
          Auto-copy screenshots to clipboard
        </label>
      </div>

      <div className="setting-group">
        <label>
          Clipboard History Size
          <input
            type="number"
            value={settings.clipboard_history_size || 100}
            onChange={(e) => updateSetting('clipboard_history_size', parseInt(e.target.value))}
          />
        </label>
      </div>
    </div>
  );
}
```

## tauri.conf.json Configuration

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:5173",
    "distDir": "../dist"
  },
  "package": {
    "productName": "Aplikacja",
    "version": "4.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true
      },
      "dialog": {
        "all": true
      },
      "fs": {
        "all": true,
        "scope": [
          "$HOME/Pictures/Screenshots/**",
          "$APPDATA/**"
        ]
      },
      "clipboard": {
        "all": true
      },
      "globalShortcut": {
        "all": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.aplikacja.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "resizable": true,
        "title": "Aplikacja 4.0",
        "width": 1200,
        "height": 800,
        "center": true,
        "visible": false
      }
    ],
    "systemTray": {
      "iconPath": "icons/tray-icon.png",
      "iconAsTemplate": true
    }
  }
}
```

## Key Benefits of This Integration

1. **Type Safety**: Full type safety from Rust backend to TypeScript frontend
2. **Event-Driven**: Real-time updates via Tauri events
3. **Performance**: Zero serialization overhead with Tauri's IPC
4. **Native Features**: System tray, global hotkeys, notifications
5. **Security**: Tauri's allowlist prevents unauthorized access
6. **Developer Experience**: Hot reload, debugging tools
7. **Cross-Platform**: Single codebase for Windows, macOS, Linux

## Testing Strategy

```rust
// tests/integration_test.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_execution() {
        let core = CoreServices::new().await;
        let mut registry = PluginRegistry::new();

        registry.register(Box::new(EchoPlugin::new()));
        registry.initialize_all(core).await.unwrap();

        let result = registry.execute("echo", "echo", json!({
            "message": "test"
        })).await.unwrap();

        assert_eq!(result["echoed"], "test");
    }
}
```

This integration provides a solid foundation for building your modular Tauri application with plugin support!
