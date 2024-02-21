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

fn get_flip_clock_width(clock_size: f32, show_seconds: bool) -> u32 {
    let gap = clock_size / 10.0;
    match show_seconds {
        true => (clock_size * 2.5 + gap * 2.0).ceil() as u32,
        false => (clock_size * 2.0 + gap).ceil() as u32,
    }
}

fn get_logical_size(settings: &Settings) -> LogicalSize<u32> {
    let clock_size = settings.clock_size as f32;
    let show_seconds = settings.show_seconds;
    match settings.variant {
        ClockVariant::Flip => LogicalSize {
            height: clock_size as u32,
            width: get_flip_clock_width(clock_size, show_seconds),
        },
        ClockVariant::Digital => LogicalSize {
            height: clock_size as u32,
            width: match show_seconds {
                true => (clock_size * 2.5).ceil() as u32,
                false => (clock_size * 2.0).ceil() as u32,
            },
        },
    }
}

#[tauri::command]
fn set_settings(app: tauri::AppHandle, new_settings: &str) -> String {
    let settings = Settings::from_json(new_settings);
    let serialized = settings.to_json();
    let widget_window = app.get_window("widget").unwrap();

    widget_window.set_size(get_logical_size(&settings)).unwrap();

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
