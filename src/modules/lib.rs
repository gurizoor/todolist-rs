
pub use yew::prelude::*;
pub use web_sys::wasm_bindgen::JsCast;
pub use stylist::{css, style, Style, yew::Global};
pub use super::style::*;
pub use gloo_console::log;
pub use web_sys::{HtmlElement, HtmlInputElement};
pub use web_sys::wasm_bindgen::prelude::Closure;
pub use gloo_storage::LocalStorage;
pub use gloo_storage::Storage;
// pub use std::rc::Rc;
// pub use std::cell::RefCell;
pub use chrono::*;
pub use chrono::prelude::*;

pub use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq)]
pub struct Item {
    pub id: String,    //general-+item
    pub level: usize,  //general-event-item -> 3
    pub value: String, // todo label
    pub checked: bool, // ✅❌
    pub is_task: bool, //Identify whether it is a task or a folder
}


#[derive(Clone, PartialEq, Debug)]
pub struct ItemManager {
    pub value: Vec<Item>
}
