use tauri_sys::event;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::console;
use crate::{api::load, app_context::AppContextProvider, meter::Meter, settings::Settings, logs::Logs};
use yew_router::prelude::*;
use futures::StreamExt;

#[derive(Clone, Debug, PartialEq)]
struct GeneralSettings {
    
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Meter,
    #[at("/settings")]
    Settings,
    #[at("/logs")]
    Logs,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Meter => html! { <Meter></Meter> },
        Route::Settings => html! { <Settings></Settings> },
        Route::Logs => html! { <Logs></Logs> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
   
    let settings_context = use_state(|| GeneralSettings {
        
    });

    html! {
        <AppContextProvider>
            <ContextProvider<GeneralSettings> context={(*settings_context).clone()}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<GeneralSettings>>
        </AppContextProvider>
    }
}
