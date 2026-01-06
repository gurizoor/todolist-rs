// use web_sys::{wasm_bindgen::prelude::Closure, HtmlElement, HtmlInputElement};

use super::super::lib::*;
use super::super::lib::storage::{StorageManager, date_utils};

impl ItemManager {
    /// Create a new instance of `ItemManager` with an empty list of items.
    ///
    /// This function initializes a new `ItemManager` with an empty `value`
    /// vector, which will hold items of type `Item`. It also logs an empty
    /// message to the console.
    pub fn new() -> Self {
        log!("");
        Self {
            value: Vec::<Item>::new(),
        }
    }

    /// Return a new ItemManager instance with all items sorted by their name
    /// in ascending order, and then by their level in ascending order.
    pub fn sort_value(&self) -> Self {
        let mut value = self.value.clone();
        value.sort_by(|a, b| a.value.cmp(&b.value));
        value.sort_by(|a, b| a.level.cmp(&b.level));
        Self { value: value }
    }

    /// Return a new ItemManager instance with all items of level 1 removed from
    /// the list.
    pub fn rmv_levl_below_1(&self) -> Self {
        let mut value = self.value.clone();
        value.retain(|x| x.level > 1);
        Self { value: value }
    }

    /// Add a new item to the ItemManager's list.
    ///
    /// If an item with the same ID as the given one already exists in the list,
    /// it is not added again.
    ///
    /// The new item is added to the end of the list.
    pub fn add(&self, item: Item) -> Self {
        let mut value = self.value.clone();
        for value_item in &value {
            if value_item.id == item.id {
                return Self { value: value };
            }
        }
        // item.add();
        value.push(item.clone());
        Self { value: value }
    }

    /// Remove an item from the ItemManager's list.
    ///
    /// This function does not just remove the item from the list,
    /// but also calls the `rmv` method on the item to remove it
    /// from the DOM.
    ///
    /// The returned ItemManager will have the item removed from its
    /// internal list.
    pub fn rmv(&self, item: Item) -> Self {
        item.rmv();
        let mut value = self.value.clone();
        value.retain(|x| x.id != item.id);
        Self { value: value }
    }

    // pub fn all_rmv(&self) -> Self
    // {
    //     self.value.iter().for_each(|x| x.rmv());
    //     let value = Vec::<Item>::new();
    //     Self { value: value }
    // }

    /// Save the current state of the items to the browser's local storage.
    ///
    /// This saves only the items with level > 1 (i.e. the items that are
    /// visible in the UI). The items are sorted first, and then serialized
    /// to JSON. The JSON is then stored in the "items" key in the local
    /// storage.
    pub fn save_data(&self) -> Self {
        log!("");
        let mut value = self.sort_value().value;
        value.retain(|item| item.level > 1);
        
        if let Err(e) = StorageManager::save_items(&value) {
            log!("Error saving items: {}", format!("{}", e));
        }
        
        log!(format!("Saved items: {:?}", value));
        self.clone()
    }

    /// Loads data from localStorage and returns a new ItemManager instance with the loaded data.
    /// If data is not found in localStorage, return a new instance with an empty value.
    ///
    /// This method is used to load the data from localStorage when the application starts.
    pub fn load_data(&self) -> Self {
        let mut new_self = self.clone();
        match StorageManager::load_items() {
            Ok(mut loaded_items) => {
                loaded_items.retain(|item| item.level > 1);
                new_self.value.append(&mut loaded_items);
                log!("Loaded from localStorage!");
            }
            Err(e) => {
                log!("Error loading items: {}", format!("{}", e));
            }
        }
        new_self
    }

    /// Returns a new ItemManager instance with the value loaded from localStorage.
    /// If data is not found in localStorage, return a new instance with an empty value.
    pub fn s_load_data() -> Self {
        log!("");
        log!("s_load_data Function was called");
        match StorageManager::load_items() {
            Ok(mut value) => {
                value.retain(|item| item.level > 1);
                log!(format!("Loaded from localStorage: {:?}", value));
                Self { value }
            }
            Err(e) => {
                log!("Error loading items: {}", format!("{}", e));
                Self {
                    value: Vec::<Item>::new(),
                }
            }
        }
    }

    /// Return the id of the selected folder.
    ///
    /// This function go through self.value and find the first folder which is checked.
    /// If no folder is checked, return "Not found selected folder".
    ///
    pub fn selected_folder(&self) -> String {
        self.debug();
        log!("");
        log!("selected_folder Function was called");
        log!(format!("    Function items.value:{:?}", &self.value));
        let document = web_sys::window().unwrap().document().unwrap();
        let mut selected_folder = "Not found selected folder".to_string();
        for i in self.value.iter() {
            log!(format!("        Function selected_folder: {}", i.id));
            if i.is_task {
                continue;
            }
            let folder = document
                .get_element_by_id(&format!("{}-input", i.id))
                .unwrap();
            if folder
                .dyn_ref::<web_sys::HtmlInputElement>()
                .unwrap()
                .checked()
            {
                selected_folder = i.id.clone();
            }
        }
        selected_folder
    }

    /// This function is called when the page is loaded. It checks if the day, week, or month has changed, and if so, unchecks the corresponding checkboxes. It also saves the current date to local storage, and saves the state of the checkboxes to local storage.
    pub fn init_checkbox(&self) -> Self {
        log!("");
        log!("init_checkbox Function was called");
        
        let current_date = date_utils::get_current_date();
        let previous_date = StorageManager::load_previous_date().unwrap_or_else(|_| current_date.clone());

        let day_changed = date_utils::has_day_changed(&current_date, &previous_date).unwrap_or(false);
        let week_changed = date_utils::has_week_changed(&current_date, &previous_date).unwrap_or(false);
        let month_changed = date_utils::has_month_changed(&current_date, &previous_date).unwrap_or(false);

        log!(format!("day_changed: {}", day_changed));
        log!(format!("week_changed: {}", week_changed));
        log!(format!("month_changed: {}", month_changed));

        let mut new_self = self.clone();
        let mut new_value = Vec::new();
        for item in &new_self.value {
            let mut new_item = item.clone();
            match new_item.get_check_type().as_str() {
                "General" => (),
                "Day" => {
                    if day_changed {
                        new_item = new_item.set_checked(false);
                    }
                }
                "Week" => {
                    if week_changed {
                        new_item = new_item.set_checked(false);
                    }
                }
                "Month" => {
                    if month_changed {
                        new_item = new_item.set_checked(false);
                    }
                }
                _ => panic!("Invalid check type: {}", item.get_check_type()),
            }
            new_value.push(new_item);
        }
        new_self.value = new_value;

        // save date
        if let Err(e) = StorageManager::save_previous_date(&current_date) {
            log!("Error saving previous date: {}", format!("{}", e));
        }

        // save items
        if let Err(e) = StorageManager::save_items(&new_self.value) {
            log!("Error saving items: {}", format!("{}", e));
        }

        log!(format!("new self value :{:?}", new_self.value));
        new_self
    }

    pub fn debug(&self) {
        log!("");
        log!(format!(
            "{}",
            Local::now().format("%Y-%m-%d-%u").to_string()
        )); //.split("-").collect::<Vec<_>>()));
        log!(format!(
            "{:?}",
            Local::now()
                .format("%Y-%m-%d-%u")
                .to_string()
                .split("-")
                .collect::<Vec<_>>()
        ));
        // log!(format!("{}", "2".to_string().parse::<usize>().unwrap()+1));
        // log!(format!("Function debug self.value:{:?}", &self.value));
        // log!(format!("Function debug LocalStorage: {:?}", &self.load_data().value));
    }
}
