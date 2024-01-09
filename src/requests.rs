pub mod gear;
pub mod enchantment;
pub mod consumable;

use serenity::all::{ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseMessage, Message};
use serenity::prelude::*;

async fn ok_response(interaction: &ComponentInteraction, ctx: &Context) {
    interaction.create_response(&ctx.http, CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()))
        .await.unwrap();
}

async fn await_component_interaction(msg: Message, ctx: &Context) -> Option<ComponentInteraction> {
    match msg.await_component_interaction(ctx).timeout(std::time::Duration::from_secs(60 * 3)).await {
        Some(x) => Some(x),
        None => {
            msg.reply(&ctx.http, "Agotado el tiempo para la solicitud, vuelve a empezar").await.unwrap();
            None
        }
    }
}

