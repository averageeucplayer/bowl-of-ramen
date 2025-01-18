// #![allow(unused_imports)]
use crate::abstractions::*;
use raw_window_handle::HasWindowHandle;
use std::{sync::{Arc, Mutex}, error::Error, path::Path};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use tauri::{EventId, Event, PackageInfo, Position, Size, WebviewWindow, Window};

#[cfg(test)]
use mockall::{
    mock,
    predicate::{self, *},
};


#[cfg(test)]
mock! {
    pub AppWindow {}
    impl AppWindow for AppWindow {
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
}

#[cfg(test)]
mock! {
    pub AppHandle {}
    impl AppHandle for AppHandle {
        fn get_window(&self, str: &str) -> Option<Box<dyn AppWindow>>;
        fn save_window_state(&self, flags: StateFlags) -> Result<(), Box<dyn Error>>;
        fn exit(&self);
    }
}