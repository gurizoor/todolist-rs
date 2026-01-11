use super::super::lib::*;
use crate::modules::lib::storage::StorageManager;

impl History {
    pub fn new() -> Self {
        History {
            history: vec![],
            is_added_for_history: Vec::new(),
            undo_history: Vec::new(),
            is_added_for_undo_history: Vec::new(),
        }
    }

    pub fn add_log(&self, item: Item, is_add: bool, _items: ItemManager, is_undone: bool) -> Self {
        let mut new_self = StorageManager::load_logs().unwrap();
        if !is_undone {
            new_self.history.push(item);
            new_self.is_added_for_history.push(is_add);
        }
        StorageManager::save_logs(&new_self).unwrap();
        new_self
    }

    pub fn undo(
        &self,
        items: UseStateHandle<ItemManager>,
        history: &UseStateHandle<History>,
    ) -> Self {
        let mut new_self = StorageManager::load_logs().unwrap();
        if let Some(task) = new_self.history.pop() {
            match new_self.is_added_for_history.pop() {
                Some(true) => {
                    let current_items = ItemManager::s_load_data();
                    let updated_items = current_items.remove(task.clone());
                    items.set(updated_items.save_data());

                    new_self.undo_history.push(task);
                    new_self.is_added_for_undo_history.push(false);
                }
                Some(false) => {
                    let current_items = ItemManager::s_load_data();
                    let updated_items = current_items
                        .add(current_items.add_item_to_dom(&task, true, &items, history));
                    items.set(updated_items.save_data());
                    
                    new_self.undo_history.push(task);
                    new_self.is_added_for_undo_history.push(true);
                }
                None => log!("Error! in History.remove()"),
            }
            log!("undo operation completed");
        }
        StorageManager::save_logs(&new_self).unwrap();
        log!(format!("History after undo: {:?}", new_self));
        new_self
    }

    pub fn redo(
        &self,
        items: UseStateHandle<ItemManager>,
        history: &UseStateHandle<History>,
    ) -> Self {
        let mut new_self = StorageManager::load_logs().unwrap();
        if let Some(task) = new_self.undo_history.pop() {
            match new_self.is_added_for_undo_history.pop() {
                Some(true) => {
                    let current_items = ItemManager::s_load_data();
                    let updated_items = current_items.remove(task.clone());
                    items.set(updated_items.save_data());

                    new_self.history.push(task);
                    new_self.is_added_for_history.push(false);
                }
                Some(false) => {
                    let current_items = ItemManager::s_load_data();
                    let updated_items = current_items
                        .add(current_items.add_item_to_dom(&task, true, &items, history));
                    items.set(updated_items.save_data());

                    new_self.history.push(task);
                    new_self.is_added_for_history.push(true);
                }
                None => log!("Error! in History.redo()"),
            }
            log!("redo operation completed");
        }
        StorageManager::save_logs(&new_self).unwrap();
        log!(format!("History after redo: {:?}", new_self));
        new_self
    }

    pub fn clear_redo(&self) -> Self {
        let mut new_self = StorageManager::load_logs().unwrap();
        new_self.undo_history.clear();
        new_self.is_added_for_undo_history.clear();
        StorageManager::save_logs(&new_self).unwrap();
        new_self
    }
}
