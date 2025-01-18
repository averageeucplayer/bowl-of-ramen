use tauri::{Manager, Runtime};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use super::AppWindow;
use std::{error::Error, sync::{Arc, Mutex}};

pub trait AppHandle: Send + Sync + 'static {
    fn get_window(&self, str: &str) -> Option<Box<dyn AppWindow>>;
    fn save_window_state(&self, flags: StateFlags) -> Result<(), Box<dyn Error>>;
    fn exit(&self);
}

impl<R: Runtime> AppHandle for tauri::AppHandle<R> {

    fn get_window(&self, name: &str) -> Option<Box<dyn AppWindow>> {
        self.get_webview_window(name)
            .map(|window| Box::new(window) as Box<dyn AppWindow>)
    }

    fn save_window_state(&self, flags: StateFlags) -> Result<(), Box<dyn Error>> {
        AppHandleExt::save_window_state(self.app_handle(), flags)
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }
    
    fn exit(&self) {
        self.exit(0)
    }
}
