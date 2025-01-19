use tauri::menu::MenuEvent;
use tauri::{LogicalPosition, LogicalSize, Manager, Position, Runtime, Size};

use crate::abstractions::AppHandle;
use crate::constants::WINDOW_STATE_FLAGS;


pub fn handle_menu_event<R: Runtime>(app: &tauri::AppHandle<R>, event: MenuEvent) {
    let menu_item_id = event.id().0.as_str();
    let app_handle: &dyn AppHandle = app.app_handle();
    handle_menu_event_inner(app_handle, menu_item_id);
}

pub fn handle_menu_event_inner(app_handle: &dyn AppHandle, menu_item_id: &str){

    match menu_item_id {
        "quit" => {
            app_handle.save_window_state(WINDOW_STATE_FLAGS).unwrap();
            app_handle.exit();
        }
        "hide" => {
            let meter_window = app_handle.get_window("main").unwrap();
            meter_window.hide().unwrap();
        }
        "show-meter" => {
            let meter_window = app_handle.get_window("main").unwrap();
            meter_window.show().unwrap();
            meter_window.unminimize().unwrap();
            meter_window.set_ignore_cursor_events(false).unwrap();
        }
        "load" => {
            let meter_window = app_handle.get_window("main").unwrap();
            meter_window.restore_state(WINDOW_STATE_FLAGS).unwrap();
        }
        "save" => {
            let meter_window = app_handle.get_window("main").unwrap();
            meter_window.save_window_state(WINDOW_STATE_FLAGS).unwrap();
        }
        "reset" => {
            let meter_window = app_handle.get_window("main").unwrap();
            let size = Size::Logical(LogicalSize {
                width: 500.0,
                height: 350.0,
            });
            meter_window.set_size(size).unwrap();
            let position = Position::Logical(LogicalPosition { 
                x: 100.0,
                y: 100.0
            });
            meter_window.set_position(position).unwrap();
            meter_window.show().unwrap();
            meter_window.unminimize().unwrap();
            meter_window.set_focus().unwrap();
            meter_window.set_ignore_cursor_events(false).unwrap();
        }
        "show-logs" => {
            let logs_window = app_handle.get_window("logs").unwrap();
            logs_window.show().unwrap();
            logs_window.unminimize().unwrap();
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{abstractions::{AppHandle, AppWindow}, mocks::{MockAppHandle, MockAppWindow}};
    use mockall::{
        mock,
        predicate::{self, *},
    };

    #[test]
    fn test_handle_menu_event_quit() {

        let mut mock_app_handle = MockAppHandle::new();

        mock_app_handle
            .expect_save_window_state()
            .with(always())
            .returning(|_| Ok(()));

        mock_app_handle
            .expect_exit()
            .times(1)
            .returning(|| ());

        let app: &dyn AppHandle = &mock_app_handle;

        handle_menu_event_inner(app, "quit");
    }

    #[test]
    fn test_handle_menu_event_hide() {

        let mut mock_app_handle = MockAppHandle::new();

        mock_app_handle
            .expect_get_window()
            .with(eq("main"))
            .returning(move |_| mock_meter_window());

        fn mock_meter_window() -> Option<Box<dyn AppWindow>> {
            let mut mock_meter_window = MockAppWindow::new();

            mock_meter_window
                .expect_hide()
                .returning(|| Ok(()));

            Some(Box::new(mock_meter_window) as Box<dyn AppWindow>)
        }

        let app: &dyn AppHandle = &mock_app_handle;

        handle_menu_event_inner(app, "hide");
    }

    #[test]
    fn test_handle_menu_event_show_meter() {
        let mut mock_app_handle = MockAppHandle::new();

        mock_app_handle
            .expect_get_window()
            .with(eq("main"))
            .returning(move |_| mock_meter_window());

        fn mock_meter_window() -> Option<Box<dyn AppWindow>> {
            let mut mock_meter_window = MockAppWindow::new();

            mock_meter_window
                .expect_show()
                .returning(|| Ok(()));

            mock_meter_window
                .expect_unminimize()
                .returning(|| Ok(()));

            mock_meter_window
                .expect_set_ignore_cursor_events()
                .with(eq(false))
                .returning(|_|Ok(()));

            Some(Box::new(mock_meter_window) as Box<dyn AppWindow>)
        }

        let app: &dyn AppHandle = &mock_app_handle;

        handle_menu_event_inner(app, "show-meter");
    }

    #[test]
    fn test_handle_menu_event_load() {
        let mut mock_app_handle = MockAppHandle::new();
        
        mock_app_handle
            .expect_get_window()
            .with(eq("main"))
            .returning(move |_| mock_meter_window());

        fn mock_meter_window() -> Option<Box<dyn AppWindow>> {
            let mut mock_meter_window = MockAppWindow::new();

            mock_meter_window
                .expect_restore_state()
                .with(always())
                .returning(|_| Ok(()));

            Some(Box::new(mock_meter_window) as Box<dyn AppWindow>)
        }
        
        let app: &dyn AppHandle = &mock_app_handle;

        handle_menu_event_inner(app, "load");
    }
    
    #[test]
    fn test_handle_menu_event_save() {
        let mut mock_app_handle = MockAppHandle::new();
        
        mock_app_handle
            .expect_get_window()
            .with(eq("main"))
            .returning(move |_| mock_meter_window());

        fn mock_meter_window() -> Option<Box<dyn AppWindow>> {
            let mut mock_meter_window = MockAppWindow::new();

            mock_meter_window
                .expect_save_window_state()
                .with(always())
                .returning(|_| Ok(()));

            Some(Box::new(mock_meter_window) as Box<dyn AppWindow>)
        }

        let app: &dyn AppHandle = &mock_app_handle;

        handle_menu_event_inner(app, "save");
    }

    #[test]
    fn test_handle_menu_event_reset() {
        let mut mock_app_handle = MockAppHandle::new();
        
        mock_app_handle
            .expect_get_window()
            .with(eq("main"))
            .returning(move |_| mock_meter_window());

        fn mock_meter_window() -> Option<Box<dyn AppWindow>> {
            let mut mock_meter_window = MockAppWindow::new();

            mock_meter_window
                .expect_set_size()
                .with(always())
                .returning(|_| Ok(()));

            mock_meter_window
                .expect_set_position()
                .with(always())
                .returning(|_| Ok(()));

            mock_meter_window
                .expect_show()
                .returning(|| Ok(()));

            mock_meter_window
                .expect_unminimize()
                .returning(|| Ok(()));

            mock_meter_window
                .expect_set_focus()
                .returning(|| Ok(()));

            mock_meter_window
                .expect_set_ignore_cursor_events()
                .with(eq(false))
                .returning(|_|Ok(()));

            Some(Box::new(mock_meter_window) as Box<dyn AppWindow>)
        }

        let app: &dyn AppHandle = &mock_app_handle;

        handle_menu_event_inner(app, "reset");
    }

    #[test]
    fn test_handle_menu_event_show_logs() {
        let mut mock_app_handle = MockAppHandle::new();
        
        mock_app_handle
            .expect_get_window()
            .with(eq("logs"))
            .returning(move |_| mock_logs_window());

        fn mock_logs_window() -> Option<Box<dyn AppWindow>> {
            let mut mock_logs_window = MockAppWindow::new();

            mock_logs_window
                .expect_show()
                .returning(|| Ok(()));

            mock_logs_window
                .expect_unminimize()
                .returning(|| Ok(()));

            Some(Box::new(mock_logs_window) as Box<dyn AppWindow>)
        }

        let app: &dyn AppHandle = &mock_app_handle;

        handle_menu_event_inner(app, "show-logs");
    }
}
