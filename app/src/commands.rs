use commands::load::{load_inner, LoadResult};
use tauri::{command, generate_handler, ipc, AppHandle, Manager};

#[command]
pub fn load(app_handle: AppHandle, locale: String) -> LoadResult {
    let package_info = app_handle.package_info();
    let version = package_info.version.to_string();

    load_inner(version)
}

pub fn generate_handlers() -> Box<dyn Fn(ipc::Invoke) -> bool + Send + Sync> {
    Box::new(generate_handler![load])
}
