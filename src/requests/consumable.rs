use regex::Regex;
use serenity::model::prelude::*;
use serenity::model::prelude::component::ActionRowComponent::*;
use serenity::model::prelude::message_component::*;
use serenity::model::prelude::modal::*;
use serenity::prelude::*;
use tracing::info;
use crate::components;

pub async fn consumables(interaction: MessageComponentInteraction, ctx: &Context) {
    if let Err(why) = interaction
        .create_interaction_response(&ctx.http, |r| r
            .kind(InteractionResponseType::Modal)
            .interaction_response_data(|d| d
                .custom_id("consumables_modal")
                .title("⚒️ Solicitud de Consumibles ⚒️")
                .components(|c| c
                    .create_action_row(|row| row
                        .add_input_text(components::consumables_modal("consumables"))
                    )
                )
            )
        ).await
    {
        info!("Cannot respond to consumables request: {}", why)
    }
}



pub async fn consumables_modal(interaction: ModalSubmitInteraction, ctx: &Context) {
    if let Err(why) = interaction.create_interaction_response(&ctx.http, |response| response
        .kind(InteractionResponseType::DeferredUpdateMessage)
    ).await {
        info!("Cannot respond to consumables modal: {}", why)
    } else {
        if let InputText(input) = interaction
            .data.components.get(0).unwrap()
            .components.get(0).unwrap()
        {
            let re = Regex::new(r"<@&\d+>").unwrap();
            let msg = &interaction.message.unwrap();
            let role = re.captures(&msg.content).unwrap()
                .get(0).unwrap().as_str();
            interaction.channel_id.send_message(&ctx.http, |m|
                m.content(format!("{}\n\n__**‼️Peticion de Consumibles para {}‼️**__ \n {}", role, Mention::User(interaction.user.id), &input.value))
            ).await.unwrap();
        }
    }
}