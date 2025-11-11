# Minimal Working Example: Plugin System MVP

This document provides a minimal, runnable example to get started with the plugin architecture.

## Step 1: Core Plugin API

```rust
// core/src/lib.rs

pub mod plugin_api;
pub mod plugin_registry;
pub mod core_services;

pub use plugin_api::*;
pub use plugin_registry::*;
pub use core_services::*;
```

```rust
// core/src/plugin_api.rs

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
}

#[async_trait]
pub trait Plugin: Send + Sync {
    fn info(&self) -> PluginInfo;

    async fn initialize(&mut self, core: CoreServices) -> anyhow::Result<()>;

    async fn execute(&self, command: &str, args: serde_json::Value) -> anyhow::Result<serde_json::Value>;

    async fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
```

## Step 2: Core Services (Minimal)

```rust
// core/src/core_services.rs

use tokio::sync::broadcast;
use std::sync::Arc;

#[derive(Clone)]
pub struct CoreServices {
    pub events: EventBus,
}

impl CoreServices {
    pub fn new() -> Self {
        Self {
            events: EventBus::new(),
        }
    }
}

#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<String>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub fn publish(&self, event: String) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.sender.subscribe()
    }
}
```

## Step 3: Plugin Registry

```rust
// core/src/plugin_registry.rs

use crate::plugin_api::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PluginRegistry {
    plugins: HashMap<String, Arc<RwLock<Box<dyn Plugin>>>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        let id = plugin.info().id.clone();
        self.plugins.insert(id, Arc::new(RwLock::new(plugin)));
    }

    pub async fn initialize_all(&self, core: CoreServices) -> anyhow::Result<()> {
        for (_id, plugin) in &self.plugins {
            let mut p = plugin.write().await;
            p.initialize(core.clone()).await?;
        }
        Ok(())
    }

    pub async fn execute(
        &self,
        plugin_id: &str,
        command: &str,
        args: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        let plugin = self.plugins.get(plugin_id)
            .ok_or_else(|| anyhow::anyhow!("Plugin not found"))?;

        let p = plugin.read().await;
        p.execute(command, args).await
    }

    pub async fn list_plugins(&self) -> Vec<PluginInfo> {
        let mut list = Vec::new();
        for (_id, plugin) in &self.plugins {
            let p = plugin.read().await;
            list.push(p.info());
        }
        list
    }

    pub async fn shutdown_all(&self) -> anyhow::Result<()> {
        for (_id, plugin) in &self.plugins {
            let mut p = plugin.write().await;
            p.shutdown().await?;
        }
        Ok(())
    }
}
```

## Step 4: Example Plugin 1 - Echo Plugin

```rust
// plugins/echo/src/lib.rs

use aplikacja_core::*;
use async_trait::async_trait;
use serde_json::json;

pub struct EchoPlugin {
    info: PluginInfo,
    core: Option<CoreServices>,
}

impl EchoPlugin {
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                id: "echo".to_string(),
                name: "Echo Plugin".to_string(),
                version: "1.0.0".to_string(),
            },
            core: None,
        }
    }
}

#[async_trait]
impl Plugin for EchoPlugin {
    fn info(&self) -> PluginInfo {
        self.info.clone()
    }

    async fn initialize(&mut self, core: CoreServices) -> anyhow::Result<()> {
        println!("✓ Echo plugin initialized");

        // Publish initialization event
        core.events.publish("plugin:echo:initialized".to_string());

        self.core = Some(core);
        Ok(())
    }

    async fn execute(&self, command: &str, args: serde_json::Value) -> anyhow::Result<serde_json::Value> {
        match command {
            "echo" => {
                let message = args.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Hello!");

                // Publish event
                if let Some(core) = &self.core {
                    core.events.publish(format!("echo: {}", message));
                }

                Ok(json!({
                    "echoed": message,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }))
            }
            _ => Err(anyhow::anyhow!("Unknown command: {}", command))
        }
    }

    async fn shutdown(&mut self) -> anyhow::Result<()> {
        println!("✓ Echo plugin shutdown");
        Ok(())
    }
}
```

## Step 5: Example Plugin 2 - Counter Plugin

```rust
// plugins/counter/src/lib.rs

use aplikacja_core::*;
use async_trait::async_trait;
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct CounterPlugin {
    info: PluginInfo,
    core: Option<CoreServices>,
    count: AtomicU64,
}

impl CounterPlugin {
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                id: "counter".to_string(),
                name: "Counter Plugin".to_string(),
                version: "1.0.0".to_string(),
            },
            core: None,
            count: AtomicU64::new(0),
        }
    }
}

#[async_trait]
impl Plugin for CounterPlugin {
    fn info(&self) -> PluginInfo {
        self.info.clone()
    }

    async fn initialize(&mut self, core: CoreServices) -> anyhow::Result<()> {
        println!("✓ Counter plugin initialized");

        // Subscribe to echo events
        let mut events = core.events.subscribe();
        let count = self.count.clone();

        tokio::spawn(async move {
            while let Ok(event) = events.recv().await {
                if event.starts_with("echo:") {
                    let new_count = count.fetch_add(1, Ordering::SeqCst) + 1;
                    println!("  → Counter: Echo event received (count: {})", new_count);
                }
            }
        });

        core.events.publish("plugin:counter:initialized".to_string());
        self.core = Some(core);
        Ok(())
    }

    async fn execute(&self, command: &str, args: serde_json::Value) -> anyhow::Result<serde_json::Value> {
        match command {
            "get" => {
                Ok(json!({
                    "count": self.count.load(Ordering::SeqCst)
                }))
            }
            "increment" => {
                let new_count = self.count.fetch_add(1, Ordering::SeqCst) + 1;

                if let Some(core) = &self.core {
                    core.events.publish(format!("counter:incremented:{}", new_count));
                }

                Ok(json!({
                    "count": new_count
                }))
            }
            "reset" => {
                self.count.store(0, Ordering::SeqCst);
                Ok(json!({ "count": 0 }))
            }
            _ => Err(anyhow::anyhow!("Unknown command: {}", command))
        }
    }
}
```

## Step 6: Main Application

```rust
// src/main.rs

use aplikacja_core::*;
use echo_plugin::EchoPlugin;
use counter_plugin::CounterPlugin;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 Aplikacja 4.0 - Plugin System Demo\n");

    // Create core services
    let core = CoreServices::new();

    // Create plugin registry
    let mut registry = PluginRegistry::new();

    // Register plugins
    println!("📦 Registering plugins...");
    registry.register(Box::new(EchoPlugin::new()));
    registry.register(Box::new(CounterPlugin::new()));

    // Initialize all plugins
    println!("\n⚙️  Initializing plugins...");
    registry.initialize_all(core.clone()).await?;

    // List plugins
    println!("\n📋 Available plugins:");
    for plugin_info in registry.list_plugins().await {
        println!("  • {} ({})", plugin_info.name, plugin_info.id);
    }

    // Demo: Execute commands
    println!("\n🎯 Demo: Executing commands...\n");

    // Echo command
    let result = registry.execute("echo", "echo", serde_json::json!({
        "message": "Hello from Aplikacja!"
    })).await?;
    println!("Echo result: {}", result);

    // Small delay to let event propagate
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Get counter
    let result = registry.execute("counter", "get", serde_json::json!({})).await?;
    println!("Counter result: {}", result);

    // More echos
    for i in 1..=3 {
        registry.execute("echo", "echo", serde_json::json!({
            "message": format!("Message {}", i)
        })).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    // Check counter again
    let result = registry.execute("counter", "get", serde_json::json!({})).await?;
    println!("\nFinal counter: {}", result);

    // Shutdown
    println!("\n🛑 Shutting down...");
    registry.shutdown_all().await?;

    println!("\n✅ Done!");

    Ok(())
}
```

## Step 7: Cargo Workspace Configuration

```toml
# Cargo.toml (workspace root)

[workspace]
members = [
    "core",
    "plugins/echo",
    "plugins/counter",
]

resolver = "2"

[workspace.dependencies]
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
chrono = "0.4"
```

```toml
# core/Cargo.toml

[package]
name = "aplikacja-core"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
async-trait = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
```

```toml
# plugins/echo/Cargo.toml

[package]
name = "echo-plugin"
version = "0.1.0"
edition = "2021"

[dependencies]
aplikacja-core = { path = "../../core" }
tokio = { workspace = true }
async-trait = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
chrono = { workspace = true }
```

```toml
# plugins/counter/Cargo.toml

[package]
name = "counter-plugin"
version = "0.1.0"
edition = "2021"

[dependencies]
aplikacja-core = { path = "../../core" }
tokio = { workspace = true }
async-trait = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
```

## Step 8: Run the Example

```bash
# In the workspace root
cargo run
```

**Expected Output:**

```
🚀 Aplikacja 4.0 - Plugin System Demo

📦 Registering plugins...

⚙️  Initializing plugins...
✓ Echo plugin initialized
✓ Counter plugin initialized

📋 Available plugins:
  • Echo Plugin (echo)
  • Counter Plugin (counter)

🎯 Demo: Executing commands...

Echo result: {"echoed":"Hello from Aplikacja!","timestamp":"2025-11-11T10:30:00Z"}
  → Counter: Echo event received (count: 1)
Counter result: {"count":1}
  → Counter: Echo event received (count: 2)
  → Counter: Echo event received (count: 3)
  → Counter: Echo event received (count: 4)

Final counter: {"count":4}

🛑 Shutting down...
✓ Echo plugin shutdown
✓ Counter plugin shutdown

✅ Done!
```

## Key Concepts Demonstrated

1. **Plugin Trait**: Simple interface that all plugins implement
2. **CoreServices**: Shared services (event bus) accessible to all plugins
3. **Event Bus**: Pub/sub for inter-plugin communication
4. **Plugin Registry**: Central manager for plugin lifecycle
5. **Async/Await**: Fully async architecture using Tokio
6. **Type Safety**: Compile-time guarantees with traits
7. **Zero-Cost Abstraction**: Static dispatch (no vtable overhead in hot paths)

## Next Steps to Add to This MVP

1. **Add Tauri integration**:
   ```rust
   #[tauri::command]
   async fn plugin_command(
       registry: tauri::State<'_, PluginRegistry>,
       plugin_id: String,
       command: String,
       args: serde_json::Value,
   ) -> Result<serde_json::Value, String> {
       registry.execute(&plugin_id, &command, args)
           .await
           .map_err(|e| e.to_string())
   }
   ```

2. **Add persistence** (using `sled` or `redb`)

3. **Add permissions system** (validate before execute)

4. **Add configuration UI** (React frontend)

5. **Add your first real plugin** (Screenshot Tool with egui)

---

This minimal example gives you a working foundation to build upon. Each plugin is isolated, communicates via events, and can be easily tested independently.
