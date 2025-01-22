use std::{error::Error, sync::Arc, time::Duration};

use log::debug;
use tokio::time::sleep;

use crate::{fight_simulator::{EstherTemplate, FightSimulator}, misc::AppEventEmitter};

pub async fn run_background_work<E: AppEventEmitter>(event_emitter: Arc<E>) -> Result<(), Box<dyn Error + Send + Sync>> {
    
    debug!("run_background_work");
    // let event_emitter = event_emitter.as_ref();

    let mut fight_simulator = FightSimulator::new();
    let min_dmg = 100_000_000;
    let max_dmg = 250_000_000;
    fight_simulator.create_8_players(min_dmg, max_dmg)?;
    fight_simulator.set_random_player_dead();
    fight_simulator.create_boss(485000, "Red Doom Narkiel", 100_000_000_000, 180);
    fight_simulator.configure_esther(EstherTemplate {
        name: "Azena".into(),
        icon: "azena.png".into(),
        min_dmg: 2_000_000_000,
        max_dmg: 4_000_000_000
    });
    fight_simulator.configure_esther(EstherTemplate {
        name: "Avele".into(),
        icon: "avele.png".into(),
        min_dmg: 1_000_000_000,
        max_dmg: 2_000_000_000
    });
    fight_simulator.configure_esther(EstherTemplate {
        name: "Thar".into(),
        icon: "thar.png".into(),
        min_dmg: 500_000_000,
        max_dmg: 1_000_000_000
    });

    let interval = Duration::from_millis(500);

    for _ in 1..=10 {

        if fight_simulator.has_ended() {
            break;
        }

        fight_simulator.update_time();
        fight_simulator.update_esther_gauge();

        fight_simulator.update_duration();
        fight_simulator.perform_attacks_and_update_stats()?;

        fight_simulator.try_use_esther();

        let app_event = fight_simulator.to_fight_update_event();
        event_emitter.emit(app_event)?;

        sleep(interval).await;
    }

    Ok(())
}