use yew::prelude::*;
use app_core::models::*;

#[function_component(PlayerHeader)]
pub fn player_header() -> Html {
    html! {
        <thead>
            <tr>
                <th></th>
                <th class="w-[60px]">{"Dps"}</th>
                <th class="w-[60px]">{"D%"}</th>
                <th class="w-[60px]">{"Crit"}</th>
                <th class="w-[60px]">{"Back"}</th>
            </tr>
        </thead>
    }
}