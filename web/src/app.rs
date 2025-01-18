use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api::load;

#[derive(Clone, Debug, PartialEq)]
struct AppState {
    version: String,
}

#[derive(Clone, Debug, PartialEq)]
struct Settings {
    
}


#[function_component(App)]
pub fn app() -> Html {
    let app_context = use_state(|| AppState {
        version: "".to_owned(),
    });
    let settings_context = use_state(|| Settings {
        
    });

    {
        let context = app_context.clone();
        use_effect(|| {

            spawn_local(async move {
                let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
                let load_result = load().await.unwrap();
                let app_state = AppState {
                    version: load_result.version
                };
                context.set(app_state);
            });
    
            || {}
        });
    }

    html! {
        <ContextProvider<AppState> context={(*app_context).clone()}>
            <ContextProvider<Settings> context={(*settings_context).clone()}>
                <main class="container">
                
                </main>
            </ContextProvider<Settings>>
        </ContextProvider<AppState>>
    }
}
