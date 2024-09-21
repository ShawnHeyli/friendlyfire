use std::path::PathBuf;

use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_store::{with_store, StoreCollection};

#[derive(Debug, Clone)]
pub enum StoreError {
    UnretrievableStore(String),
    NonExistentKey(String),
}

impl From<StoreError> for tauri_plugin_store::Error {
    fn from(err: StoreError) -> Self {
        match err {
            StoreError::UnretrievableStore(msg) => {
                tauri_plugin_store::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, msg))
            }
            StoreError::NonExistentKey(msg) => tauri_plugin_store::Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                msg,
            )),
        }
    }
}

pub fn store_get_value(handle: &AppHandle, key: String) -> Result<serde_json::Value, StoreError> {
    let stores =
        handle
            .try_state::<StoreCollection<Wry>>()
            .ok_or(StoreError::UnretrievableStore(
                "Store not found".to_string(),
            ))?;
    let path = PathBuf::from("store.bin");

    let value = with_store(handle.clone(), stores, path, |store| {
        let value = store
            .get(key.clone())
            .ok_or(StoreError::NonExistentKey(key.clone()))?;
        Ok(value.clone())
    })
    .unwrap();
    Ok(value)
}

pub fn store_set_value(
    handle: &AppHandle,
    key: String,
    value: serde_json::Value,
) -> Result<(), StoreError> {
    let stores =
        handle
            .try_state::<StoreCollection<Wry>>()
            .ok_or(StoreError::UnretrievableStore(
                "Store not found".to_string(),
            ))?;
    let path = PathBuf::from("store.bin");

    with_store(handle.clone(), stores, path, |store| {
        store.insert(key.clone(), value.clone())
    })
    .unwrap();

    Ok(())
}
