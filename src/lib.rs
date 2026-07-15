#![allow(clippy::manual_is_multiple_of)]

use pgrx::prelude::*;
use cyphera::{Client, ConfigurationFile, MemoryProvider, KeyRecord, KeyStatus};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pgrx::pg_module_magic!();

/// Global Cyphera client — loaded once from JSON configuration file.
static CLIENT: Lazy<Mutex<Option<Client>>> = Lazy::new(|| {
    let path = std::env::var("CYPHERA_CONFIGURATION_FILE")
        .unwrap_or_else(|_| "/etc/cyphera/cyphera.json".to_string());

    match load_client(&path) {
        Ok(client) => {
            pgrx::log!("Cyphera SDK loaded from {}", path);
            Mutex::new(Some(client))
        }
        Err(e) => {
            pgrx::warning!("Failed to load Cyphera config from {}: {}", path, e);
            Mutex::new(None)
        }
    }
});

fn load_client(path: &str) -> Result<Client, String> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read {}: {}", path, e))?;
    let config: serde_json::Value = serde_json::from_str(&contents)
        .map_err(|e| format!("failed to parse JSON: {}", e))?;

    // Extract configurations
    let cf = ConfigurationFile::from_json(&contents)
        .map_err(|e| format!("failed to load configurations: {}", e))?;

    // Extract keys and build MemoryProvider
    let mut key_records = Vec::new();
    if let Some(keys) = config.get("keys").and_then(|k| k.as_object()) {
        for (name, val) in keys {
            if let Some(material_str) = val.get("material").and_then(|m| m.as_str()) {
                let material = hex::decode(material_str)
                    .map_err(|e| format!("bad key hex for {}: {}", name, e))?;
                key_records.push(KeyRecord {
                    key_ref: name.to_string(),
                    version: 1,
                    status: KeyStatus::Active,
                    material,
                    tweak: vec![],
                });
            }
        }
    }

    let provider = MemoryProvider::new(key_records);
    Client::from_configuration(cf, Box::new(provider))
        .map_err(|e| format!("failed to create client: {}", e))
}

fn with_client<F, T>(f: F) -> T
where
    F: FnOnce(&Client) -> T,
    T: Default,
{
    let guard = CLIENT.lock().unwrap();
    match guard.as_ref() {
        Some(client) => f(client),
        None => {
            pgrx::warning!("Cyphera SDK not loaded — check CYPHERA_CONFIGURATION_FILE");
            T::default()
        }
    }
}

/// Protect a value using a named configuration.
/// Returns header-prefixed ciphertext with passthrough characters preserved.
#[pg_extern]
fn cyphera_protect(configuration_name: &str, value: &str) -> String {
    with_client(|client| {
        match client.protect(configuration_name, value) {
            Ok(result) => result.output,
            Err(e) => format!("[error: {}]", e),
        }
    })
}

/// Access a protected value using the embedded header.
/// No configuration name needed — the header identifies the configuration.
#[pg_extern]
fn cyphera_access(protected_value: &str) -> String {
    with_client(|client| {
        match client.access(protected_value) {
            Ok(result) => result.output,
            Err(e) => format!("[error: {}]", e),
        }
    })
}

/// Access a protected value with explicit configuration name.
/// Use this for values without a header.
#[pg_extern(name = "cyphera_access")]
fn cyphera_access_with_configuration(configuration_name: &str, protected_value: &str) -> String {
    with_client(|client| {
        match client.access_with_config(configuration_name, protected_value) {
            Ok(result) => result.output,
            Err(e) => format!("[error: {}]", e),
        }
    })
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_extension_loads() {
        // Just verify the extension can be created
        assert!(true);
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
