use stylist::StyleSource;

use super::lib::*;

pub fn global_style() -> StyleSource {
    css!(
        r#"
            body {
                background-color: rgb(30, 30, 30);
                color: rgb(255, 255, 255);
            }
        "#
    )
}

pub fn title() -> Style {
    style!(
        r#"
            color: rgb(220, 220, 0);
        "#
    )
    .unwrap()
}

pub fn div_list() -> Style {
    style!(
        r#"
            border: 2px solid rgb(50, 50, 50);
            border-radius: 10px;
            margin: 5px;
            padding: 5px;
            padding-left: 10px;
        "#
    )
    .unwrap()
}

pub fn button_list() -> Style {
    style!(
        r#"
            background-color: rgb(50, 50, 50);
            border: 1px solid rgb(200, 200, 200);
            border-radius: 5px;
            color: rgb(220, 220, 220);
        "#
    )
    .unwrap()
}

pub fn task_input() -> Style {
    style!(
        r#"
            width: 50%;
            color: rgb(220, 220, 220);
            background-color: rgb(30, 30, 30);
            border: 1px solid rgb(255, 255, 255);
        "#
    )
    .unwrap()
}

pub fn task_button() -> Style {
    style!(
        r#"
            width: 10%;
            color: rgb(240, 240, 240);
            background: rgb(50, 50, 50);
            border: 1px solid rgb(200, 200, 200);
        "#
    )
    .unwrap()
}

pub fn task_checkbox() -> Style {
    style!(
        r#"
            accent-color: rgba(80, 120, 255, 1);
        "#
    )
    .unwrap()
}

pub fn folder_input() -> Style {
    style!(
        r#"
            accent-color: rgba(80, 120, 255, 1);
            background-color: rgb(30, 30, 30);
            border: 1px solid rgb(255, 255, 255);
        "#
    )
    .unwrap()
}
