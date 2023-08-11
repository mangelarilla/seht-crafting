use serenity::model::prelude::*;
use serenity::model::prelude::component::ActionRowComponent::*;
use serenity::model::prelude::message_component::*;
use serenity::model::prelude::modal::*;
use serenity::prelude::*;
use tracing::info;
use crate::components;

pub async fn enchantment(interaction: MessageComponentInteraction, ctx: &Context) {
    if let Err(why) = interaction
        .create_interaction_response(&ctx.http, |r| r
            .kind(InteractionResponseType::Modal)
            .interaction_response_data(|d| d
                .custom_id("enchantment_modal")
                .title("⚒️ Solicitud de Runas ⚒️")
                .components(|c| c
                    .create_action_row(|row| row
                        .add_input_text(components::enchantments_modal("enchantments"))
                    )
                )
            )
        ).await
    {
        info!("Cannot respond to enchantments request: {}", why)
    }
}

pub async fn enchantment_modal(interaction: ModalSubmitInteraction, ctx: &Context) {
    if let Err(why) = interaction.create_interaction_response(&ctx.http, |response| response
        .kind(InteractionResponseType::DeferredUpdateMessage)
    ).await {
        info!("Cannot respond to enchantments modal: {}", why)
    } else {
        if let InputText(input) = interaction
            .data.components.get(0).unwrap()
            .components.get(0).unwrap()
        {
            interaction.channel_id.send_message(&ctx.http, |m|
                m.content(format!("__**‼️Peticion de Encantamientos para {}‼️**__ \n {}", Mention::User(interaction.user.id), &input.value))
            ).await.unwrap();
        }
    }
}