use std::error::Error;

use tracing::{error, warn};
use twilight_model::{
    application::interaction::{Interaction, InteractionData, InteractionType},
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use super::bot::BotState;
use super::util::Color;

mod embed;

type InteractionResult = Result<InteractionResponse, Box<dyn Error + Send + Sync>>;

pub async fn handle(interaction: Interaction, state: BotState) {
    let interaction_id = interaction.id;
    let interaction_token = &*interaction.token.clone();

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
        embed::error::internal_error()
    });

    if let Err(error) = state
        .interaction()
        .create_response(interaction_id, interaction_token, &response)
        .await
    {
        error!("Failed to create an interaction response {:?}", error);
    }
}

async fn handle_command(interaction: Interaction, _state: &BotState) -> InteractionResult {
    let _name = match &interaction.data {
        Some(InteractionData::ApplicationCommand(data)) => &*data.name,
        _ => {
            warn!(
                "Interaction application command data not found {:?}",
                interaction
            );
            return Ok(embed::error::interaction_data_not_found());
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
