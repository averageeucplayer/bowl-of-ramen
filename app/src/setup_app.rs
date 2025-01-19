use std::{error::Error, sync::Arc};
use tauri::{App, EventTarget, Manager};
use tokio::task;

use crate::{background::run_background_work, misc::{AppEventEmitter, DefaultEventEmitter}, system_tray::setup_system_tray};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {

    let app_handle = app.app_handle();
    setup_system_tray(app)?;

    let event_emitter = DefaultEventEmitter::new(
        app_handle.clone(),
        EventTarget::WebviewWindow { label: "main".into() });
    let shared_event_emitter: Arc<dyn AppEventEmitter> = Arc::new(event_emitter);

    task::spawn_blocking(move || {
        let event_emitter = shared_event_emitter.clone();
        run_background_work(event_emitter)
    });

    Ok(())
}