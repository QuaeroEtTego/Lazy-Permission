use std::error::Error;

use tracing::{error, warn};
use twilight_model::{
    application::interaction::{Interaction, InteractionData, InteractionType},
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{embed::EmbedBuilder, InteractionResponseDataBuilder};

use super::bot::BotState;
use super::util::Color;

type InteractionResult = Result<InteractionResponse, Box<dyn Error + Send + Sync>>;

pub async fn handle(interaction: Interaction, state: BotState) {
    let interaction_id = interaction.id;
    let interaction_token = interaction.token.clone();

    let result = match &interaction.kind {
        InteractionType::ApplicationCommand => handle_command(interaction, &state),
        other => {
            warn!("Unexpected {} interaction", other.kind());
            return;
        }
    }
    .await;

    let response = result.unwrap_or_else(|error| {
        error!("Failed to execute a command {:?}", error);

        let embed = EmbedBuilder::new()
            .title("Internal Error")
            .color(Color::Red.into())
            .build();

        InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .embeds([embed])
                    .flags(MessageFlags::EPHEMERAL)
                    .build(),
            ),
        }
    });

    if let Err(error) = state
        .http
        .interaction(state.application_id)
        .create_response(interaction_id, &interaction_token, &response)
        .await
    {
        error!("Failed to create an interaction response {:?}", error);
    }
}

async fn handle_command(interaction: Interaction, _state: &BotState) -> InteractionResult {
    let _name = match &interaction.data {
        Some(InteractionData::ApplicationCommand(data)) => &*data.name,
        _ => {
            return Ok(InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .content("Interaction data not found")
                        .flags(MessageFlags::EPHEMERAL)
                        .build(),
                ),
            })
        }
    };

    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(
            InteractionResponseDataBuilder::new()
                .content("Command handler")
                .build(),
        ),
    })
}
