use tauri::Manager;
use tauri::{Window, WindowEvent};

use crate::abstractions::window_event::{CloseRequestApiLike, CustomWindowEvent};
use crate::abstractions::{AppHandle, AppWindow};
use crate::constants::WINDOW_STATE_FLAGS;

pub fn on_window_event(window: &Window, event: &WindowEvent) {
    let label = window.label();
    let app_handle: &dyn AppHandle = window.app_handle();
    let app_window: &dyn AppWindow = window;
    let event_wrapper = &CustomWindowEvent::Real(event.clone());
    on_window_event_inner(
        app_handle,
        app_window,
        label,
        event_wrapper);
}

pub fn handle_close_requested(
    api: &dyn CloseRequestApiLike,
    app_handle: &dyn AppHandle,
    window: &dyn AppWindow,
    label: &str) {
    api.prevent_close();
    
    match label {
        "main" => {
            let meter_window = window;
            let logs_window = app_handle.get_window("logs").unwrap();
            let is_logs_minimized = logs_window.is_minimized().unwrap();
            let is_meter_minimized = meter_window.is_minimized().unwrap();

            if is_logs_minimized {
                logs_window.unminimize().unwrap();
            }

            if is_meter_minimized {
                meter_window.unminimize().unwrap();
            }

            app_handle
                .save_window_state(WINDOW_STATE_FLAGS)
                .expect("failed to save window state");
            app_handle.exit();
        }
        "logs" => {
            window.hide().unwrap();
        }
        _ => {}
    }
}

pub fn on_window_event_inner(
    app_handle: &dyn AppHandle,
    window: &dyn AppWindow,
    label: &str,
    event: &CustomWindowEvent) {
    match event {
        CustomWindowEvent::Real(real_event) => match real_event {
            WindowEvent::CloseRequested { api, .. } => handle_close_requested(
                api,
                app_handle,
                window,
                label),
            WindowEvent::Focused(_) => {
                app_handle
                    .save_window_state(WINDOW_STATE_FLAGS)
                    .expect("");
            },
            _ => {}
        },
        CustomWindowEvent::FakeCloseRequested { api } => handle_close_requested(
            api.as_ref(),
            app_handle,
            window,
            label),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::{MockAppWindow, MockAppHandle};
    use super::*;
    use mockall::{
        mock,
        predicate::{self, *},
    };

    pub struct FakeCloseRequestApi;

    impl CloseRequestApiLike for FakeCloseRequestApi {
        fn prevent_close(&self) {}
    }

    #[test]
    fn test_on_window_event_close_meter() {

        let mut mock_app_handle = MockAppHandle::new();

        mock_app_handle
            .expect_get_window()
            .with(eq("logs"))
            .returning(move |_| mock_logs_window());

        mock_app_handle
            .expect_save_window_state()
            .with(always())
            .returning(move |_| Ok(()));

        mock_app_handle
            .expect_exit()
            .times(1)
            .returning(|| ());

        let mut mock_app_window = MockAppWindow::new();
        
        mock_app_window
            .expect_is_minimized()
            .returning(move || Ok(true));

        mock_app_window
            .expect_unminimize()
            .returning(move || Ok(()));

        fn mock_logs_window() -> Option<Box<dyn AppWindow>> {
            let mut mock_logs_window = MockAppWindow::new();

            mock_logs_window
                .expect_is_minimized()
                .returning(move || Ok(true));

            mock_logs_window
                .expect_unminimize()
                .returning(move || Ok(()));

            Some(Box::new(mock_logs_window) as Box<dyn AppWindow>)
        }

        let app: &dyn AppHandle = &mock_app_handle;
        let app_window: &dyn AppWindow = &mock_app_window;
        let event = CustomWindowEvent::FakeCloseRequested { api: Box::new(FakeCloseRequestApi{}) };

        on_window_event_inner(app, app_window, "main", &event);
    }

    #[test]
    fn test_on_window_event_close_logs() {
        let mock_app_handle = MockAppHandle::new();
        let mut mock_app_window = MockAppWindow::new();
        
        mock_app_window
            .expect_hide()
            .returning(move || Ok(()));

        let app: &dyn AppHandle = &mock_app_handle;
        let app_window: &dyn AppWindow = &mock_app_window;
        let event = CustomWindowEvent::FakeCloseRequested { api: Box::new(FakeCloseRequestApi{}) };

        on_window_event_inner(app, app_window, "logs", &event);
    }

    #[test]
    fn test_on_window_event_focused() {

        let mut mock_app_handle = MockAppHandle::new();
        
        mock_app_handle
            .expect_save_window_state()
            .with(always())
            .returning(move |_| Ok(()));

        let mock_app_window = MockAppWindow::new();
        
        let app_window: &dyn AppWindow = &mock_app_window;
        let app: &dyn AppHandle = &mock_app_handle;

        let event = CustomWindowEvent::Real(WindowEvent::Focused(false));
        on_window_event_inner(app, app_window, "logs", &event);

    }
}
