use tracing::info;

use twilight_model::gateway::{event::Event, payload::incoming::Ready, ShardId};

use super::cluster::ShardState;
use super::util::ShutdownSubscriber;

pub async fn handle(
    _state: ShardState,
    event: Event,
    shard_id: ShardId,
    mut _shutdown_subscriber: ShutdownSubscriber,
) {
    match event {
        Event::InteractionCreate(_) => {}
        Event::Ready(e) => ready(shard_id, e),
        Event::Resumed => resumed(shard_id),
        _ => {}
    }
}

async fn interaction_create() {
    todo!()
}

fn ready(shard_id: ShardId, ready: Box<Ready>) {
    info!(
        "Shard {} ready with {} guild!",
        shard_id,
        ready.guilds.len()
    );
}

fn resumed(shard_id: ShardId) {
    info!("Shard {} resumed!", shard_id);
}
