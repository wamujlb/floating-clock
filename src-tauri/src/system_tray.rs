use tauri::{SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

pub fn app_system_tray() -> SystemTray {
    let settings_menu_item = CustomMenuItem::new("settings".to_string(), "Settings");
    let quit_menu_item = CustomMenuItem::new("quit".to_string(), "Quit");
    let menu: SystemTrayMenu = SystemTrayMenu::new()
        .add_item(settings_menu_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit_menu_item);

    SystemTray::new().with_menu(menu)
}
