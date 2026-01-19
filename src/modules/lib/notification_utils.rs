use crate::modules::lib::*;
use web_sys::wasm_bindgen::prelude::Closure;
use web_sys::wasm_bindgen::JsCast;

/// Creates the main notification element with styling and content
fn create_notification_element() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let notification = document.create_element("div").unwrap();
    notification.set_attribute("style", 
        "position: fixed; top: 20px; right: 20px; background: rgba(0, 0, 0, 0.8); color: white; padding: 15px 20px; border-radius: 8px; z-index: 3000; font-size: 14px; max-width: 300px;"
    ).unwrap();
    notification.set_inner_html("テーマを変更しました。<br>変更を完全に適用するにはページをリロードしてください。<br><small>リロードすると現在の作業内容は保存されます。</small>");
    notification
}

/// Creates the buttons container for reload and close buttons
fn create_buttons_container() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let buttons_container = document.create_element("div").unwrap();
    buttons_container
        .set_attribute("style", "margin-top: 10px; display: flex; gap: 10px;")
        .unwrap();
    buttons_container
}

/// Creates the reload button with click handler
fn create_reload_button() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let reload_btn = document.create_element("button").unwrap();
    reload_btn.set_attribute("style", 
        "background: #28a745; color: white; border: none; padding: 5px 10px; border-radius: 4px; cursor: pointer; font-size: 12px;"
    ).unwrap();
    reload_btn.set_inner_html("今すぐリロード");

    let reload_listener = Closure::wrap(Box::new(move |_: MouseEvent| {
        if let Some(window) = web_sys::window() {
            window.location().reload().unwrap();
        }
    }) as Box<dyn FnMut(_)>);

    reload_btn
        .add_event_listener_with_callback("click", reload_listener.as_ref().unchecked_ref())
        .ok();
    reload_listener.forget();

    reload_btn
}

/// Creates the close button with click handler
fn create_close_button(notification: web_sys::Element) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let close_btn = document.create_element("button").unwrap();
    close_btn.set_attribute("style", 
        "background: #6c757d; color: white; border: none; padding: 5px 10px; border-radius: 4px; cursor: pointer; font-size: 12px;"
    ).unwrap();
    close_btn.set_inner_html("閉じる");

    let notification_clone = notification.clone();
    let close_listener = Closure::wrap(Box::new(move |_: MouseEvent| {
        if let Some(parent) = notification_clone.parent_node() {
            parent.remove_child(&notification_clone).ok();
        }
    }) as Box<dyn FnMut(_)>);

    close_btn
        .add_event_listener_with_callback("click", close_listener.as_ref().unchecked_ref())
        .ok();
    close_listener.forget();

    close_btn
}

/// Sets up auto-removal of notification after 8 seconds
fn setup_auto_remove(notification: web_sys::Element) {
    if let Some(window) = web_sys::window() {
        let notification_auto = notification.clone();
        let timeout_closure = Closure::wrap(Box::new(move || {
            if let Some(parent) = notification_auto.parent_node() {
                parent.remove_child(&notification_auto).ok();
            }
        }) as Box<dyn Fn()>);

        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                timeout_closure.as_ref().unchecked_ref(),
                8000,
            )
            .ok();
        timeout_closure.forget();
    }
}

/// Shows a reload notification when theme is changed
pub fn show_reload_notification() {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(body) = document.body() {
                // Create notification element
                let notification = create_notification_element();

                // Create buttons container
                let buttons_container = create_buttons_container();

                // Create and add buttons
                let reload_btn = create_reload_button();
                let close_btn = create_close_button(notification.clone());

                // Assemble the notification
                buttons_container.append_child(&reload_btn).unwrap();
                buttons_container.append_child(&close_btn).unwrap();
                notification.append_child(&buttons_container).unwrap();
                body.append_child(&notification).unwrap();

                // Setup auto-removal
                setup_auto_remove(notification);
            }
        }
    }
}
