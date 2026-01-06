use super::*;
use gloo_storage::{LocalStorage, Storage};

/// Storage key for items
const ITEMS_KEY: &str = "items";
/// Storage key for previous date
const PREVIOUS_DATE_KEY: &str = "previous_date";

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
}

/// Date utilities for storage operations
pub mod date_utils {
    use chrono::Local;

    /// Get current date as formatted string
    pub fn get_current_date() -> String {
        Local::now().format("%Y-%m-%d-%u").to_string()
    }

    /// Parse date string into components
    pub fn parse_date(date_str: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
        date_str
            .split("-")
            .map(|x| x.parse::<usize>().map_err(|e| e.into()))
            .collect()
    }

    /// Check if day has changed between two dates
    pub fn has_day_changed(current: &str, previous: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let current_parts = parse_date(current)?;
        let previous_parts = parse_date(previous)?;
        
        Ok(current_parts[2] != previous_parts[2]
            || current_parts[1] != previous_parts[1]
            || current_parts[0] != previous_parts[0])
    }

    /// Check if week has changed between two dates
    pub fn has_week_changed(current: &str, previous: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let current_parts = parse_date(current)?;
        let previous_parts = parse_date(previous)?;
        
        Ok(current_parts[0] != previous_parts[0]
            || (current_parts[3] <= previous_parts[3]
                && (current_parts[2] != previous_parts[2]
                    || current_parts[1] != previous_parts[1]))
            || previous_parts[2] + 7 <= current_parts[2])
    }

    /// Check if month has changed between two dates
    pub fn has_month_changed(current: &str, previous: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let current_parts = parse_date(current)?;
        let previous_parts = parse_date(previous)?;
        
        Ok(current_parts[0] != previous_parts[0]
            || current_parts[1] != previous_parts[1])
    }
}
