# Aplikacja 4.0 - Plugin Architecture Blueprint

## Executive Summary

This document outlines the recommended plugin architecture for Aplikacja 4.0, a modular Rust/Tauri desktop productivity suite. The architecture uses a **hybrid approach**: static linking for core plugins with a trait-based system, combined with optional dynamic loading for third-party extensions, and process isolation for performance-critical UI components.

**Key Recommendations:**
- **Plugin Type**: Trait-based static plugins (primary) + optional dynamic libraries (advanced)
- **IPC**: Tokio channels (in-process), gRPC/tonic (out-of-process)
- **State**: Message-passing architecture with a central state manager
- **Discovery**: Manifest-based registration with compile-time safety

---

## 1. Plugin Definition & Discovery

### 1.1 Recommended Approach: Hybrid Static-First Architecture

**Primary: Static Trait-Based Plugins (90% of plugins)**

```rust
// core/src/plugin_api.rs

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    /// Core services this plugin requires
    pub required_services: Vec<String>,
    /// Permissions needed (clipboard, filesystem, network, etc.)
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    Clipboard,
    Filesystem { paths: Vec<String> },
    Network { domains: Vec<String> },
    Screenshot,
    GlobalHotkeys,
    SystemTray,
}

/// Core plugin trait - all plugins must implement this
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn manifest(&self) -> &PluginManifest;

    /// Initialize the plugin with access to core services
    async fn initialize(&mut self, core: CoreServices) -> Result<(), PluginError>;

    /// Shutdown and cleanup
    async fn shutdown(&mut self) -> Result<(), PluginError>;

    /// Handle commands from the frontend or other plugins
    async fn handle_command(&self, command: PluginCommand) -> Result<PluginResponse, PluginError>;

    /// Optional: Register global hotkeys
    fn hotkeys(&self) -> Vec<HotkeyBinding> {
        vec![]
    }

    /// Optional: Provide system tray menu items
    fn tray_menu_items(&self) -> Vec<TrayMenuItem> {
        vec![]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCommand {
    pub command: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResponse {
    pub success: bool,
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Initialization failed: {0}")]
    InitializationError(String),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Core service error: {0}")]
    CoreServiceError(String),
}

#[derive(Debug, Clone)]
pub struct HotkeyBinding {
    pub id: String,
    pub keys: String, // e.g., "Ctrl+Shift+S"
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct TrayMenuItem {
    pub id: String,
    pub label: String,
    pub icon: Option<Vec<u8>>,
}
```

**Why Static First?**
- ✅ Compile-time safety (no ABI breakage)
- ✅ Zero overhead abstraction
- ✅ Easy debugging and profiling
- ✅ Better IDE support
- ✅ Simpler dependency management
- ❌ Requires recompilation to add plugins

### 1.2 Plugin Registry & Discovery

```rust
// core/src/plugin_registry.rs

use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

pub struct PluginRegistry {
    plugins: HashMap<String, Arc<RwLock<Box<dyn Plugin>>>>,
    enabled_plugins: HashMap<String, bool>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            enabled_plugins: HashMap::new(),
        }
    }

    /// Register a plugin (called at startup)
    pub async fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let id = plugin.manifest().id.clone();

        // Validate permissions
        self.validate_permissions(&plugin)?;

        self.plugins.insert(id.clone(), Arc::new(RwLock::new(plugin)));
        self.enabled_plugins.insert(id, true);

        Ok(())
    }

    /// Initialize all enabled plugins
    pub async fn initialize_all(&mut self, core: CoreServices) -> Result<(), PluginError> {
        for (id, plugin) in &self.plugins {
            if *self.enabled_plugins.get(id).unwrap_or(&false) {
                let mut plugin = plugin.write().await;
                plugin.initialize(core.clone()).await?;
            }
        }
        Ok(())
    }

    /// Get a plugin by ID
    pub fn get(&self, id: &str) -> Option<Arc<RwLock<Box<dyn Plugin>>>> {
        self.plugins.get(id).cloned()
    }

    /// Send command to a specific plugin
    pub async fn send_command(
        &self,
        plugin_id: &str,
        command: PluginCommand,
    ) -> Result<PluginResponse, PluginError> {
        let plugin = self.get(plugin_id)
            .ok_or_else(|| PluginError::InvalidCommand("Plugin not found".into()))?;

        let plugin = plugin.read().await;
        plugin.handle_command(command).await
    }

    fn validate_permissions(&self, plugin: &Box<dyn Plugin>) -> Result<(), PluginError> {
        // TODO: Implement permission validation logic
        Ok(())
    }
}
```

### 1.3 Application Startup - Plugin Registration

```rust
// src/main.rs

mod plugins;
use plugins::*;

#[tokio::main]
async fn main() {
    // Initialize core services
    let core_services = CoreServices::new().await;

    // Create plugin registry
    let mut registry = PluginRegistry::new();

    // Register all plugins (static linking)
    registry.register(Box::new(ScreenshotPlugin::new())).await.unwrap();
    registry.register(Box::new(ClipboardManagerPlugin::new())).await.unwrap();
    registry.register(Box::new(TextExpansionPlugin::new())).await.unwrap();
    registry.register(Box::new(WindowTilingPlugin::new())).await.unwrap();
    // ... register other plugins

    // Initialize all plugins
    registry.initialize_all(core_services.clone()).await.unwrap();

    // Start Tauri application
    tauri::Builder::default()
        .manage(registry)
        .manage(core_services)
        .invoke_handler(tauri::generate_handler![
            plugin_command,
            get_plugin_list,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 1.4 Optional: Dynamic Loading for Third-Party Plugins

For advanced use cases where you want to support third-party plugins without recompilation:

```rust
// core/src/dynamic_plugin.rs

use libloading::{Library, Symbol};
use std::path::PathBuf;

/// Function signature for dynamic plugin entry point
type PluginCreate = unsafe fn() -> *mut dyn Plugin;

pub struct DynamicPluginLoader {
    plugin_dir: PathBuf,
}

impl DynamicPluginLoader {
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self { plugin_dir }
    }

    /// Load a dynamic plugin from a .dll/.so file
    pub unsafe fn load(&self, plugin_name: &str) -> Result<Box<dyn Plugin>, PluginError> {
        let plugin_path = self.plugin_dir.join(format!("lib{}.so", plugin_name));

        let lib = Library::new(plugin_path)
            .map_err(|e| PluginError::InitializationError(e.to_string()))?;

        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")
            .map_err(|e| PluginError::InitializationError(e.to_string()))?;

        let plugin_ptr = constructor();
        let plugin = Box::from_raw(plugin_ptr);

        // Keep library loaded
        std::mem::forget(lib);

        Ok(plugin)
    }
}
```

**⚠️ Dynamic Loading Caveats:**
- ABI compatibility is fragile in Rust (use `abi_stable` crate if you go this route)
- Security concerns (loading untrusted code)
- More complex debugging
- Potential dependency conflicts

**Recommendation**: Start with static plugins, add dynamic loading later only if needed for a plugin marketplace.

---

## 2. Communication Architecture

### 2.1 In-Process Communication: Shared State + Message Passing

For plugins running in the same process (most of your plugins):

```rust
// core/src/core_services.rs

use tokio::sync::{mpsc, RwLock, broadcast};
use std::sync::Arc;

/// Core services provided to all plugins
#[derive(Clone)]
pub struct CoreServices {
    /// Clipboard service
    pub clipboard: Arc<CoreClipboard>,
    /// Notification service
    pub notifications: Arc<CoreNotifications>,
    /// Storage service (persistent data)
    pub storage: Arc<CoreStorage>,
    /// Event bus for inter-plugin communication
    pub events: EventBus,
    /// Settings service
    pub settings: Arc<CoreSettings>,
}

impl CoreServices {
    pub async fn new() -> Self {
        Self {
            clipboard: Arc::new(CoreClipboard::new().await),
            notifications: Arc::new(CoreNotifications::new()),
            storage: Arc::new(CoreStorage::new("./data").await),
            events: EventBus::new(),
            settings: Arc::new(CoreSettings::new().await),
        }
    }
}

/// Event bus for pub/sub between plugins
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<AppEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self { sender }
    }

    /// Publish an event
    pub fn publish(&self, event: AppEvent) {
        let _ = self.sender.send(event);
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.sender.subscribe()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    ClipboardChanged { content: String },
    ScreenshotTaken { path: String },
    HotkeyTriggered { hotkey_id: String },
    PluginEnabled { plugin_id: String },
    PluginDisabled { plugin_id: String },
    SettingsChanged { key: String, value: serde_json::Value },
}
```

### 2.2 Core Service Example: Clipboard

```rust
// core/src/services/clipboard.rs

use arboard::Clipboard as SystemClipboard;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct CoreClipboard {
    system_clipboard: Arc<RwLock<SystemClipboard>>,
    history: Arc<RwLock<Vec<ClipboardEntry>>>,
    max_history: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    pub id: String,
    pub content: ClipboardContent,
    pub timestamp: i64,
    pub source_app: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardContent {
    Text(String),
    Image(Vec<u8>),
    Files(Vec<String>),
}

impl CoreClipboard {
    pub async fn new() -> Self {
        Self {
            system_clipboard: Arc::new(RwLock::new(SystemClipboard::new().unwrap())),
            history: Arc::new(RwLock::new(Vec::new())),
            max_history: 1000,
        }
    }

    /// Get clipboard history (for clipboard manager plugin)
    pub async fn get_history(&self) -> Vec<ClipboardEntry> {
        self.history.read().await.clone()
    }

    /// Add entry to history
    pub async fn add_to_history(&self, entry: ClipboardEntry) {
        let mut history = self.history.write().await;
        history.insert(0, entry);
        if history.len() > self.max_history {
            history.truncate(self.max_history);
        }
    }

    /// Get current clipboard content
    pub async fn get_text(&self) -> Result<String, String> {
        self.system_clipboard.read().await
            .get_text()
            .map_err(|e| e.to_string())
    }

    /// Set clipboard content
    pub async fn set_text(&self, text: String) -> Result<(), String> {
        self.system_clipboard.write().await
            .set_text(text)
            .map_err(|e| e.to_string())
    }
}
```

### 2.3 Out-of-Process Communication: gRPC for Native Overlays

For plugins like the Screenshot Tool that use `egui` in a separate process:

```protobuf
// proto/screenshot_service.proto

syntax = "proto3";

package screenshot;

service ScreenshotService {
    // Start the overlay
    rpc StartOverlay(StartOverlayRequest) returns (StartOverlayResponse);

    // Screenshot taken event
    rpc OnScreenshotTaken(ScreenshotData) returns (ScreenshotResponse);

    // Cancel screenshot
    rpc CancelScreenshot(CancelRequest) returns (CancelResponse);
}

message StartOverlayRequest {
    int32 monitor_index = 1;
    bool include_cursor = 2;
}

message StartOverlayResponse {
    bool success = 1;
    string error_message = 2;
}

message ScreenshotData {
    bytes image_data = 1;
    string format = 2; // "png", "jpg"
    int32 width = 3;
    int32 height = 4;
}

message ScreenshotResponse {
    bool success = 1;
    string saved_path = 2;
}

message CancelRequest {}
message CancelResponse {
    bool success = 1;
}
```

**Implementation:**

```rust
// plugins/screenshot/src/grpc_server.rs

use tonic::{transport::Server, Request, Response, Status};
use screenshot_service::screenshot_service_server::{ScreenshotService, ScreenshotServiceServer};

pub struct ScreenshotServiceImpl {
    core: CoreServices,
}

#[tonic::async_trait]
impl ScreenshotService for ScreenshotServiceImpl {
    async fn start_overlay(
        &self,
        request: Request<StartOverlayRequest>,
    ) -> Result<Response<StartOverlayResponse>, Status> {
        let req = request.into_inner();

        // Spawn the egui overlay process
        let result = spawn_overlay_process(req.monitor_index, req.include_cursor).await;

        match result {
            Ok(_) => Ok(Response::new(StartOverlayResponse {
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Ok(Response::new(StartOverlayResponse {
                success: false,
                error_message: e.to_string(),
            })),
        }
    }

    async fn on_screenshot_taken(
        &self,
        request: Request<ScreenshotData>,
    ) -> Result<Response<ScreenshotResponse>, Status> {
        let data = request.into_inner();

        // Save screenshot
        let path = save_screenshot(&data).await.map_err(|e| {
            Status::internal(e.to_string())
        })?;

        // Publish event
        self.core.events.publish(AppEvent::ScreenshotTaken {
            path: path.clone(),
        });

        Ok(Response::new(ScreenshotResponse {
            success: true,
            saved_path: path,
        }))
    }
}

async fn spawn_overlay_process(monitor: i32, include_cursor: bool) -> Result<(), String> {
    // Spawn the separate egui process
    std::process::Command::new("./screenshot_overlay")
        .arg("--monitor")
        .arg(monitor.to_string())
        .arg(if include_cursor { "--cursor" } else { "" })
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

**Why gRPC for out-of-process?**
- ✅ Strong typing with protobuf
- ✅ Efficient binary serialization
- ✅ Bidirectional streaming
- ✅ Language-agnostic (if you need C++ for some overlay logic)
- ✅ Built-in error handling

**Alternative**: Unix domain sockets with custom protocol (lighter weight but less type-safe)

---

## 3. State Management

### 3.1 Recommended Pattern: Message-Passing with Centralized State Manager

```rust
// core/src/state_manager.rs

use tokio::sync::{mpsc, RwLock};
use std::sync::Arc;
use std::collections::HashMap;

/// Centralized state manager
pub struct StateManager {
    state: Arc<RwLock<AppState>>,
    command_tx: mpsc::Sender<StateCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub settings: HashMap<String, serde_json::Value>,
    pub plugin_states: HashMap<String, serde_json::Value>,
    pub user_data: HashMap<String, serde_json::Value>,
}

#[derive(Debug)]
pub enum StateCommand {
    UpdateSetting {
        key: String,
        value: serde_json::Value,
        reply: tokio::sync::oneshot::Sender<Result<(), String>>,
    },
    GetSetting {
        key: String,
        reply: tokio::sync::oneshot::Sender<Option<serde_json::Value>>,
    },
    UpdatePluginState {
        plugin_id: String,
        state: serde_json::Value,
        reply: tokio::sync::oneshot::Sender<Result<(), String>>,
    },
}

impl StateManager {
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::channel(1000);
        let state = Arc::new(RwLock::new(AppState {
            settings: HashMap::new(),
            plugin_states: HashMap::new(),
            user_data: HashMap::new(),
        }));

        // Spawn state manager task
        let state_clone = state.clone();
        tokio::spawn(async move {
            Self::run_state_loop(state_clone, command_rx).await;
        });

        Self { state, command_tx }
    }

    async fn run_state_loop(
        state: Arc<RwLock<AppState>>,
        mut command_rx: mpsc::Receiver<StateCommand>,
    ) {
        while let Some(command) = command_rx.recv().await {
            match command {
                StateCommand::UpdateSetting { key, value, reply } => {
                    let mut state = state.write().await;
                    state.settings.insert(key.clone(), value.clone());

                    // Persist to disk asynchronously
                    tokio::spawn(async move {
                        // Save to file
                    });

                    let _ = reply.send(Ok(()));
                }
                StateCommand::GetSetting { key, reply } => {
                    let state = state.read().await;
                    let value = state.settings.get(&key).cloned();
                    let _ = reply.send(value);
                }
                StateCommand::UpdatePluginState { plugin_id, state: plugin_state, reply } => {
                    let mut state = state.write().await;
                    state.plugin_states.insert(plugin_id, plugin_state);
                    let _ = reply.send(Ok(()));
                }
            }
        }
    }

    /// Public API for plugins
    pub async fn set_setting(&self, key: String, value: serde_json::Value) -> Result<(), String> {
        let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();

        self.command_tx.send(StateCommand::UpdateSetting {
            key,
            value,
            reply: reply_tx,
        }).await.map_err(|e| e.to_string())?;

        reply_rx.await.map_err(|e| e.to_string())?
    }

    pub async fn get_setting(&self, key: String) -> Option<serde_json::Value> {
        let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();

        self.command_tx.send(StateCommand::GetSetting {
            key,
            reply: reply_tx,
        }).await.ok()?;

        reply_rx.await.ok()?
    }
}
```

### 3.2 Plugin-Specific State Isolation

```rust
// core/src/services/storage.rs

use sled::Db;
use std::path::PathBuf;

pub struct CoreStorage {
    db: Db,
}

impl CoreStorage {
    pub async fn new(data_dir: &str) -> Self {
        let db = sled::open(data_dir).unwrap();
        Self { db }
    }

    /// Get plugin-specific storage namespace
    pub fn get_plugin_storage(&self, plugin_id: &str) -> PluginStorage {
        let tree = self.db.open_tree(plugin_id).unwrap();
        PluginStorage { tree }
    }
}

pub struct PluginStorage {
    tree: sled::Tree,
}

impl PluginStorage {
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
        self.tree.get(key)
            .map(|opt| opt.map(|v| v.to_vec()))
            .map_err(|e| e.to_string())
    }

    pub fn set(&self, key: &str, value: &[u8]) -> Result<(), String> {
        self.tree.insert(key, value)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    pub fn delete(&self, key: &str) -> Result<(), String> {
        self.tree.remove(key)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}
```

---

## 4. Potential Pitfalls & Mitigations

### 4.1 Dependency Hell Between Plugins

**Problem**: Plugin A uses `tokio 1.30`, Plugin B uses `tokio 1.35` → compilation fails.

**Solutions**:
1. **Workspace-level dependency management** (recommended for static plugins)
   ```toml
   # Cargo.toml (workspace root)
   [workspace]
   members = [
       "core",
       "plugins/screenshot",
       "plugins/clipboard_manager",
       # ...
   ]

   [workspace.dependencies]
   tokio = { version = "1.35", features = ["full"] }
   serde = { version = "1.0", features = ["derive"] }
   # ... other shared dependencies
   ```

2. **Strict version pinning** in plugin manifests

3. **For dynamic plugins**: Use `abi_stable` crate to create stable ABI boundaries

### 4.2 Performance Bottlenecks

**Potential Issues**:
- Too much locking (deadlocks, contention)
- Inefficient serialization for IPC
- Plugin initialization blocking main thread

**Mitigations**:
1. **Use message-passing instead of shared locks** where possible
2. **Lazy initialization**:
   ```rust
   pub async fn initialize(&mut self, core: CoreServices) -> Result<(), PluginError> {
       // Don't do heavy work here
       self.core = Some(core);

       // Spawn background task for actual initialization
       tokio::spawn(async move {
           // Heavy initialization work
       });

       Ok(())
   }
   ```

3. **Profile early**: Use `cargo flamegraph` and `tokio-console`

4. **Batch operations**:
   ```rust
   // Bad: 1000 individual state updates
   for item in items {
       state_manager.update(item).await;
   }

   // Good: Batch update
   state_manager.batch_update(items).await;
   ```

### 4.3 Security Vulnerabilities

**Risks**:
- Plugins accessing resources they shouldn't
- Untrusted plugin code execution
- Command injection via IPC

**Mitigations**:
1. **Permission system** (already in design):
   ```rust
   impl Plugin {
       fn manifest(&self) -> &PluginManifest {
           &PluginManifest {
               // ...
               permissions: vec![
                   Permission::Clipboard,
                   Permission::Filesystem { paths: vec!["/home/user/screenshots".into()] },
               ],
           }
       }
   }
   ```

2. **Validate all IPC inputs**:
   ```rust
   pub async fn handle_command(&self, command: PluginCommand) -> Result<PluginResponse, PluginError> {
       // Validate command
       if !self.is_command_allowed(&command.command) {
           return Err(PluginError::PermissionDenied(command.command));
       }

       // Sanitize payload
       let payload = sanitize_json(&command.payload)?;

       // Process...
   }
   ```

3. **For dynamic plugins**: Implement code signing and verification

### 4.4 Tauri Frontend Synchronization

**Problem**: Frontend React state getting out of sync with backend plugin state.

**Solution**: Use Tauri events as single source of truth:

```rust
// backend: plugins/clipboard/src/lib.rs

impl Plugin for ClipboardManagerPlugin {
    async fn handle_command(&self, command: PluginCommand) -> Result<PluginResponse, PluginError> {
        match command.command.as_str() {
            "get_history" => {
                let history = self.core.clipboard.get_history().await;
                Ok(PluginResponse {
                    success: true,
                    data: serde_json::to_value(history).unwrap(),
                })
            }
            "paste_item" => {
                let item_id: String = serde_json::from_value(command.payload)?;
                // ... paste logic

                // Emit event to frontend
                self.app_handle.emit_all("clipboard:item_pasted", item_id)?;

                Ok(PluginResponse { success: true, data: json!({}) })
            }
            _ => Err(PluginError::InvalidCommand(command.command)),
        }
    }
}
```

```typescript
// frontend: src/hooks/useClipboardManager.ts

import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

export function useClipboardManager() {
  const [history, setHistory] = useState([]);

  useEffect(() => {
    // Subscribe to backend events
    const unlisten = listen('clipboard:item_pasted', (event) => {
      // Refresh history
      refreshHistory();
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const refreshHistory = async () => {
    const result = await invoke('plugin_command', {
      pluginId: 'clipboard_manager',
      command: { command: 'get_history', payload: {} }
    });
    setHistory(result.data);
  };

  return { history, refreshHistory };
}
```

### 4.5 Plugin Lifecycle Management

**Problem**: Plugins not cleaning up resources on shutdown.

**Solution**: Implement proper shutdown sequence:

```rust
// core/src/plugin_registry.rs

impl PluginRegistry {
    pub async fn shutdown_all(&mut self) -> Result<(), PluginError> {
        // Shutdown in reverse order of initialization
        let plugin_ids: Vec<_> = self.plugins.keys().cloned().collect();

        for id in plugin_ids.iter().rev() {
            if let Some(plugin) = self.plugins.get(id) {
                let mut plugin = plugin.write().await;

                // Timeout after 5 seconds
                tokio::time::timeout(
                    std::time::Duration::from_secs(5),
                    plugin.shutdown()
                ).await.ok();
            }
        }

        Ok(())
    }
}
```

---

## 5. Example Plugin Implementation

### 5.1 Simple Plugin: Text Expansion

```rust
// plugins/text_expansion/src/lib.rs

use aplikacja_core::*;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct TextExpansionPlugin {
    manifest: PluginManifest,
    core: Option<CoreServices>,
    shortcuts: HashMap<String, String>,
}

impl TextExpansionPlugin {
    pub fn new() -> Self {
        Self {
            manifest: PluginManifest {
                id: "text_expansion".into(),
                name: "Text Expansion".into(),
                version: "1.0.0".into(),
                author: "Aplikacja Team".into(),
                description: "Expand text shortcuts".into(),
                required_services: vec!["clipboard".into()],
                permissions: vec![Permission::Clipboard],
            },
            core: None,
            shortcuts: HashMap::new(),
        }
    }
}

#[async_trait]
impl Plugin for TextExpansionPlugin {
    fn manifest(&self) -> &PluginManifest {
        &self.manifest
    }

    async fn initialize(&mut self, core: CoreServices) -> Result<(), PluginError> {
        // Load shortcuts from storage
        let storage = core.storage.get_plugin_storage("text_expansion");

        if let Ok(Some(data)) = storage.get("shortcuts") {
            self.shortcuts = serde_json::from_slice(&data).unwrap_or_default();
        }

        // Subscribe to clipboard events
        let mut events = core.events.subscribe();
        let shortcuts = self.shortcuts.clone();
        let clipboard = core.clipboard.clone();

        tokio::spawn(async move {
            while let Ok(event) = events.recv().await {
                if let AppEvent::ClipboardChanged { content } = event {
                    // Check if content matches a shortcut
                    if let Some(expansion) = shortcuts.get(&content) {
                        clipboard.set_text(expansion.clone()).await.ok();
                    }
                }
            }
        });

        self.core = Some(core);
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), PluginError> {
        // Save shortcuts
        if let Some(core) = &self.core {
            let storage = core.storage.get_plugin_storage("text_expansion");
            let data = serde_json::to_vec(&self.shortcuts).unwrap();
            storage.set("shortcuts", &data).ok();
        }
        Ok(())
    }

    async fn handle_command(&self, command: PluginCommand) -> Result<PluginResponse, PluginError> {
        match command.command.as_str() {
            "add_shortcut" => {
                #[derive(Deserialize)]
                struct AddShortcut {
                    trigger: String,
                    expansion: String,
                }

                let payload: AddShortcut = serde_json::from_value(command.payload)
                    .map_err(|e| PluginError::InvalidCommand(e.to_string()))?;

                // Would need interior mutability pattern here, or use message passing
                // self.shortcuts.insert(payload.trigger, payload.expansion);

                Ok(PluginResponse {
                    success: true,
                    data: json!({}),
                })
            }
            "get_shortcuts" => {
                Ok(PluginResponse {
                    success: true,
                    data: serde_json::to_value(&self.shortcuts).unwrap(),
                })
            }
            _ => Err(PluginError::InvalidCommand(command.command)),
        }
    }
}
```

### 5.2 Complex Plugin: Screenshot Tool with egui Overlay

```rust
// plugins/screenshot/src/lib.rs

use aplikacja_core::*;
use async_trait::async_trait;

pub struct ScreenshotPlugin {
    manifest: PluginManifest,
    core: Option<CoreServices>,
    grpc_server: Option<ScreenshotServiceImpl>,
}

impl ScreenshotPlugin {
    pub fn new() -> Self {
        Self {
            manifest: PluginManifest {
                id: "screenshot".into(),
                name: "Screenshot Tool".into(),
                version: "1.0.0".into(),
                author: "Aplikacja Team".into(),
                description: "Native screenshot tool with overlay".into(),
                required_services: vec!["notifications".into(), "storage".into()],
                permissions: vec![
                    Permission::Screenshot,
                    Permission::Filesystem { paths: vec!["/home/user/screenshots".into()] },
                ],
            },
            core: None,
            grpc_server: None,
        }
    }
}

#[async_trait]
impl Plugin for ScreenshotPlugin {
    fn manifest(&self) -> &PluginManifest {
        &self.manifest
    }

    async fn initialize(&mut self, core: CoreServices) -> Result<(), PluginError> {
        // Start gRPC server for overlay communication
        let grpc_service = ScreenshotServiceImpl::new(core.clone());

        tokio::spawn(async move {
            Server::builder()
                .add_service(ScreenshotServiceServer::new(grpc_service))
                .serve("[::1]:50051".parse().unwrap())
                .await
                .unwrap();
        });

        self.core = Some(core);
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), PluginError> {
        // Shutdown gRPC server
        Ok(())
    }

    async fn handle_command(&self, command: PluginCommand) -> Result<PluginResponse, PluginError> {
        match command.command.as_str() {
            "take_screenshot" => {
                // Spawn overlay process
                std::process::Command::new("./screenshot_overlay")
                    .spawn()
                    .map_err(|e| PluginError::InitializationError(e.to_string()))?;

                Ok(PluginResponse {
                    success: true,
                    data: json!({}),
                })
            }
            _ => Err(PluginError::InvalidCommand(command.command)),
        }
    }

    fn hotkeys(&self) -> Vec<HotkeyBinding> {
        vec![
            HotkeyBinding {
                id: "screenshot:capture".into(),
                keys: "Ctrl+Shift+S".into(),
                description: "Take screenshot".into(),
            }
        ]
    }
}
```

---

## 6. Project Structure

```
aplikacja-4.0/
├── Cargo.toml (workspace)
├── core/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── plugin_api.rs
│   │   ├── plugin_registry.rs
│   │   ├── core_services.rs
│   │   ├── state_manager.rs
│   │   └── services/
│   │       ├── clipboard.rs
│   │       ├── notifications.rs
│   │       ├── storage.rs
│   │       └── settings.rs
├── plugins/
│   ├── screenshot/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── grpc_server.rs
│   │   └── proto/
│   │       └── screenshot_service.proto
│   ├── clipboard_manager/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── text_expansion/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── window_tiling/
│       ├── Cargo.toml
│       └── src/lib.rs
├── overlays/
│   └── screenshot_overlay/
│       ├── Cargo.toml
│       └── src/main.rs (egui application)
├── src/
│   ├── main.rs (Tauri app + plugin registration)
│   └── tauri_commands.rs
├── frontend/
│   └── src/
│       ├── App.tsx
│       └── hooks/
│           ├── usePlugins.ts
│           └── useClipboardManager.ts
└── proto/ (shared protobuf definitions)
    └── common.proto
```

---

## 7. Key Recommendations Summary

### ✅ DO:

1. **Start with static trait-based plugins** (90% of your needs)
2. **Use message-passing** for state management (avoid shared mutable state)
3. **Use gRPC** for out-of-process communication (screenshot overlay)
4. **Implement a permission system** from day one
5. **Use workspace dependencies** to avoid version conflicts
6. **Profile early and often** (use `cargo flamegraph`, `tokio-console`)
7. **Lazy initialization** for plugins (don't block startup)
8. **Tauri events** as source of truth for frontend state

### ❌ DON'T:

1. **Don't use dynamic loading** unless you need a plugin marketplace
2. **Don't share mutable state** directly between plugins (use CoreServices)
3. **Don't do heavy work** in `initialize()` (spawn background tasks)
4. **Don't trust plugin inputs** (validate, sanitize, check permissions)
5. **Don't block the main thread** (everything async)
6. **Don't forget shutdown** (implement proper cleanup)

---

## 8. Next Steps

1. **Prototype the core plugin trait and registry** (~1 week)
2. **Implement 2-3 simple plugins** (text expansion, clipboard) to validate API (~1 week)
3. **Build the screenshot plugin + egui overlay** to test IPC (~2 weeks)
4. **Implement state persistence** with `sled` (~3 days)
5. **Build the Tauri frontend** for plugin management (~1 week)
6. **Add remaining plugins** incrementally

---

## 9. Alternative Architectures Considered

### Architecture B: WASM-based Plugins
**Pros**: True sandboxing, cross-platform
**Cons**: Performance overhead, limited system access, immature Rust WASM-WASI ecosystem
**Verdict**: ❌ Not suitable for performance-critical desktop app

### Architecture C: Microservices with REST
**Pros**: Language-agnostic, easy debugging
**Cons**: HTTP overhead, more complex than needed for local app
**Verdict**: ❌ Overkill for desktop application

### Architecture D: Pure Dynamic Loading
**Pros**: No recompilation needed
**Cons**: ABI instability, security risks, complex debugging
**Verdict**: ❌ Too risky for initial version, maybe v2.0 feature

---

## 10. References & Further Reading

- **Tauri Plugin System**: https://tauri.app/v1/guides/features/plugin/
- **Rust Plugin Architecture**: https://nullderef.com/blog/plugin-dynload/
- **`abi_stable` crate**: https://docs.rs/abi_stable/latest/abi_stable/
- **tonic (gRPC)**: https://github.com/hyperium/tonic
- **egui**: https://github.com/emilk/egui
- **sled (embedded DB)**: https://github.com/spacejam/sled

---

**Document Version**: 1.0
**Last Updated**: 2025-11-11
**Author**: Claude (Anthropic)
**Status**: Draft for Review
