// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{LogicalSize, Manager};

#[derive(Clone, Serialize, Deserialize)]
struct Settings {
    show_seconds: bool,
    opacity: f32,
    clock_size: u32,
}

fn parse_settings(settings: &str) -> Settings {
    let parsed = serde_json::from_str(settings);

    match parsed {
        Ok(parsed_settings) => parsed_settings,
        Err(_) => Settings {
            show_seconds: true,
            opacity: 1.0,
            clock_size: 400,
        },
    }
}

#[tauri::command]
fn set_settings(app: tauri::AppHandle, new_settings: &str) -> String {
    let parsed = parse_settings(new_settings);
    let serialized = serde_json::to_string(&parsed);
    let windget_window = app.get_window("widget").unwrap();

    windget_window
        .set_size(LogicalSize {
            width: parsed.clock_size,
            height: parsed.clock_size,
        })
        .unwrap();

    app.emit_all("settings-change", &parsed).unwrap();

    match serialized {
        Ok(serialized) => serialized.into(),
        Err(_) => "Error serializing settings".into(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
