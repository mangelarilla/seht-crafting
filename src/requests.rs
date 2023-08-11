pub mod gear;
pub mod enchantment;
pub mod consumable;

use std::sync::Arc;
use serenity::model::prelude::*;
use serenity::model::prelude::message_component::MessageComponentInteraction;
use serenity::prelude::*;

async fn ok_response(interaction: &Arc<MessageComponentInteraction>, ctx: &Context) {
    interaction.create_interaction_response(&ctx.http, |response| response
        .kind(InteractionResponseType::DeferredUpdateMessage))
        .await.unwrap();
}

async fn await_component_interaction(msg: Message, ctx: &Context) -> Option<Arc<MessageComponentInteraction>> {
    match msg.await_component_interaction(ctx).timeout(std::time::Duration::from_secs(60 * 3)).await {
        Some(x) => Some(x),
        None => {
            msg.reply(&ctx.http, "Agotado el tiempo para la solicitud, vuelve a empezar").await.unwrap();
            None
        }
    }
}

