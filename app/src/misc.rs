use std::error::Error;

use app_core::models::*;
use log::debug;
use serde::Serialize;
use tauri::{AppHandle, Emitter, EventTarget};

pub trait AppEventEmitter : Send + Sync + 'static  {
    fn emit(&self, event: impl AppEvent) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct DefaultEventEmitter {
    app_handle: AppHandle,
    target: EventTarget
}

impl AppEventEmitter for DefaultEventEmitter {
    fn emit(&self, event: impl AppEvent) -> Result<(), Box<dyn Error + Send + Sync>> {

        let json_string = serde_json::to_string_pretty(&event)?;
        debug!("{json_string}");
        self.app_handle.emit_to(self.target.clone(), event.event_name(), event)?;
        Ok(())
    }
}

impl DefaultEventEmitter {
    pub fn new(
        app_handle: AppHandle,
        target: EventTarget) -> Self {
        Self {
            app_handle,
            target
        }
    }
}