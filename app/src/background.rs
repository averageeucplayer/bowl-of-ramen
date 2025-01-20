use std::{error::Error, sync::Arc, time::Duration};

use log::debug;
use tokio::time::sleep;

use crate::{fight_simulator::FightSimulator, misc::AppEventEmitter};


pub async fn run_background_work<E: AppEventEmitter>(event_emitter: Arc<E>) -> Result<(), Box<dyn Error + Send + Sync>> {
    
    debug!("run_background_work");
    // let event_emitter = event_emitter.as_ref();

    let mut fight_simulator = FightSimulator::new();
    let min_dmg = 500_000_000;
    let max_dmg = 1_100_000_000;
    fight_simulator.create_8_players(min_dmg, max_dmg);
    fight_simulator.create_boss("Test Boss", 100_000_000_000, 300);
    let one_second = Duration::from_millis(500);

    loop {
        if fight_simulator.has_ended() {
            break;
        }

        fight_simulator.perform_attack_and_update_stats();

        let app_event = fight_simulator.to_fight_update_event();
        event_emitter.emit(app_event)?;

        sleep(one_second).await;
    }

    Ok(())
}