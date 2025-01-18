use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use raw_window_handle::HasWindowHandle;
use tauri::{Manager, PackageInfo, Position, Runtime, Size, WebviewWindow, Window};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

pub trait AppWindow: Send + Sync + 'static {
    fn as_has_window_handle<'a>(&'a self) -> Arc<Mutex<dyn HasWindowHandle + 'a>>;
    fn get_window(&self, str: &str) -> Option<Box<dyn AppWindow>>;
    fn set_always_on_top(&self, value: bool) -> Result<(), Box<dyn Error>>;
    fn save_window_state(&self, flags: StateFlags) -> Result<(), Box<dyn Error>>;
    fn is_visible(&self) -> Result<bool, Box<dyn Error>>;
    fn hide(&self) -> Result<(), Box<dyn Error>>;
    fn show(&self) -> Result<(), Box<dyn Error>>;
    fn is_minimized(&self) -> Result<bool, Box<dyn Error>>;
    fn unminimize(&self) -> Result<(), Box<dyn Error>>;
    fn get_package_info(&self) -> &PackageInfo;
    fn set_ignore_cursor_events(&self, ignore: bool) -> Result<(), Box<dyn Error>>;
    fn set_size(&self, size: Size) -> Result<(), Box<dyn Error>>;
    fn set_position(&self, position: Position) -> Result<(), Box<dyn Error>>;
    fn set_focus(&self) -> Result<(), Box<dyn Error>>;
    fn restore_state(&self, flags: StateFlags) -> Result<(), Box<dyn Error>>;
}

impl<R: Runtime> AppWindow for WebviewWindow<R> {
   
    fn as_has_window_handle<'a>(&'a self) -> Arc<Mutex<dyn HasWindowHandle + 'a>> {
        Arc::new(Mutex::new(self))
    }

    fn get_window(&self, _label: &str) -> Option<Box<dyn AppWindow>> {
        None
    }

    fn set_always_on_top(&self, value: bool) -> Result<(), Box<dyn Error>> {
        self.set_always_on_top(value).unwrap();
        Ok(())
    }

    fn save_window_state(&self, flags: StateFlags) -> Result<(), Box<dyn Error>> {
        AppHandleExt::save_window_state(self.app_handle(), flags).unwrap();
        Ok(())
    }

    fn is_visible(&self) -> Result<bool, Box<dyn Error>> {
        self.is_visible()
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn hide(&self) -> Result<(), Box<dyn Error>> {
        self.hide().map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn show(&self) -> Result<(), Box<dyn Error>> {
        self.show().map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn is_minimized(&self) -> Result<bool, Box<dyn Error>> {
        self.is_minimized()
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn unminimize(&self) -> Result<(), Box<dyn Error>> {
        self.unminimize()
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn get_package_info(&self) -> &PackageInfo {
        self.package_info()
    }

    fn set_ignore_cursor_events(&self, ignore: bool) -> Result<(), Box<dyn Error>> {
        self.set_ignore_cursor_events(ignore).map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn set_size(&self, size: Size) -> Result<(), Box<dyn Error>> {
        self.set_size(size).map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn set_position(&self, position: Position) -> Result<(), Box<dyn Error>> {
        self.set_position(position).map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn set_focus(&self) -> Result<(), Box<dyn Error>> {
        self.set_focus().map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn restore_state(&self, flags: StateFlags) -> Result<(), Box<dyn Error>> {
        WindowExt::restore_state(self, flags).map_err(|err| Box::new(err) as Box<dyn Error>)
    }
}

impl<R: Runtime> AppWindow for Window<R> {
    fn as_has_window_handle<'a>(&'a self) -> Arc<Mutex<dyn HasWindowHandle + 'a>> {
        Arc::new(Mutex::new(self))
    }

    fn get_window(&self, label: &str) -> Option<Box<dyn AppWindow>> {
        self.get_webview_window(label)
            .map(|window| Box::new(window) as Box<dyn AppWindow>)
    }

    fn set_always_on_top(&self, value: bool) -> Result<(), Box<dyn Error>> {
        self.set_always_on_top(value).unwrap();
        Ok(())
    }

    fn save_window_state(&self, flags: StateFlags) -> Result<(), Box<dyn Error>> {
        AppHandleExt::save_window_state(self.app_handle(), flags).unwrap();
        Ok(())
    }

    fn is_visible(&self) -> Result<bool, Box<dyn Error>> {
        self.is_visible()
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn hide(&self) -> Result<(), Box<dyn Error>> {
        self.hide().map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn show(&self) -> Result<(), Box<dyn Error>> {
        self.show().map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn is_minimized(&self) -> Result<bool, Box<dyn Error>> {
        self.is_minimized()
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn unminimize(&self) -> Result<(), Box<dyn Error>> {
        self.unminimize()
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn get_package_info(&self) -> &PackageInfo {
        self.package_info()
    }

    fn set_ignore_cursor_events(&self, ignore: bool) -> Result<(), Box<dyn Error>> {
        self.set_ignore_cursor_events(ignore).map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn set_size(&self, size: Size) -> Result<(), Box<dyn Error>> {
        self.set_size(size).map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn set_position(&self, position: Position) -> Result<(), Box<dyn Error>> {
        self.set_position(position).map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn set_focus(&self) -> Result<(), Box<dyn Error>> {
        self.set_focus().map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    fn restore_state(&self, flags: StateFlags) -> Result<(), Box<dyn Error>> {
        WindowExt::restore_state(self, flags).map_err(|err| Box::new(err) as Box<dyn Error>)
    }
}