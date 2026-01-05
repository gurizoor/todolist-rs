use super::*;
use web_sys::MouseEvent;

/// Create a container div element for an item
pub fn create_container_div(item: &Item) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let new_div = document.create_element("div").unwrap();
    new_div.set_id(&item.id);
    item.get_parent_element().append_child(&new_div).unwrap();
    new_div
}

/// Create an input element (checkbox or radio) for an item
pub fn create_input_element(item: &Item) -> HtmlInputElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let new_input = document.create_element("input").unwrap();
    let input_element = new_input.dyn_into::<HtmlInputElement>().unwrap();
    input_element.set_id(&format!("{}-input", item.id));

    if item.is_task {
        input_element.set_type("checkbox");
        input_element.set_checked(item.checked);
        input_element
            .set_attribute("class", task_checkbox().get_class_name())
            .unwrap();
    } else {
        input_element.set_type("radio");
        input_element.set_name("folder");
        input_element.set_checked(item.checked);
        input_element
            .set_attribute("class", folder_input().get_class_name())
            .unwrap();
    }

    input_element
}

/// Create a label element for an item
pub fn create_label_element(item: &Item) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let new_label = document.create_element("label").unwrap();
    let label_element = new_label.clone().dyn_into::<HtmlElement>().unwrap();
    label_element.set_inner_html(&item.value);
    label_element
        .set_attribute("for", &format!("{}-input", item.id))
        .unwrap();
    new_label
}

/// Create a remove button for an item
pub fn create_remove_button() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let new_button = document.create_element("button").unwrap();
    new_button.set_inner_html("x");
    new_button.set_class_name(button_list().get_class_name());
    new_button
}

/// Set up click event listener for input element
pub fn setup_input_click_listener(
    input_element: &HtmlInputElement,
    item: &Item,
    items: &UseStateHandle<ItemManager>,
) {
    let items_clone = items.clone();
    let item_clone = item.clone();
    let input_element_clone = input_element.clone();
    
    let listener = Closure::wrap(Box::new(move |_: MouseEvent| {
        let mut value = ItemManager::s_load_data();
        value.value.retain(|x| x.id != item_clone.id);
        value
            .value
            .push(item_clone.set_checked(input_element_clone.checked()));
        items_clone.set(value.save_data());
    }) as Box<dyn Fn(MouseEvent)>);

    input_element
        .add_event_listener_with_callback("click", &listener.as_ref().unchecked_ref())
        .unwrap();
    listener.forget();
}

/// Set up click event listener for remove button
pub fn setup_remove_click_listener(
    button_element: &web_sys::Element,
    item: &Item,
    items: &UseStateHandle<ItemManager>,
) {
    let items_clone = items.clone();
    let item_clone = item.clone();
    
    let listener = Closure::wrap(Box::new(move |_: MouseEvent| {
        items_clone.set(
            ItemManager::s_load_data()
                .rmv(item_clone.clone())
                .save_data(),
        );
    }) as Box<dyn Fn(MouseEvent)>);

    button_element
        .add_event_listener_with_callback("click", &listener.as_ref().unchecked_ref())
        .unwrap();
    listener.forget();
}