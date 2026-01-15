use stylist::StyleSource;

use super::lib::*;

pub fn global_style_for_theme(theme: &str) -> StyleSource {
    match theme {
        "dark" => css!(
            r#"
                body {
                    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
                    color: #ffffff;
                    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                    min-height: 100vh;
                    margin: 0;
                    padding: 20px;
                    padding-bottom: 100px;
                }
                @media (max-width: 768px) {
                    body {
                        padding: 15px;
                        padding-bottom: 70px;
                        font-size: 16px;
                    }
                }
                @media (max-width: 480px) {
                    body {
                        padding: 10px;
                        padding-bottom: 200px;
                        font-size: 14px;
                    }
                }
            "#
        ),
        "light" => css!(
            r#"
                body {
                    background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
                    color: #333333 !important;
                    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                    min-height: 100vh;
                    margin: 0;
                    padding: 20px;
                    padding-bottom: 80px;
                }
                @media (max-width: 768px) {
                    body {
                        padding: 15px;
                        padding-bottom: 65px;
                        font-size: 16px;
                    }
                }
                @media (max-width: 480px) {
                    body {
                        padding: 10px;
                        padding-bottom: 200px;
                        font-size: 14px;
                    }
                }
                h2 {
                    color: #333333 !important;
                }
                .task-input {
                    color: #333333 !important;
                    background: rgba(255, 255, 255, 0.8) !important;
                    border: 1px solid rgba(0, 0, 0, 0.3) !important;
                }
                .task-input::placeholder {
                    color: rgba(0, 0, 0, 0.5) !important;
                }
                .task-input:focus {
                    border-color: rgba(0, 0, 0, 0.5) !important;
                    background: rgba(255, 255, 255, 0.9) !important;
                }
                .div-list {
                    background: rgba(255, 255, 255, 0.8) !important;
                    border: 1px solid rgba(0, 0, 0, 0.2) !important;
                }
                .div-list label {
                    color: #333333 !important;
                }
                /* Keep input container and settings elements white */
                .input-container {
                    background: rgba(255, 255, 255, 0.9) !important;
                    border-top: 1px solid rgba(0, 0, 0, 0.2) !important;
                }
                .input-container * {
                    color: #ffffff !important;
                }
                .settings-overlay * {
                    color: #ffffff !important;
                }
                .settings-modal * {
                    color: #ffffff !important;
                }
                /* Override any white text in light theme for main content */
                body > div:not(.input-container):not(.settings-overlay) label, 
                body > div:not(.input-container):not(.settings-overlay) span, 
                body > div:not(.input-container):not(.settings-overlay) div:not(.input-container):not(.settings-overlay):not(.settings-modal) {
                    color: #333333 !important;
                }
            "#
        ),
        _ => css!(
            r#"
                body {
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    color: #ffffff;
                    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                    min-height: 100vh;
                    margin: 0;
                    padding: 20px;
                    padding-bottom: 80px;
                }
                @media (max-width: 768px) {
                    body {
                        padding: 15px;
                        padding-bottom: 65px;
                        font-size: 16px;
                    }
                }
                @media (max-width: 480px) {
                    body {
                        padding: 10px;
                        padding-bottom: 200px;
                        font-size: 14px;
                    }
                }
            "#
        ),
    }
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
            @media (max-width: 768px) {
                font-size: 2rem;
                margin-bottom: 25px;
                margin-top: 10px;
            }
            @media (max-width: 480px) {
                font-size: 1.5rem;
                margin-bottom: 20px;
                margin-top: 5px;
                line-height: 1.2;
            }
        "#
    )
    .unwrap()
}

pub fn input_container() -> Style {
    style!(
        r#"
            position: fixed;
            bottom: 0;
            left: 0;
            right: 0;
            background: rgba(0, 0, 0, 0.8);
            backdrop-filter: blur(10px);
            border-top: 1px solid rgba(255, 255, 255, 0.2);
            padding: 15px;
            display: flex;
            justify-content: space-between;
            align-items: center;
            gap: 8px;
            z-index: 1000;
            flex-wrap: nowrap;
            @media (max-width: 768px) {
                padding: 12px;
                gap: 6px;
            }
            @media (max-width: 480px) {
                padding: 10px;
                gap: 4px;
                flex-direction: column;
                align-items: stretch;
                min-height: auto;
            }
        "#
    )
    .unwrap()
}

pub fn undo_redo_container() -> Style {
    style!(
        r#"
            display: flex;
            gap: 4px;
            align-items: center;
            @media (max-width: 480px) {
                justify-content: space-between;
                width: 100%;
                margin-bottom: 3px;
            }
        "#
    )
    .unwrap()
}

pub fn undo_redo_button() -> Style {
    style!(
        r#"
            background: linear-gradient(45deg, #28a745, #20c997);
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
            flex-shrink: 0;
            white-space: nowrap;
            &:hover {
                background: linear-gradient(45deg, #34b853, #2dd4a0);
                border-color: rgba(255, 255, 255, 0.5);
                box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
                transform: translateY(-2px);
            }
            &:active {
                background: linear-gradient(45deg, #218838, #1ea085);
                border-color: rgba(255, 255, 255, 0.7);
                box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
                transform: translateY(0);
            }
            @media (max-width: 768px) {
                padding: 3px 5px;
                font-size: 9px;
            }
            @media (max-width: 480px) {
                padding: 6px;
                font-size: 12px;
                width: 49%;
                margin-bottom: 3px;
            }
        "#
    )
    .unwrap()
}

pub fn action_buttons_container() -> Style {
    style!(
        r#"
            display: flex;
            gap: 6px;
            align-items: center;
            @media (max-width: 480px) {
                flex-direction: row;
                width: 100%;
                margin-bottom: 3px;
                gap: 4px;
            }
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
                font-size: 22px;
                font-weight: 400;
                color: #ffffff;
            }
            &[data-deep="true"] label {
                text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.8);
                -webkit-text-stroke: 1px rgba(0, 0, 0, 0.5);
                text-stroke: 1px rgba(0, 0, 0, 0.5);
            }
            @media (max-width: 768px) {
                margin: 8px 0;
                padding: 12px;
                padding-left: 15px;
                label {
                    font-size: 18px;
                }
            }
            @media (max-width: 480px) {
                margin: 5px 0;
                padding: 10px;
                padding-left: 12px;
                border-radius: 10px;
                label {
                    font-size: 16px;
                    line-height: 1.3;
                }
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
            flex: 1;
            min-width: 0;
            padding: 8px 12px;
            font-size: 14px;
            color: #ffffff;
            background: rgba(255, 255, 255, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.3);
            border-radius: 8px;
            backdrop-filter: blur(5px);
            transition: all 0.3s ease;
            ::placeholder {
                color: rgba(255, 255, 255, 0.7);
            }
            &:focus {
                outline: none;
                border-color: rgba(255, 255, 255, 0.5);
                background: rgba(255, 255, 255, 0.2);
            }
            @media (max-width: 768px) {
                padding: 6px 10px;
                font-size: 13px;
            }
            @media (max-width: 480px) {
                padding: 10px;
                font-size: 14px;
                margin-bottom: 4px;
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
            flex-shrink: 0;
            white-space: nowrap;
            min-height: 30px;
            display: flex;
            align-items: center;
            justify-content: center;
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
            @media (max-width: 768px) {
                padding: 5px 10px;
                font-size: 11px;
            }
            @media (max-width: 480px) {
                padding: 6px;
                font-size: 12px;
                flex: 1;
                margin-bottom: 0;
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

pub fn settings_button() -> Style {
    style!(
        r#"
            background: linear-gradient(45deg, #6c757d, #495057);
            border: 2px solid rgba(255, 255, 255, 0.3);
            border-radius: 8px;
            color: #ffffff;
            padding: 8px 8px;
            font-size: 16px;
            cursor: pointer;
            transition: all 0.3s ease;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
            transform: translateY(0);
            flex-shrink: 0;
            min-height: 30px;
            display: flex;
            align-items: center;
            justify-content: center;
            &:hover {
                background: linear-gradient(45deg, #7d858a, #5a6168);
                border-color: rgba(255, 255, 255, 0.5);
                box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
                transform: translateY(-2px);
            }
            &:active {
                background: linear-gradient(45deg, #5a6268, #495057);
                border-color: rgba(255, 255, 255, 0.7);
                box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
                transform: translateY(0);
            }
            @media (max-width: 768px) {
                padding: 6px 8px;
                font-size: 12px;
                min-height: 28px;
            }
            @media (max-width: 480px) {
                padding: 8px;
                font-size: 14px;
                width: 10%;
                margin-bottom: 3px;
                min-height: 32px;
                aspect-ratio: 1;
            }
        "#
    )
    .unwrap()
}

pub fn settings_overlay() -> Style {
    style!(
        r#"
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0, 0, 0, 0.7);
            backdrop-filter: blur(5px);
            z-index: 2000;
            display: flex;
            justify-content: center;
            align-items: center;
            animation: fadeIn 0.3s ease;
            @keyframes fadeIn {
                from { opacity: 0; }
                to { opacity: 1; }
            }
        "#
    )
    .unwrap()
}

pub fn settings_modal() -> Style {
    style!(
        r#"
            background: rgba(0, 0, 0, 0.9);
            backdrop-filter: blur(20px);
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 15px;
            padding: 0;
            max-width: 500px;
            width: 90%;
            max-height: 80vh;
            overflow: hidden;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
            animation: slideIn 0.3s ease;
            @keyframes slideIn {
                from { 
                    opacity: 0;
                    transform: translateY(-20px);
                }
                to { 
                    opacity: 1;
                    transform: translateY(0);
                }
            }
            @media (max-width: 768px) {
                width: 95%;
                max-height: 85vh;
                border-radius: 10px;
            }
            @media (max-width: 480px) {
                width: 98%;
                max-height: 90vh;
                border-radius: 8px;
                margin: 10px;
            }
        "#
    )
    .unwrap()
}

pub fn settings_header() -> Style {
    style!(
        r#"
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 20px 25px;
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
            h3 {
                margin: 0;
                color: #ffffff;
                font-size: 1.5rem;
                font-weight: 400;
            }
            @media (max-width: 768px) {
                padding: 15px 20px;
                h3 {
                    font-size: 1.3rem;
                }
            }
            @media (max-width: 480px) {
                padding: 12px 15px;
                h3 {
                    font-size: 1.1rem;
                }
            }
        "#
    )
    .unwrap()
}

pub fn close_button() -> Style {
    style!(
        r#"
            background: transparent;
            border: none;
            color: rgba(255, 255, 255, 0.7);
            font-size: 20px;
            cursor: pointer;
            padding: 5px;
            border-radius: 5px;
            transition: all 0.2s ease;
            &:hover {
                color: #ffffff;
                background: rgba(255, 255, 255, 0.1);
            }
        "#
    )
    .unwrap()
}

pub fn settings_content() -> Style {
    style!(
        r#"
            padding: 25px;
            overflow-y: auto;
            max-height: calc(80vh - 80px);
            @media (max-width: 768px) {
                padding: 20px;
                max-height: calc(85vh - 70px);
            }
            @media (max-width: 480px) {
                padding: 15px;
                max-height: calc(90vh - 60px);
            }
        "#
    )
    .unwrap()
}

pub fn settings_section() -> Style {
    style!(
        r#"
            margin-bottom: 30px;
            h4 {
                margin: 0 0 15px 0;
                color: #ffffff;
                font-size: 1.2rem;
                font-weight: 400;
                opacity: 0.9;
            }
        "#
    )
    .unwrap()
}

pub fn theme_options() -> Style {
    style!(
        r#"
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            @media (max-width: 480px) {
                flex-direction: column;
                gap: 8px;
            }
        "#
    )
    .unwrap()
}

pub fn theme_button() -> Style {
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
            @media (max-width: 768px) {
                padding: 6px 12px;
                font-size: 12px;
            }
            @media (max-width: 480px) {
                padding: 10px;
                font-size: 14px;
                width: 100%;
            }
        "#
    )
    .unwrap()
}

pub fn github_link() -> Style {
    style!(
        r#"
            color: #667eea;
            text-decoration: none;
            font-weight: 500;
            transition: all 0.3s ease;
            display: inline-block;
            padding: 8px 12px;
            border-radius: 6px;
            background: rgba(102, 126, 234, 0.1);
            &:hover {
                color: #7c8ff0;
                background: rgba(102, 126, 234, 0.2);
                text-decoration: underline;
            }
        "#
    )
    .unwrap()
}
