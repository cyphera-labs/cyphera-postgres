use pgrx::prelude::*;
use cyphera::{Client, PolicyFile, MemoryProvider, KeyRecord, KeyStatus};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pgrx::pg_module_magic!();

/// Global Cyphera client — loaded once from YAML policy file.
static CLIENT: Lazy<Mutex<Option<Client>>> = Lazy::new(|| {
    let path = std::env::var("CYPHERA_POLICY_FILE")
        .unwrap_or_else(|_| "/etc/cyphera/cyphera.yaml".to_string());

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
    let config: serde_yaml::Value = serde_yaml::from_str(&contents)
        .map_err(|e| format!("failed to parse YAML: {}", e))?;

    // Extract policies
    let pf = PolicyFile::from_yaml(&contents)
        .map_err(|e| format!("failed to load policies: {}", e))?;

    // Extract keys and build MemoryProvider
    let mut key_records = Vec::new();
    if let Some(keys) = config.get("keys").and_then(|k| k.as_mapping()) {
        for (name, val) in keys {
            if let (Some(name_str), Some(material_str)) = (
                name.as_str(),
                val.get("material").and_then(|m| m.as_str()),
            ) {
                let material = hex::decode(material_str)
                    .map_err(|e| format!("bad key hex for {}: {}", name_str, e))?;
                key_records.push(KeyRecord {
                    key_ref: name_str.to_string(),
                    version: 1,
                    status: KeyStatus::Active,
                    material,
                    tweak: vec![],
                });
            }
        }
    }

    let provider = MemoryProvider::new(key_records);
    Client::from_policy(pf, Box::new(provider))
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
            pgrx::warning!("Cyphera SDK not loaded — check CYPHERA_POLICY_FILE");
            T::default()
        }
    }
}

/// Protect a value using a named policy.
/// Returns tagged ciphertext with passthrough characters preserved.
#[pg_extern]
fn cyphera_protect(policy_name: &str, value: &str) -> String {
    with_client(|client| {
        match client.protect(policy_name, value) {
            Ok(result) => result.output,
            Err(e) => format!("[error: {}]", e),
        }
    })
}

/// Access (decrypt) a protected value using the embedded tag.
/// No policy name needed — the tag identifies the policy.
#[pg_extern]
fn cyphera_unprotect(protected_value: &str) -> String {
    with_client(|client| {
        match client.access_by_tag(protected_value) {
            Ok(result) => result.output,
            Err(e) => format!("[error: {}]", e),
        }
    })
}

/// Access (decrypt) a protected value with explicit policy name.
/// Use this for untagged values.
#[pg_extern]
fn cyphera_access(policy_name: &str, protected_value: &str) -> String {
    with_client(|client| {
        match client.access(policy_name, protected_value) {
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
