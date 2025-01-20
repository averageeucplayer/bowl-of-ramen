mod player_row;
mod player_header;

use log::error;
use log::info;
use player_header::PlayerHeader;
use player_row::PlayerRow;
use tauri_sys::event;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::console;
use yew_router::prelude::*;
use futures::StreamExt;
use app_core::models::*;

struct State {
    pub players: Vec<Player>
}
    // {
    //     use_effect_with(context_dep, move |_| {

    //         spawn_local(async move {
    //             let mut event_stream = event::listen::<FightUpdate>("fight-update").await.unwrap();

    //             while let Some(event) = event_stream.next().await {
    //                 let players = event.payload.players;
    //             }
    //         });
    //         || {}
    //     });
    // }

async fn listen_for_updates(state: yew::UseStateHandle<State>) {
    let mut event_stream = match event::listen::<FightUpdate>("fight-update").await {
        Ok(stream) => stream,
        Err(err) => {

            let error_str = format!("{:?}", err);

            if error_str.contains("Cannot read properties of undefined") {
                error!("not tauri browser");    
            }

            return;
        },
    };

    while let Some(event) = event_stream.next().await {
       
        // info!("{:?}", event);

        let players = event.payload.players;

        state.set(State {
            players
        });
    }
}

#[function_component(Meter)]
pub fn meter() -> Html {
    let state = use_state(|| State {
        players: vec![]
    });

    {
        let state = state.clone();
        use_effect(move || {
            spawn_local(async move {
                listen_for_updates(state).await; 
            });
    
            || {}
        });
    }

    let player_rows: Vec<Html> = state.players.clone().iter().map(|player| {
        html! { <PlayerRow key={player.id} player={player.clone()} /> }
    }).collect(); 

    html! {
        <div>
            <table class="relative w-full table-fixed">
                <PlayerHeader/>
                <tbody>
                    {player_rows}
                </tbody>
            </table>
        </div>
    }
}

// let navigator = use_navigator().unwrap();

// let go_home_button = {
//     let navigator = navigator.clone();
//     let onclick = Callback::from(move |_| navigator.push(&Route::Home));
//     html! {
//         <button {onclick}>{"click to go home"}</button>
//     }
// };