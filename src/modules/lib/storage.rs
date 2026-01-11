use super::*;
use gloo_storage::{LocalStorage, Storage};

/// Storage key for items
const ITEMS_KEY: &str = "items";
/// Storage key for previous date
const PREVIOUS_DATE_KEY: &str = "previous_date";
/// Storage key for logs
const HISTORY_KEY: &str = "history";

/// Storage abstraction layer for localStorage operations
pub struct StorageManager;

impl StorageManager {
    /// Save items to localStorage
    pub fn save_items(items: &[Item]) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(items)?;
        LocalStorage::set(ITEMS_KEY, &json)?;
        log!("Items saved to localStorage");
        Ok(())
    }

    /// Load items from localStorage
    pub fn load_items() -> Result<Vec<Item>, Box<dyn std::error::Error>> {
        if let Ok(json) = LocalStorage::get::<String>(ITEMS_KEY) {
            let items: Vec<Item> = serde_json::from_str(&json)?;
            log!("Items loaded from localStorage");
            Ok(items)
        } else {
            log!("No items found in localStorage");
            Ok(Vec::new())
        }
    }

    /// Save previous date to localStorage
    pub fn save_previous_date(date: &str) -> Result<(), Box<dyn std::error::Error>> {
        LocalStorage::set(PREVIOUS_DATE_KEY, date)?;
        log!("Previous date saved to localStorage");
        Ok(())
    }

    /// Load previous date from localStorage
    pub fn load_previous_date() -> Result<String, Box<dyn std::error::Error>> {
        if let Ok(date) = LocalStorage::get::<String>(PREVIOUS_DATE_KEY) {
            log!("Previous date loaded from localStorage");
            Ok(date)
        } else {
            log!("No previous date found in localStorage");
            Ok(String::new())
        }
    }

    /// Save logs to localStorage
    pub fn save_logs(logs: &History) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(logs)?;
        LocalStorage::set(HISTORY_KEY, &json)?;
        log!("Logs saved to localStorage");
        Ok(())
    }

    /// Load logs from localStorage
    pub fn load_logs() -> Result<History, Box<dyn std::error::Error>> {
        if let Ok(json) = LocalStorage::get::<String>(HISTORY_KEY) {
            let logs: History = serde_json::from_str(&json)?;
            log!("Logs loaded from localStorage");
            Ok(logs)
        } else {
            log!("No logs found in localStorage");
            Ok(History::new())
        }
    }
}
