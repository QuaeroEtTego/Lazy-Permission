mod state;

use std::{error::Error, sync::Arc};

use futures_util::{future::join_all, StreamExt};
use tracing::{debug, error, info, warn};
use twilight_gateway::{
    error::ReceiveMessageError, stream, stream::ShardEventStream, ConfigBuilder, EventTypeFlags,
    Intents, Shard,
};
use twilight_http::Client as HttpClient;
use twilight_model::{
    application::command::Command,
    gateway::{
        payload::outgoing::update_presence::UpdatePresencePayload,
        presence::{MinimalActivity, Status},
        CloseFrame,
    },
};

use super::config::DiscordConfig;
use super::event;
use super::util::{Shutdown, ShutdownSubscriber};

pub use state::ShardState;

const EVENT_FLAGS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
    EventTypeFlags::INTERACTION_CREATE.bits()
        | EventTypeFlags::READY.bits()
        | EventTypeFlags::RESUMED.bits(),
);

pub struct ShardCluster {
    shards: Vec<Shard>,
    state: ShardState,
}

impl ShardCluster {
    pub async fn new(config: &DiscordConfig) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let token = String::from(config.token());

        let http = Arc::new(HttpClient::new(token.clone()));

        let application_id = http.current_user_application().await?.model().await?.id;
        let current_user_id = http.current_user().await?.model().await?.id;

        let shard_config = ConfigBuilder::new(token, Intents::empty())
            .event_types(EVENT_FLAGS)
            .presence(get_presence())
            .build();

        let shards = stream::create_recommended(&http, shard_config, |_, builder| builder.build())
            .await?
            .collect::<Vec<_>>();

        info!("Cluster with {} shard(s).", shards.len());

        let state = ShardState::new(application_id, current_user_id, http);

        Ok(Self { shards, state })
    }

    pub async fn set_interactions(&self, commands: &[Command]) -> Result<(), twilight_http::Error> {
        self.state
            .http
            .interaction(self.state.application_id)
            .set_global_commands(commands)
            .await?;

        Ok(())
    }

    pub async fn start(
        mut self,
        mut shutdown_subscriber: ShutdownSubscriber,
    ) -> Result<(), ReceiveMessageError> {
        let handler_shutdown = Shutdown::new();

        tokio::select! {
            result = self.handle_events(&handler_shutdown) => result?,
            _ = shutdown_subscriber.wait_shutdown() => {},
        }

        join_all(
            self.shards
                .iter_mut()
                .map(|shard| async move { shard.close(CloseFrame::NORMAL).await }),
        )
            .await;

        handler_shutdown.shutdown().await;

        Ok(())
    }

    async fn handle_events(&mut self, shutdown: &Shutdown) -> Result<(), ReceiveMessageError> {
        let mut stream = ShardEventStream::new(self.shards.iter_mut());

        while let Some((shard, event)) = stream.next().await {
            debug!(
                "Shard {} | Status {:?} | Latency {:?} - {:?}",
                shard.id(),
                shard.status(),
                shard.latency().recent(),
                shard.latency().average()
            );

            let event = match event {
                Ok(event) => event,
                Err(source) => {
                    if source.is_fatal() {
                        error!("Fatal error while receiving event: {}.", source);
                        return Err(source);
                    } else {
                        warn!("Error while receiving event: {}.", source);
                        continue;
                    }
                }
            };

            let state = self.state.clone();

            tokio::spawn(event::handle(
                state,
                event,
                shard.id(),
                shutdown.subscriber(),
            ));
        }

        Ok(())
    }
}

fn get_presence() -> UpdatePresencePayload {
    UpdatePresencePayload::new(
        vec![MinimalActivity {
            kind: Default::default(),
            name: "Lazy with permissions".to_string(),
            url: None,
        }
        .into()],
        false,
        None,
        Status::Online,
    )
    .unwrap()
}
