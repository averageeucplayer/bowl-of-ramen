mod player_row;
mod player_header;
mod dps_bar;
mod fight_update_example;

use html::Scope;
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

async fn listen_for_updates(link: Scope<Meter>) {
    
    let event_stream_result = fight_update_example::fake_listen().await;
    // let event_stream_result = event::listen::<FightUpdate>("fight-update").await;
    let mut event_stream = match event_stream_result {
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
        link.send_message(MeterMessage::Update(event.payload));
    }
}

pub struct Meter {
    stats: Option<EncounterStats>,
    players: Vec<Player>,
    boss: Option<Boss>
}

pub enum MeterMessage {
    Update(FightUpdate),
    Error
}

impl Component for Meter {
    type Message = MeterMessage;
    type Properties = ();

    fn create(context: &Context<Self>) -> Self {
        let link = context.link().clone();

        spawn_local(async move {
            listen_for_updates(link).await;
        });

        Self {
            stats: None,
            players: vec![],
            boss: None
        }
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) -> bool {      
        match message {
            MeterMessage::Update(payload) => {
                self.players = payload.players;
                self.boss = Some(payload.boss);
                true
            },
            MeterMessage::Error => {
                false
            },
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {
      
        let player_rows: Vec<Html> = self.players.clone().iter().map(|player| {
            html! { <PlayerRow key={player.id} player={player.clone()} /> }
        }).collect();

        if let Some(boss) = &self.boss {
            return  html! {
                <div data-tauri-drag-region="true">
                    <div class="flex">
                        {boss.name.clone()}
                    </div>
                    <img class="absolute w-full brightness-25" src="public/images/bosses/narkiel.png"/>
                    <table class="relative w-full table-fixed">
                        <PlayerHeader/>
                        <tbody>
                            {player_rows}
                        </tbody>
                    </table>
                </div>
            };
        }

        html! {}
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

