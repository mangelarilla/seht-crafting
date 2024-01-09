use regex::Regex;
use serenity::all::{ComponentInteraction, Context, CreateActionRow, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateModal, Mention, ModalInteraction};
use serenity::all::ActionRowComponent::InputText;
use tracing::info;
use crate::components;

pub async fn enchantment(interaction: ComponentInteraction, ctx: &Context) {
    let response = CreateInteractionResponse::Modal(
        CreateModal::new("enchantment_modal", "⚒️ Solicitud de Runas ⚒️")
            .components(vec![CreateActionRow::InputText(components::enchantments_modal("enchantments"))])
    );

    if let Err(why) = interaction.create_response(&ctx.http, response).await {
        info!("Cannot respond to enchantments request: {}", why)
    }
}

pub async fn enchantment_modal(interaction: ModalInteraction, ctx: &Context) {
    if let Err(why) = interaction
        .create_response(&ctx.http, CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new())).await {
        info!("Cannot respond to enchantments modal: {}", why)
    } else {
        if let InputText(input) = interaction
            .data.components.get(0).unwrap()
            .components.get(0).unwrap()
        {
            let re = Regex::new(r"<@&\d+>").unwrap();
            let msg = &interaction.message.unwrap();
            let role = re.captures(&msg.content).unwrap()
                .get(0).unwrap().as_str();
            interaction.channel_id.send_message(&ctx.http, CreateMessage::new()
                .content(format!("{}\n\n__**‼️Peticion de Encantamientos para {}‼️**__ \n {}", role, Mention::User(interaction.user.id), input.value.clone().unwrap()))
            ).await.unwrap();
        }
    }
}