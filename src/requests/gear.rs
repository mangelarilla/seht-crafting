use serenity::model::prelude::*;
use serenity::model::prelude::component::ActionRowComponent::*;
use serenity::model::prelude::message_component::*;
use serenity::model::prelude::modal::*;
use serenity::prelude::*;
use tracing::info;
use crate::components;
use crate::components::SetPiece;
use crate::entities::armour::ArmourParts;
use crate::entities::Gear;
use crate::entities::jewelry::Jewelries;
use crate::entities::weapon::WeaponKind;
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
                m.content(format!("__**‼️Peticion de Equipamiento para {}‼️**__\n\n",
                                  Mention::User(interaction.user.id)));
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
    let mut all_armour_trait: Option<String> = None;
    let mut all_jewelry_trait: Option<String> = None;
    let mut all_weapons_trait: Option<String> = None;
    let mut all_armour_enchantment: Option<String> = None;
    let mut all_jewelry_enchantment: Option<String> = None;
    let mut all_weapons_enchantment: Option<String> = None;
    let mut all_armour_quality: Option<String> = None;
    let mut all_jewelry_quality: Option<String> = None;
    let mut all_weapons_quality: Option<String> = None;
    let mut all_armor_weight: Option<String> = None;
    let mut asked_all_armour = false;
    let mut asked_all_jewelry = false;
    let mut asked_all_weapons = false;
    for part in process_duplicates(selected_parts, user, ctx).await
    {
        let selected_trait = match part {
            Gear::Weapon(_) if all_weapons_trait.is_some() => all_weapons_trait.clone().unwrap(),
            Gear::Armour(_) if all_armour_trait.is_some() => all_armour_trait.clone().unwrap(),
            Gear::Jewelry(_) if all_jewelry_trait.is_some() => all_jewelry_trait.clone().unwrap(),
            _ => select_trait(&part, user, ctx).await,
        };
        let selected_enchantment = match part {
            Gear::Weapon(_) if all_weapons_enchantment.is_some() => all_weapons_enchantment.clone().unwrap(),
            Gear::Armour(_) if all_armour_enchantment.is_some() => all_armour_enchantment.clone().unwrap(),
            Gear::Jewelry(_) if all_jewelry_enchantment.is_some() => all_jewelry_enchantment.clone().unwrap(),
            _ => select_enchantment(&part, user, ctx).await
        };
        let quality = match part {
            Gear::Weapon(_) if all_weapons_quality.is_some() => all_weapons_quality.clone().unwrap(),
            Gear::Armour(_) if all_armour_quality.is_some() => all_armour_quality.clone().unwrap(),
            Gear::Jewelry(_) if all_jewelry_quality.is_some() => all_jewelry_quality.clone().unwrap(),
            _ => select_quality(&part, user, ctx).await
        };

        if let Gear::Armour(armour_part) = &part {
            let weight = if all_armor_weight.is_some() {
                all_armor_weight.clone().unwrap()
            } else {
                select_weight(&armour_part, user, ctx).await
            };

            set_parts.push(SetPiece {
                part: armour_part.to_string(),
                part_trait: selected_trait.to_string(),
                weight: Some(weight),
                enchantment: selected_enchantment.to_string(),
                quality: quality.to_string()
            });
        } else {
            set_parts.push(SetPiece {
                part: part.to_string(),
                part_trait: selected_trait.to_string(),
                weight: None,
                enchantment: selected_enchantment.to_string(),
                quality: quality.to_string()
            });
        }

        let set_piece = set_parts.last().unwrap();
        user.dm(&ctx.http, |msg| msg
            .set_embed(components::gear_piece_embed(set_piece))
        ).await.unwrap();


        match part {
            Gear::Weapon(_) => {
                if !asked_all_weapons {
                    all_weapons_trait = if confirm_trait_for_all(&part, &selected_trait, user, ctx).await {Some(selected_trait)} else { None };
                    all_weapons_enchantment = if confirm_enchantment_for_all(&part, &selected_enchantment, user, ctx).await {Some(selected_enchantment)} else { None };
                    all_weapons_quality = if confirm_quality_for_all(&part, &quality, user, ctx).await {Some(quality)} else { None };
                    asked_all_weapons = true;
                }
            }
            Gear::Armour(_) => {
                if !asked_all_armour {
                    let weight = set_piece.weight.clone().unwrap();
                    all_armour_trait = if confirm_trait_for_all(&part, &selected_trait, user, ctx).await {Some(selected_trait)} else { None };
                    all_armor_weight = if confirm_weight_for_all(&weight, user, ctx).await {Some(weight)} else { None };
                    all_armour_enchantment = if confirm_enchantment_for_all(&part, &selected_enchantment, user, ctx).await {Some(selected_enchantment)} else { None };
                    all_armour_quality = if confirm_quality_for_all(&part, &quality, user, ctx).await {Some(quality)} else { None };
                    asked_all_armour = true;
                }
            }
            Gear::Jewelry(_) => {
                if !asked_all_jewelry {
                    all_jewelry_trait = if confirm_trait_for_all(&part, &selected_trait, user, ctx).await {Some(selected_trait)} else { None };
                    all_jewelry_enchantment = if confirm_enchantment_for_all(&part, &selected_enchantment, user, ctx).await {Some(selected_enchantment)} else { None };
                    all_jewelry_quality = if confirm_quality_for_all(&part, &quality, user, ctx).await {Some(quality)} else { None };
                    asked_all_jewelry = true;
                }
            }
        }

    }

    set_parts
}

async fn process_duplicates(parts: Vec<Gear>, user: &User, ctx: &Context) -> Vec<Gear> {
    let mut all_parts: Vec<Gear> = parts.clone();

    if parts.contains(&Gear::Jewelry(Jewelries::Ring)) {
        all_parts.push(Gear::Jewelry(Jewelries::Ring));
    }

    for weapon in parts.into_iter()
        .filter_map(|g| if let Gear::Weapon(w) = g {Some(w)} else { None })
        .filter_map(|w| if let WeaponKind::OneHanded(w) = w {Some(w)} else {None})
    {
        let dual = user.dm(&ctx.http, |msg| msg
            .content(format!("Has pedido una __**{}**__ de una mano. Quieres otra para ir con armas duales?", weapon))
            .components(|c| c.add_action_row(components::confirmation_row("dual_yes")))
        ).await.unwrap();

        let dual_interaction = await_component_interaction(dual, ctx).await.unwrap();
        if dual_interaction.data.custom_id == "dual_yes" {
            all_parts.push(Gear::Weapon(WeaponKind::OneHanded(weapon.clone())));
        }
        ok_response(&dual_interaction, ctx).await;
    }

    all_parts.sort();
    all_parts
}

async fn select_parts(set: &str, user: &User, ctx: &Context) -> Vec<Gear> {
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
        .filter_map(|f| Gear::try_from(f.to_string()).ok())
        .collect()
}

async fn select_enchantment(part: &Gear, user: &User, ctx: &Context) -> String {
    let msg = user.dm(&ctx.http, |msg| msg
        .content(format!("Selecciona **encantamiento** para __**{}**__", part))
        .components(|c| c
            .create_action_row(|row| row
                .add_select_menu(components::enchantments::get_enchantments(part, "enchantment"))
            )
        )
    ).await.unwrap();

    let enchantment_interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&enchantment_interaction, ctx).await;

    enchantment_interaction.data.values.get(0).unwrap().to_string()
}

async fn select_trait(part: &Gear, user: &User, ctx: &Context) -> String {
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

async fn confirm_trait_for_all(part: &Gear, current: &str, user: &User, ctx: &Context) -> bool {
    let msg = user.dm(&ctx.http, |msg| msg
        .content(format!("Quieres el rasgo __**{}**__ para el resto de __**{}**__?", current, get_gear_text(part)))
        .components(|c| c.add_action_row(components::confirmation_row("trait_yes")))
    ).await.unwrap();

    let trait_interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&trait_interaction, ctx).await;

    trait_interaction.data.custom_id == "trait_yes"
}

async fn confirm_enchantment_for_all(part: &Gear, current: &str, user: &User, ctx: &Context) -> bool {
    let msg = user.dm(&ctx.http, |msg| msg
        .content(format!("Quieres el encantamiento __**{}**__ para el resto de __**{}**__?", current, get_gear_text(part)))
        .components(|c| c.add_action_row(components::confirmation_row("enchantment_yes")))
    ).await.unwrap();

    let enchantment_interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&enchantment_interaction, ctx).await;

    enchantment_interaction.data.custom_id == "enchantment_yes"
}

async fn confirm_quality_for_all(part: &Gear, current: &str, user: &User, ctx: &Context) -> bool {
    let msg = user.dm(&ctx.http, |msg| msg
        .content(format!("Quieres la calidad __**{}**__ para el resto de __**{}**__?", current, get_gear_text(part)))
        .components(|c| c.add_action_row(components::confirmation_row("quality_yes")))
    ).await.unwrap();

    let quality_interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&quality_interaction, ctx).await;

    quality_interaction.data.custom_id == "quality_yes"
}

async fn confirm_weight_for_all(current: &str, user: &User, ctx: &Context) -> bool {
    let msg = user.dm(&ctx.http, |msg| msg
        .content(format!("Quieres el peso __**{}**__ para el resto de __**Armadura**__?", current))
        .components(|c| c.add_action_row(components::confirmation_row("weight_yes")))
    ).await.unwrap();

    let weight_interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&weight_interaction, ctx).await;

    weight_interaction.data.custom_id == "weight_yes"
}

fn get_gear_text(part: &Gear) -> &'static str {
    match part {
        Gear::Weapon(_) => "Armas",
        Gear::Armour(_) => "Armadura",
        Gear::Jewelry(_) => "Joyeria"
    }
}


async fn select_quality(part: &Gear, user: &User, ctx: &Context) -> String {
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

async fn select_weight(part: &ArmourParts, user: &User, ctx: &Context) -> String {
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