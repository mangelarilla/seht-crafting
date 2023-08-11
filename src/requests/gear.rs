use serenity::model::prelude::*;
use serenity::model::prelude::component::ActionRowComponent::*;
use serenity::model::prelude::component::*;
use serenity::model::prelude::message_component::*;
use serenity::model::prelude::modal::*;
use serenity::prelude::*;
use tracing::info;
use crate::components;
use crate::components::SetPiece;
use crate::requests::{await_component_interaction, ok_response};

pub async fn gear(interaction: MessageComponentInteraction, ctx: &Context) {
    if let Err(why) = interaction
        .create_interaction_response(&ctx.http, |r| r
            .kind(InteractionResponseType::Modal)
            .interaction_response_data(|d| d
                .custom_id("gear_set_modal")
                .title("⚒️ Solicitud de Equipamiento ⚒️")
                .components(|c| c
                    .create_action_row(|row| row
                        .add_input_text(components::gear_set_modal("gear_set"))
                    )
                )
            )
        ).await
    {
        info!("Cannot respond to gear request: {}", why)
    }
}

pub async fn gear_modal(interaction: ModalSubmitInteraction, ctx: &Context) {
    if let Err(why) = interaction.create_interaction_response(&ctx.http, |response| response
        .kind(InteractionResponseType::DeferredUpdateMessage)
    ).await {
        info!("Cannot respond to gear set modal: {}", why)
    } else {
        if let InputText(input) = interaction
            .data.components.get(0).unwrap()
            .components.get(0).unwrap()
        {
            let parts = select_gear_set_options(&input.value, &interaction.user, ctx).await;
            interaction.channel_id.send_message(&ctx.http, |m| {
                m.content(format!("__**‼️Peticion de Equipamiento para {}‼️**__", Mention::User(interaction.user.id)));
                m.set_embeds(parts.into_iter()
                    .map(|p| components::gear_set_piece_embed(&input.value, &p))
                    .collect());
                m
            }).await.unwrap();
        }
    }
}

async fn select_gear_set_options(set: &str, user: &User, ctx: &Context) -> Vec<SetPiece> {
    info!("Gear set: {:#?}", set);
    let selected_parts = select_parts(set, user, ctx).await;

    let mut set_parts: Vec<SetPiece> = Vec::new();
    for part in parse_one_handed(selected_parts, user, ctx).await
    {
        let selected_trait = select_trait(&part, user, ctx).await;
        let quality = select_quality(&part, user, ctx).await;

        if components::is_armor(&part) {
            let weight = select_weight(&part, user, ctx).await;

            set_parts.push(SetPiece {
                part: part.to_string(),
                part_trait: selected_trait,
                weight: Some(weight),
                quality
            });
        } else {
            set_parts.push(SetPiece {
                part: part.to_string(),
                part_trait: selected_trait,
                weight: None,
                quality
            });
        }

        user.dm(&ctx.http, |msg| msg
            .set_embed(components::gear_piece_embed(&set_parts.last().unwrap()))
        ).await.unwrap();
    }

    set_parts
}

async fn parse_one_handed(parts: Vec<String>, user: &User, ctx: &Context) -> Vec<String> {
    let mut all_parts: Vec<String> = Vec::new();
    for part in parts {
        if components::is_one_handed(&part) {
            let dual = user.dm(&ctx.http, |msg| msg
                .content(format!("Has pedido una __**{}**__ de una mano. Quieres otra para ir con armas duales?", part))
                .components(|c| c.create_action_row(|row| row
                    .create_button(|b| b.custom_id("dual_yes").emoji(ReactionType::Unicode("👍".to_string())).label("Si").style(ButtonStyle::Success))
                    .create_button(|b| b.custom_id("dual_no").emoji(ReactionType::Unicode("👎".to_string())).label("No").style(ButtonStyle::Danger))
                ))
            ).await.unwrap();
            let dual_interaction = await_component_interaction(dual, ctx).await.unwrap();
            if dual_interaction.data.custom_id == "dual_yes" {
                all_parts.push(format!("{} (1)", part));
                all_parts.push(format!("{} (2)", part));
            } else {
                all_parts.push(part.to_string());
            }
            dual_interaction.create_interaction_response(&ctx.http, |response| response
                .kind(InteractionResponseType::DeferredUpdateMessage))
                .await.unwrap();
        } else {
            all_parts.push(part.to_string());
        }
    }

    all_parts
}

async fn select_parts(set: &str, user: &User, ctx: &Context) -> Vec<String> {
    let msg = user.dm(&ctx.http, |msg| msg
        .set_embed(components::gear_set_embed(set))
        .components(|c| c
            .create_action_row(|row| row
                .add_select_menu(components::gear_set_parts("gear_set_parts"))
            )
        )
    ).await.unwrap();

    let interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&interaction, ctx).await;

    interaction.data.values
        .iter()
        .map(|f| f.to_string())
        .collect()
}

async fn select_trait(part: &str, user: &User, ctx: &Context) -> String {
    let msg = user.dm(&ctx.http, |msg| msg
        .content(format!("Selecciona **rasgo** para __**{}**__", part))
        .components(|c| c
            .create_action_row(|row| row
                .add_select_menu(components::traits::get_trait(part, "trait"))
            )
        )
    ).await.unwrap();

    let trait_interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&trait_interaction, ctx).await;

    trait_interaction.data.values.get(0).unwrap().to_string()
}

async fn select_quality(part: &str, user: &User, ctx: &Context) -> String {
    let msg = user.dm(&ctx.http, |msg| msg
        .content(format!("Selecciona **calidad** para __**{}**__", part))
        .components(|c| {
            c.create_action_row(|row| row
                .add_select_menu(components::gear_quality("quality"))
            )
        })
    ).await.unwrap();

    let quality_interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&quality_interaction, ctx).await;

    quality_interaction.data.values.get(0).unwrap().to_string()
}

async fn select_weight(part: &str, user: &User, ctx: &Context) -> String {
    let msg = user.dm(&ctx.http, |msg| msg
        .content(format!("Selecciona **peso** de la pieza: __**{}**__", part))
        .components(|c| {
            c.create_action_row(|row| row
                .add_select_menu(components::armor_weight("weight"))
            )
        })
    ).await.unwrap();

    let weight_interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&weight_interaction, ctx).await;

    weight_interaction.data.values.get(0).unwrap().to_string()
}