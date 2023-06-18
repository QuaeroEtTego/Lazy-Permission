use twilight_model::{
    channel::message::MessageFlags,
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::{embed::EmbedBuilder, InteractionResponseDataBuilder};

use super::Color;

pub fn internal_error() -> InteractionResponse {
    let embed = EmbedBuilder::new()
        .title("Internal Error")
        .color(Color::Red as u32)
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
}

pub fn interaction_data_not_found() -> InteractionResponse {
    let embed = EmbedBuilder::new()
        .title("Interaction data not found")
        .color(Color::Red as u32)
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
}
