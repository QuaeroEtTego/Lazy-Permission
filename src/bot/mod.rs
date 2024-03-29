use std::error::Error;

use futures_util::StreamExt;
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
        presence::{Activity, MinimalActivity, Status},
    },
};

use super::config::DiscordConfig;
use super::event;
use super::util::{Itoa, Shutdown, ShutdownSubscriber};

pub use self::state::BotState;

mod state;

pub struct Bot {
    shards: Vec<Shard>,
    state: BotState,
}

impl Bot {
    pub async fn new(config: &DiscordConfig) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let token = String::from(config.token());

        let http = HttpClient::new(token.clone());

        let application_id = http.current_user_application().await?.model().await?.id;
        let current_user_id = http.current_user().await?.model().await?.id;

        let event_flags =
            EventTypeFlags::INTERACTION_CREATE | EventTypeFlags::READY | EventTypeFlags::RESUMED;

        let shard_config = ConfigBuilder::new(token, Intents::empty())
            .event_types(event_flags)
            .presence(presence())
            .build();

        let shards = stream::create_recommended(&http, shard_config, |_, builder| builder.build())
            .await?
            .collect::<Vec<_>>();

        info!("Bot with {} shard(s)", shards.len());

        let state = BotState::new(application_id, current_user_id, http);

        Ok(Self { shards, state })
    }

    pub async fn set_commands(&self, commands: &[Command]) -> Result<(), twilight_http::Error> {
        self.state
            .interaction()
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

        handler_shutdown.shutdown().await;

        Ok(())
    }

    async fn handle_events(&mut self, shutdown: &Shutdown) -> Result<(), ReceiveMessageError> {
        let mut stream = ShardEventStream::new(self.shards.iter_mut());

        while let Some((shard, event)) = stream.next().await {
            debug!(
                "Shard {} | Status {:?} | Latency {:?} - {:?}",
                shard.id().itoa(),
                shard.status(),
                shard.latency().recent(),
                shard.latency().average()
            );

            let event = match event {
                Ok(event) => event,
                Err(source) => {
                    if source.is_fatal() {
                        error!("Fatal error while receiving event: {:?}", source);
                        return Err(source);
                    } else {
                        warn!("Error while receiving event: {:?}", source);
                        continue;
                    }
                }
            };

            let state = self.state.clone();

            tokio::spawn(event::handle(
                shard.id(),
                event,
                state,
                shutdown.subscriber(),
            ));
        }

        Ok(())
    }
}

fn presence() -> UpdatePresencePayload {
    let minimal_activity = MinimalActivity {
        kind: Default::default(),
        name: String::from("Lazy with permissions"),
        url: None,
    };

    UpdatePresencePayload {
        activities: vec![Activity::from(minimal_activity)],
        afk: false,
        since: None,
        status: Status::Online,
    }
}
