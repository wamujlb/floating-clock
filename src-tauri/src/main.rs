// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod menu;
mod system_tray;
use menu::app_menu;
use system_tray::app_system_tray;
use serde::{Deserialize, Serialize};
use tauri::{LogicalSize, Manager, SystemTrayEvent, WindowBuilder, WindowMenuEvent, Wry};

#[derive(Serialize, Deserialize)]
enum ClockVariant {
    Flip,
    Digital,
    BinaryAnalog,
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
            show_pomodoro: true,
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

fn get_flip_logical_size(clock_size: u32, show_pomodoro: bool, show_seconds: bool) -> LogicalSize<u32> {
    let gap = round_to_nearest_even(clock_size as f32);
    LogicalSize {
        height: clock_size * 4 + gap * 3,
        width: match show_seconds {
            true => match show_pomodoro {
                true => clock_size * 14 + gap * 8,
                false => clock_size * 10 + gap * 6,
            },
            false => match show_pomodoro {
                true => clock_size * 12 + gap * 7,
                false => clock_size * 8 + gap * 5,
            },
        },
    }
}

fn get_digital_logical_size(clock_size: u32, show_pomodoro: bool, show_seconds: bool) -> LogicalSize<u32> {
    let gap = round_to_nearest_even(clock_size as f32);
    LogicalSize {
        height: clock_size * 4 + gap * 3,
        width: match show_seconds {
            true => match show_pomodoro {
                true => clock_size * 14 + gap * 5,
                false => clock_size * 10 + gap * 4,
            },
            false => match show_pomodoro {
                true => clock_size * 12 + gap * 4,
                false => clock_size * 8 + gap * 3,
            },
        },
    }
}

fn get_analog_logical_size(clock_size: u32, show_pomodoro: bool) -> LogicalSize<u32> {
    let gap = round_to_nearest_even(clock_size as f32);
    LogicalSize {
        height: clock_size * 4 + gap * 2,
        width: match show_pomodoro {
            true => clock_size * 8 + gap * 3,
            false => clock_size * 4 + gap * 2,
        },
    }
}

fn get_logical_size(settings: &Settings) -> LogicalSize<u32> {
    let clock_size = settings.clock_size;
    let show_pomodoro = settings.pomodoro.show_pomodoro;
    match settings.variant {
        ClockVariant::Flip => get_flip_logical_size(clock_size, show_pomodoro, settings.show_seconds),
        ClockVariant::Digital => get_digital_logical_size(clock_size, show_pomodoro, settings.show_seconds),
        ClockVariant::BinaryAnalog => get_analog_logical_size(clock_size, show_pomodoro),
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
                    .inner_size(400.0, 700.0)
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

fn system_tray_event_handler(app_handle: &tauri::AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "settings" => {
                    open_settings_window(app_handle.clone())
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn main() {
    let menu = app_menu();
    let system_tray = app_system_tray();

    tauri::Builder::default()
        .setup(|app| {
            open_settings_window(app.app_handle());

            Ok(())
        })
        .system_tray(system_tray)
        .on_system_tray_event(system_tray_event_handler)
        .menu(menu)
        .on_menu_event(menu_handler)
        .invoke_handler(tauri::generate_handler![set_settings, open_settings_window])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
