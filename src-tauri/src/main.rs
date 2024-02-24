// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu;
use menu::app_menu;
use serde::{Deserialize, Serialize};
use tauri::{LogicalSize, Manager, WindowBuilder, WindowMenuEvent, Wry};

#[derive(Serialize, Deserialize)]
enum ClockVariant {
    Flip,
    Digital,
}

#[derive(Serialize, Deserialize)]
struct PomodoroSettings {
    show_pomodoro: bool,
    interval: u32,
    focus_time: u32,
}

impl PomodoroSettings {
    fn new() -> Self {
        PomodoroSettings {
            show_pomodoro: false,
            interval: 30,
            focus_time: 25,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Settings {
    show_seconds: bool,
    opacity: f32,
    clock_size: u32,
    variant: ClockVariant,
    pomodoro: PomodoroSettings,
}

impl Settings {
    fn new() -> Self {
        Settings {
            show_seconds: true,
            opacity: 1.0,
            clock_size: 50,
            variant: ClockVariant::Flip,
            pomodoro: PomodoroSettings::new(),
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

fn round_to_nearest_even(num: f32) -> u32 {
    ((num * 0.4 / 2.0).round() * 2.0) as u32
}

fn get_flip_logical_size(clock_size: u32, show_seconds: bool) -> LogicalSize<u32> {
    let gap = round_to_nearest_even(clock_size as f32);
    LogicalSize {
        height: clock_size * 4 + gap * 3,
        width: match show_seconds {
            true => clock_size * 10 + gap * 6,
            false => clock_size * 8 + gap * 5,
        },
    }
}

fn get_digital_logical_size(clock_size: u32, show_seconds: bool) -> LogicalSize<u32> {
    let gap = round_to_nearest_even(clock_size as f32);
    LogicalSize {
        height: clock_size * 4 + gap * 3,
        width: match show_seconds {
            true => clock_size * 10 + gap * 4,
            false => clock_size * 8 + gap * 3,
        },
    }
}

fn get_logical_size(settings: &Settings) -> LogicalSize<u32> {
    let clock_size = settings.clock_size;
    let show_seconds = settings.show_seconds;
    match settings.variant {
        ClockVariant::Flip => get_flip_logical_size(clock_size, show_seconds),
        ClockVariant::Digital => get_digital_logical_size(clock_size, show_seconds),
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

#[tauri::command]
fn open_settings_window(app: tauri::AppHandle) {
    let settings_window = app.get_window("settings");

    match settings_window {
        Some(settings_window) => {
            settings_window.show().expect("Failed to show window.");
        }
        None => {
            let settings_window =
                WindowBuilder::new(&app, "settings", tauri::WindowUrl::App("/settings".into()))
                    .title("Settings")
                    .inner_size(400.0, 600.0)
                    .resizable(false)
                    .center()
                    .closable(true)
                    .minimizable(false)
                    .always_on_top(true)
                    .build()
                    .expect("Failed to create window.");
            settings_window.show().expect("Failed to show window.");
            settings_window
                .set_focus()
                .expect("Failed to focus window.");
        }
    }
}

fn menu_handler(event: WindowMenuEvent<Wry>) {
    match event.menu_item_id() {
        "quit" => {
            std::process::exit(0);
        }
        "settings" => {
            let window = event.window();
            let app_handle = window.app_handle();
            open_settings_window(app_handle)
        }
        _ => {}
    }
}

fn main() {
    let menu = app_menu();

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(menu_handler)
        .invoke_handler(tauri::generate_handler![set_settings, open_settings_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
