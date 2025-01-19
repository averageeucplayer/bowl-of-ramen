use std::error::Error;

use tauri::menu::{IsMenuItem, MenuBuilder, MenuItemBuilder, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{App, Wry};

use super::handle_menu_event::handle_menu_event;
use super::handle_tray_icon_event::handle_tray_icon_event;

pub fn setup_system_tray(app: &App) -> Result<(), Box<dyn Error>> {
    let menu_item_names: Vec<(&str, &str, bool)> = vec![
        ("show-logs", "Show Logs", true),
        ("show-meter", "Show Meter", false),
        ("hide", "Hide Meter", true),
        ("save", "Save Position", false),
        ("load", "Load Saved", false),
        ("reset", "Reset Window", true),
        ("quit", "Quit", false),
    ];
    let menu_builder = MenuBuilder::new(app);
    let mut menu_items: Vec<Box<dyn IsMenuItem<Wry>>> = Vec::new();

    for (id, title, put_separator) in menu_item_names {
        let menu_item = MenuItemBuilder::new(title).id(id).build(app).unwrap();

        let boxed_menu_item: Box<dyn IsMenuItem<Wry>> = Box::new(menu_item);
        menu_items.push(boxed_menu_item);

        if put_separator {
            let separator = PredefinedMenuItem::separator(app).unwrap();
            let boxed_separator: Box<dyn IsMenuItem<Wry>> = Box::new(separator);
            menu_items.push(boxed_separator);
        }
    }

    let converted_menu_items = menu_items
        .iter()
        .map(|item| item.as_ref())
        .collect::<Vec<_>>();

    let menu = menu_builder
        .items(&converted_menu_items)
        .separator()
        .build()
        .unwrap();

    TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(handle_menu_event)
        .on_tray_icon_event(handle_tray_icon_event)
        .build(app)
        .unwrap();

    Ok(())
}