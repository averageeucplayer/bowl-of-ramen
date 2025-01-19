use std::error::Error;

use serde::Serialize;
use tauri::{AppHandle, Emitter, EventTarget};

use crate::fight_simulator::{Boss, Player};

#[derive(Clone, Serialize)]
pub enum AppEvent<'a> {
    FightUpdate {
        players: &'a [Player],
        boss: &'a Boss<'a>
    }
}

impl<'a> AppEvent<'a> {
    pub fn event_name(&self) -> &'static str {
        match &self {
            AppEvent::FightUpdate { .. } => "fight-update",
        }
    }
}

pub trait AppEventEmitter : Send + Sync + 'static  {
    fn emit(&self, event: AppEvent) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct DefaultEventEmitter {
    app_handle: AppHandle,
    target: EventTarget
}

impl AppEventEmitter for DefaultEventEmitter {
    fn emit(&self, event: AppEvent) -> Result<(), Box<dyn Error + Send + Sync>> {

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