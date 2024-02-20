// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{LogicalSize, Manager};

#[derive(Serialize, Deserialize)]
enum ClockVariant {
    Flip,
    Digital,
}

#[derive(Serialize, Deserialize)]
struct Settings {
    show_seconds: bool,
    opacity: f32,
    clock_size: u32,
    variant: ClockVariant,
}

impl Settings {
    fn new() -> Self {
        Settings {
            show_seconds: true,
            opacity: 1.0,
            clock_size: 400,
            variant: ClockVariant::Flip,
        }
    }

    fn from_json(json: &str) -> Self {
        let parsed = serde_json::from_str(json);

        match parsed {
            Ok(parsed_settings) => parsed_settings,
            Err(_) => Settings::new(),
        }
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[tauri::command]
fn set_settings(app: tauri::AppHandle, new_settings: &str) -> String {
    let settings = Settings::from_json(new_settings);
    let serialized = settings.to_json();
    let widget_window = app.get_window("widget").unwrap();

    widget_window
        .set_size(LogicalSize {
            width: settings.clock_size,
            height: settings.clock_size,
        })
        .unwrap();

    app.emit_all("settings-change", &settings).unwrap();

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
