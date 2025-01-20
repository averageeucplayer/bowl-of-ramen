use std::{error::Error, sync::Arc};
use log::{error, warn};
use tauri::{App, EventTarget, Manager};
use tokio::task;

use crate::{background::run_background_work, misc::DefaultEventEmitter, system_tray::setup_system_tray};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn Error>> {

    #[cfg(debug_assertions)]
    {
        let window = app.get_webview_window("main").unwrap();
        window.open_devtools();
    }

    let app_handle = app.app_handle();
    setup_system_tray(app)?;

    let event_emitter = DefaultEventEmitter::new(
        app_handle.clone(),
        EventTarget::WebviewWindow { label: "main".into() });
    let shared_event_emitter: Arc<DefaultEventEmitter> = Arc::new(event_emitter);

    tokio::spawn(async move {
        let event_emitter = shared_event_emitter.clone();
       
        let result = run_background_work(event_emitter).await;

        match result {
            Ok(_) => {
                warn!("early finish?")
            },
            Err(err) => {
                error!("{:?}", err);
            },
        }
    });

    Ok(())
}