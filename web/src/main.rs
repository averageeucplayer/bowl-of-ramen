mod app;
mod api;
mod settings;
mod meter;
mod logs;
mod app_context;
use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
