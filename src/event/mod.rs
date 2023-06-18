use tracing::{debug, info};
use twilight_model::gateway::{event::Event, payload::incoming::Ready, ShardId};

use super::bot::BotState;
use super::interaction;
use super::util::{Itoa, ShutdownSubscriber};

pub async fn handle(
    shard_id: ShardId,
    event: Event,
    state: BotState,
    mut _shutdown_subscriber: ShutdownSubscriber,
) {
    match event {
        Event::InteractionCreate(e) => interaction::handle((*e).0, state).await,
        Event::Ready(e) => ready(shard_id, *e),
        Event::Resumed => resumed(shard_id),
        _ => {}
    }
}

fn ready(shard_id: ShardId, ready: Ready) {
    info!(
        "Shard {} ready with {} guild(s)",
        shard_id.itoa(),
        ready.guilds.len()
    );
}

fn resumed(shard_id: ShardId) {
    debug!("Shard {} resumed", shard_id.itoa());
}
