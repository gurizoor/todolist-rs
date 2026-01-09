use stylist::StyleSource;

use super::lib::*;

pub fn global_style() -> StyleSource {
    css!(
        r#"
            body {
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                color: #ffffff;
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                min-height: 100vh;
                margin: 0;
                padding: 20px;
            }
        "#
    )
}

pub fn title() -> Style {
    style!(
        r#"
            color: #ffffff;
            font-size: 2.5rem;
            font-weight: 300;
            text-align: center;
            margin-bottom: 30px;
            text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
        "#
    )
    .unwrap()
}

pub fn div_list() -> Style {
    style!(
        r#"
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 15px;
            margin: 10px 0;
            padding: 15px;
            padding-left: 20px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
            transition: all 0.3s ease;
            label {
                font-size: 26px;
                font-weight: 700;
                color: #ffffff;
                text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.8);
                -webkit-text-stroke: 1px rgba(0, 0, 0, 0.5);
                text-stroke: 1px rgba(0, 0, 0, 0.5);
            }
        "#
    )
    .unwrap()
}

pub fn button_list() -> Style {
    style!(
        r#"
            background: linear-gradient(45deg, #667eea, #764ba2);
            border: 2px solid rgba(255, 255, 255, 0.3);
            border-radius: 8px;
            color: #ffffff;
            padding: 10px 20px;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.3s ease;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
            transform: translateY(0);
            &:hover {
                background: linear-gradient(45deg, #7c8ff0, #8b5bb8);
                border-color: rgba(255, 255, 255, 0.5);
                box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
                transform: translateY(-2px);
            }
            &:active {
                background: linear-gradient(45deg, #5a6fe8, #6a4196);
                border-color: rgba(255, 255, 255, 0.7);
                box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
                transform: translateY(0);
            }
        "#
    )
    .unwrap()
}

pub fn task_input() -> Style {
    style!(
        r#"
            width: 50%;
            padding: 12px 16px;
            font-size: 16px;
            color: #ffffff;
            background: rgba(255, 255, 255, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.3);
            border-radius: 8px;
            backdrop-filter: blur(5px);
            transition: all 0.3s ease;
            ::placeholder {
                color: rgba(255, 255, 255, 0.7);
            }
        "#
    )
    .unwrap()
}

pub fn task_button() -> Style {
    style!(
        r#"
            background: linear-gradient(45deg, #667eea, #764ba2);
            border: 2px solid rgba(255, 255, 255, 0.3);
            border-radius: 8px;
            color: #ffffff;
            padding: 8px 16px;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.3s ease;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
            font-size: 14px;
            transform: translateY(0);
            &:hover {
                background: linear-gradient(45deg, #7c8ff0, #8b5bb8);
                border-color: rgba(255, 255, 255, 0.5);
                box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
                transform: translateY(-2px);
            }
            &:active {
                background: linear-gradient(45deg, #5a6fe8, #6a4196);
                border-color: rgba(255, 255, 255, 0.7);
                box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
                transform: translateY(0);
            }
        "#
    )
    .unwrap()
}

pub fn task_checkbox() -> Style {
    style!(
        r#"
            accent-color: #667eea;
            width: 18px;
            height: 18px;
            cursor: pointer;
        "#
    )
    .unwrap()
}

pub fn folder_input() -> Style {
    style!(
        r#"
            accent-color: #667eea;
            width: 18px;
            height: 18px;
            cursor: pointer;
        "#
    )
    .unwrap()
}
