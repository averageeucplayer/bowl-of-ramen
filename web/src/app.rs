use tauri_sys::event;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::console;
use crate::{api::load, app_context::AppContextProvider, meter::Meter};
use yew_router::prelude::*;
use futures::StreamExt;
use app_core::models::FightUpdate;



#[derive(Clone, Debug, PartialEq)]
struct Settings {
    
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
        Route::Settings => html! { <h1>{ "Settings" }</h1> },
        Route::Logs => html! { <h1>{ "Logs" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
   
    let settings_context = use_state(|| Settings {
        
    });
    let meter_context = use_state(|| Settings {
        
    });


    html! {
        <AppContextProvider>
            <ContextProvider<Settings> context={(*settings_context).clone()}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<Settings>>
        </AppContextProvider>
    }
}
