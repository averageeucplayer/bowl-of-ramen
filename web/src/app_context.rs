use log::info;
use yew::{platform::spawn_local, prelude::*};

use crate::api::load;


#[derive(Clone, Debug, PartialEq)]
struct AppState {
    is_loading: bool,
    version: String,
}

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(AppContextProvider)]
pub fn provider(props: &Props) -> Html {
    let context = use_state(|| AppState {
        is_loading: true,
        version: "".to_owned(),
    });

    {
        let context = context.clone();
        use_effect(move || {

            spawn_local(async move {
                if !context.is_loading {
                    return;
                }
              
                info!("Loading...");
                let load_result = load().await.unwrap();
                let app_state = AppState {
                    is_loading: false,
                    version: load_result.version
                };
                context.set(app_state);
            });
    
            || {}
        });
    }

    html! {
        <ContextProvider<AppState> context={(*context).clone()}>
            {props.children.clone()}
        </ContextProvider<AppState>>
    }
}