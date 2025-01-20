use yew::prelude::*;
use app_core::models::*;

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

    html! {
        <tr class="" data-id={id}>
            <td>
                <img class="size-5" src={img_src} alt={class_name}/> 
            </td>
            <td class="truncate">{name}</td>
            <td>{stats.dps.abbreviated.clone()}</td>
        </tr>
    }
}
