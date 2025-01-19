use tauri::tray::{TrayIcon, TrayIconEvent};
use tauri::Runtime;

use crate::abstractions::AppHandle;

pub fn handle_tray_icon_event<R: Runtime>(icon: &TrayIcon<R>, event: TrayIconEvent) {
    let app_handle: &dyn AppHandle = icon.app_handle();
    handle_tray_icon_event_inner(app_handle, event);
}

fn handle_tray_icon_event_inner(app_handle: &dyn AppHandle, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click { .. } => {
            let meter_window = app_handle.get_window("main").unwrap();
            meter_window.show().unwrap();
            meter_window.unminimize().unwrap();
            meter_window.set_ignore_cursor_events(false).unwrap();
        },
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
    use tauri::{tray::{MouseButton, MouseButtonState, TrayIconId}, LogicalPosition, LogicalSize, PhysicalPosition, Position, Rect, Size, State};

    #[test]
    fn test_tray_icon_event() {
        let event = TrayIconEvent::Click {
            id: TrayIconId("test".to_string()),
            position: PhysicalPosition{
                x: 0.0,
                y: 0.0,
            },
            rect: Rect{
                position: Position::Logical(LogicalPosition { 
                    x: 100.0,
                    y: 100.0
                }),
                size: Size::Logical(LogicalSize {
                    width: 500.0,
                    height: 350.0,
                })
            },
            button: MouseButton::Left,
            button_state: MouseButtonState::Down
        };

        let mut mock_app_handle = MockAppHandle::new();

        mock_app_handle
            .expect_get_window()
            .with(eq("main"))
            .returning(move |_| mock_meter_window());

        let app: &dyn AppHandle = &mock_app_handle;

        handle_tray_icon_event_inner(app, event);
    }

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
}
