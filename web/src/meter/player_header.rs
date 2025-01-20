use yew::prelude::*;
use app_core::models::*;

#[function_component(PlayerHeader)]
pub fn player_header() -> Html {
    html! {
        <thead>
            <tr>
                <th>{"Dps"}</th>
            </tr>
            <tr>
                <th>{"Crit"}</th>
            </tr>
        </thead>
    }
}