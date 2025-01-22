use yew::prelude::*;
use app_core::models::*;

use crate::meter::dps_bar::DpsBar;

#[derive(Properties, Debug, PartialEq)]
pub struct PlayerRowProps {
    pub player: Player,
}

#[function_component(PlayerRow)]
pub fn player_row(props: &PlayerRowProps) -> Html {
    let id = props.player.id.to_string();
    let name = &props.player.name;
    let class_id = &props.player.class_id;
    let class_name = props.player.class_name.to_string();
    let stats = &props.player.stats;
    let img_src = format!("public/images/classes/{}.png", class_id);
    let dps_value = format!("{:.1}", stats.dps.value);
    let crit_rate = format!("{:.2}", stats.crit_rate * 100.0);
    let damage_percentage = format!("{:.2}", stats.damage_percentage * 100.0);

    html! {
        <>
        <DpsBar percentage={stats.damage_percentage * 100.0}/>
        <tr class="" data-id={id}>
            <td>
                <img class="size-5 inline-block mr-2" src={img_src} alt={class_name}/>
                <span>{name}</span>
            </td>
            <td class="px-1 text-center">{dps_value}<span class="text-xs text-gray-300">{stats.dps.unit}</span></td>
            <td class="text-center">{damage_percentage}<span class="text-xs text-gray-300">{"%"}</span></td>
            <td class="text-center">{crit_rate}<span class="text-xs text-gray-300">{"%"}</span></td>
        </tr>
        </>
    }
}
