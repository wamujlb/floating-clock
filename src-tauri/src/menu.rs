use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn app_menu() -> Menu {
    let settings_menu_item = CustomMenuItem::new("settings".to_string(), "Settings");
    let quit_menu_item = CustomMenuItem::new("quit".to_string(), "Quit");
    let submenu = Submenu::new(
        "Floating Clock",
        Menu::new()
            .add_item(settings_menu_item)
            .add_native_item(MenuItem::Separator)
            .add_item(quit_menu_item),
    );
    Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu)
}
