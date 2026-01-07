pub use super::style::*;
pub use chrono::*;
pub use gloo_console::log;
pub use gloo_storage::Storage;
pub use stylist::{css, style, yew::Global, Style};
pub use web_sys::wasm_bindgen::prelude::Closure;
pub use web_sys::wasm_bindgen::JsCast;
pub use web_sys::{HtmlElement, HtmlInputElement};
pub use yew::prelude::*;

pub use serde::{Deserialize, Serialize};

pub mod dom_utils;
pub mod storage;
pub mod date_utils;

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
    pub value: Vec<Item>,
}
