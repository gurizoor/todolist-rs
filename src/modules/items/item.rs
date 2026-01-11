use super::super::lib::*;
use web_sys::Element;

impl Item {
    /// Create a new `Item` instance.
    ///
    /// This function initializes a new `Item` with the provided `id`, `checked` state,
    /// and `is_task` flag. The `level` of the item is determined by the count of
    /// "-+" in the `id`, and the `value` is derived from the last part of the `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - A `String` representing the unique identifier for the item, which
    ///   also determines the `level` and `value`.
    /// * `checked` - A `bool` indicating whether the item is checked or not.
    /// * `is_task` - A `bool` indicating whether the item is a task.

    pub fn new(id: String, checked: bool, is_task: bool) -> Item {
        Item {
            id: id.clone(),
            level: id.split("-+").count() - 1,
            value: id.split("-+").last().unwrap().to_string(),
            checked,
            is_task,
        }
    }

    /// Return a new `Item` instance with the `checked` state updated.
    ///
    /// This function creates a new `Item` with the same `id` and `is_task`
    /// attributes, but with the `checked` attribute set to the provided value.
    /// It is useful for updating the checked state of an item.

    pub fn set_checked(&self, checked: bool) -> Self {
        Item::new(self.id.clone(), checked, self.is_task)
    }

    /// Return the type of the checkbox of the item.
    ///
    /// The type of the checkbox is the second part of the item's ID, separated by "-+". If the item's
    /// ID does not contain a "-+", the default value is returned.
    pub fn get_check_type(&self) -> String {
        self.id.split("-+").nth(1).unwrap_or_default().to_string()
    }

    /// Return the parent element of the item.
    ///
    /// The parent element is the element which has the same ID as the item, but with the last part
    /// removed. For example, if the item's ID is "1-+2-+3", its parent element's ID is "1-+2".
    /// If there is no parent element, the function will panic with the message "parent is None!".
    pub fn get_parent_element(&self) -> Element {
        let document = web_sys::window().unwrap().document().unwrap();
        let mut parts: Vec<String> = self.id.split("-+").map(String::from).collect();
        parts.pop();
        let parent_id = parts.join("-+");
        document
            .get_element_by_id(&parent_id)
            .expect("parent is None!")
    }

    /// Remove the element with the same ID as the item from the DOM.
    ///
    /// If the element is found, it is removed from its parent node. If the element is not found,
    /// the function does nothing.
    fn remove_task(&self) {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(element) = document.get_element_by_id(&self.id) {
            if let Some(parent) = element.parent_node() {
                parent.remove_child(&element).unwrap();
            }
        }
    }

    /// Remove the folder element from the DOM.
    ///
    /// This function finds the element with the same ID as the item and removes
    /// it from its parent node in the DOM. If the element or its parent is not found,
    /// the function does nothing.

    fn remove_folder(&self) {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(element) = document.get_element_by_id(&self.id) {
            if let Some(parent) = element.parent_node() {
                parent.remove_child(&element).unwrap();
            }
        }
    }

    /// Remove the item from the DOM.
    ///
    /// If the item is a task, it removes the element with the same ID as the item.
    /// If the item is a folder, it removes the folder element with the same ID as the item.
    /// If the element or its parent is not found, the function does nothing.
    pub fn remove(&self) {
        if self.is_task {
            self.remove_task();
        } else {
            self.remove_folder();
        }
    }
}
