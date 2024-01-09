use std::fmt::Display;
use std::str::FromStr;
use regex::Regex;
use serenity::all::{ButtonStyle, ComponentInteraction, ComponentInteractionDataKind, Context, CreateActionRow, CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, CreateModal, CreateSelectMenu, Mention, ModalInteraction, User};
use serenity::all::ActionRowComponent::InputText;
use tracing::info;
use crate::components;
use crate::components::SetPiece;
use crate::entities::armour::{Armour, ArmourEnchantments, ArmourParts, ArmourTraits, ArmourWeights};
use crate::entities::{Gear, GearQuality};
use crate::entities::jewelry::{Jewelries, Jewelry, JewelryEnchantments, JewelryTraits};
use crate::entities::weapon::{Weapon, WeaponEnchantments, WeaponKind, WeaponTraits};
use crate::requests::{await_component_interaction, ok_response};

pub async fn gear(interaction: ComponentInteraction, ctx: &Context) {
    let response = CreateInteractionResponse::Modal(
        CreateModal::new("gear_set_modal", "⚒️ Solicitud de Equipamiento ⚒️")
            .components(vec![CreateActionRow::InputText(components::gear_set_modal("gear_set"))])
    );
    if let Err(why) = interaction.create_response(&ctx.http, response).await {
        info!("Cannot respond to gear request: {}", why)
    }
}

pub async fn gear_research(interaction: ComponentInteraction, ctx: &Context) {
    if let Err(why) = interaction
        .create_response(&ctx.http, CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new())).await {
        info!("Cannot respond to gear request: {}", why)
    } else {
        let parts = select_gear_research_options(&interaction.user, ctx).await;
        let re = Regex::new(r"<@&\d+>").unwrap();
        let msg = &interaction.message;
        let role = re.captures(&msg.content).unwrap()
            .get(0).unwrap().as_str();
        let material_cost = components::display_research_cost(&parts);
        if confirm_set("Investigar", &parts, &interaction.user, ctx).await {
            interaction.channel_id.send_message(&ctx.http, CreateMessage::new()
                .content(format!("{}\n\n__**‼️Peticion de Investigación para {}‼️**__\n\n", role, Mention::User(interaction.user.id)))
                .add_embed(components::gear_research_piece_embed(&parts))
                .add_embed(material_cost)
            ).await.unwrap();
        }
    }
}

pub async fn gear_modal(interaction: ModalInteraction, ctx: &Context) {
    if let Err(why) = interaction
        .create_response(&ctx.http, CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new())).await {
        info!("Cannot respond to gear set modal: {}", why)
    } else {
        if let InputText(input) = interaction
            .data.components.get(0).unwrap()
            .components.get(0).unwrap()
        {
            let input_value = input.value.clone().unwrap();
            let parts = select_gear_set_options(&input_value, &interaction.user, ctx).await;
            let re = Regex::new(r"<@&\d+>").unwrap();
            let msg = &interaction.message.unwrap();
            let role = re.captures(&msg.content).unwrap()
                .get(0).unwrap().as_str();

            let material_cost = if confirm_dialog("Este equipo es CP160?", &interaction.user, ctx).await {
                Some(components::display_cost(&parts))
            } else { None };
            if confirm_set(&input_value, &parts, &interaction.user, ctx).await {
                let mut embeds = vec![components::gear_set_piece_embed(&input_value, &parts)];
                if let Some(materials) = material_cost {
                    embeds.push(materials);
                }
                interaction.channel_id.send_message(&ctx.http, CreateMessage::new()
                    .content(format!("{}\n\n__**‼️Peticion de Equipamiento para {}‼️**__\n\n", role, Mention::User(interaction.user.id)))
                    .embeds(embeds)).await.unwrap();
            }
        }
    }
}

async fn confirm_set(name: &str, set: &Vec<SetPiece>, user: &User, ctx: &Context) -> bool {
    let msg = user.dm(&ctx.http, CreateMessage::new()
        .content("Esta es la configuración elegida, **esta todo bien?**")
        .embed(components::gear_result_embed(set, name))
        .components(vec![
            CreateActionRow::Buttons(vec![
                CreateButton::new("confirm_set")
                    .label("Confirmar")
                    .style(ButtonStyle::Success)
            ])
        ])
    ).await.unwrap();

    let interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&interaction, ctx).await;

    interaction.data.custom_id == "confirm_set"
}

async fn select_gear_research_options(user: &User, ctx: &Context) -> Vec<SetPiece> {
    let selected_parts = select_parts("Investigar", user, ctx).await;

    let (weapons, armour, jewelry) = parse_gear_parts(selected_parts);

    let mut set_parts: Vec<SetPiece> = Vec::new();
    set_parts.append(&mut process_weapons_research(weapons, user, ctx).await);
    set_parts.append(&mut process_armour_research(armour, user, ctx).await);
    set_parts.append(&mut process_jewelry_research(jewelry, user, ctx).await);
    set_parts
}

async fn select_gear_set_options(set: &str, user: &User, ctx: &Context) -> Vec<SetPiece> {
    info!("Gear set: {}", set);
    let selected_parts = select_parts(set, user, ctx).await;

    let (weapons, armour, jewelry) = parse_gear_parts(selected_parts);

    let with_enchantments = confirm_dialog("Quieres tambien encantamientos para el set?", user, ctx).await;
    let mut set_parts: Vec<SetPiece> = Vec::new();
    set_parts.append(&mut process_weapons(weapons, user, ctx, with_enchantments).await);
    set_parts.append(&mut process_armour(armour, user, ctx, with_enchantments).await);
    set_parts.append(&mut process_jewelry(jewelry, user, ctx, with_enchantments).await);
    set_parts
}

fn parse_gear_parts(parts: Vec<Gear>) -> (Vec<WeaponKind>, Vec<ArmourParts>, Vec<Jewelries>) {
    let weapons: Vec<WeaponKind> = parts.iter()
        .filter_map(|g| if let Gear::Weapon(w) = g {Some(w.clone())} else {None})
        .collect();
    let armour: Vec<ArmourParts> = parts.iter()
        .filter_map(|g| if let Gear::Armour(a) = g {Some(a.clone())} else {None})
        .collect();
    let jewelry: Vec<Jewelries> = parts.iter()
        .filter_map(|g| if let Gear::Jewelry(j) = g {Some(j.clone())} else {None})
        .collect();

    (weapons, armour, jewelry)
}

async fn process_weapons_research(weapons: Vec<WeaponKind>, user: &User, ctx: &Context) -> Vec<SetPiece> {
    let mut processed_weapons: Vec<SetPiece> = Vec::new();
    for weapon in weapons {
        let piece = SetPiece::Weapon(select_weapon_feats(&weapon, user, ctx, false, false).await);
        show_piece(&piece, user, ctx).await;
        processed_weapons.push(piece);
    }

    processed_weapons
}

async fn process_weapons(weapons: Vec<WeaponKind>, user: &User, ctx: &Context, with_enchantments: bool) -> Vec<SetPiece> {
    let mut processed_weapons: Vec<SetPiece> = Vec::new();
    for weapon in weapons {
        match weapon {
            WeaponKind::OneHanded(_) => {
                let question = format!("Has pedido una __**{}**__ de una mano. Quieres otra para ir con armas duales?", &weapon);
                if confirm_dialog(&question, user, ctx).await {
                    let piece = SetPiece::Weapon(select_weapon_feats(&weapon, user, ctx, with_enchantments, true).await);
                    show_piece(&piece, user, ctx).await;
                    processed_weapons.push(piece);
                }
                let piece = SetPiece::Weapon(select_weapon_feats(&weapon, user, ctx, with_enchantments, true).await);
                show_piece(&piece, user, ctx).await;
                processed_weapons.push(piece);
            },
            WeaponKind::TwoHanded(_) => {
                let piece = SetPiece::Weapon(select_weapon_feats(&weapon, user, ctx, with_enchantments, true).await);
                show_piece(&piece, user, ctx).await;
                processed_weapons.push(piece);
            }
        };
    }

    processed_weapons
}

async fn select_weapon_feats(weapon: &WeaponKind, user: &User, ctx: &Context, with_enchantments: bool, with_quality: bool) -> Weapon {
    let selected_trait = select_weapon_trait(&weapon, user, ctx).await;

    Weapon {
        kind: weapon.clone(),
        weapon_trait: selected_trait,
        enchantment: if with_enchantments {Some(select_weapon_enchantment(&weapon, user, ctx).await)} else {None},
        quality: if with_quality {select_quality(&weapon, user, ctx).await} else {GearQuality::White},
    }
}

async fn process_jewelry_research(jewelries: Vec<Jewelries>, user: &User, ctx: &Context) -> Vec<SetPiece> {
    let mut processed_jewelry: Vec<SetPiece> = Vec::new();

    for jewelry in jewelries {
        let piece = SetPiece::Jewelry(Jewelry {
            kind: jewelry.clone(),
            jewelry_trait: select_jewelry_trait(&jewelry, user, ctx).await,
            enchantment: None,
            quality: GearQuality::White
        });
        show_piece(&piece, user, ctx).await;
        processed_jewelry.push(piece);
    }

    processed_jewelry
}

async fn process_jewelry(jewelries: Vec<Jewelries>, user: &User, ctx: &Context, with_enchantments: bool) -> Vec<SetPiece> {
    let mut processed_jewelry: Vec<SetPiece> = Vec::new();

    let mut jewelries = jewelries.clone();
    if jewelries.contains(&Jewelries::Ring) {
        jewelries.push(Jewelries::Ring);
    }

    let mut default_trait: Option<JewelryTraits> = None;
    let mut default_enchantment: Option<JewelryEnchantments> = None;
    let mut default_quality: Option<GearQuality> = None;

    if jewelries.len() > 1 {
        let sample = jewelries.pop().unwrap();
        let feats = select_jewelry_feats(&sample, user, ctx, with_enchantments).await;
        let question = format!("Aplicar __**{}**__ al resto de __**Joyeria**__?", &feats.jewelry_trait);
        if confirm_dialog(&question, user, ctx).await {
            default_trait = Some(feats.jewelry_trait.clone());
        }
        if with_enchantments {
            let question = format!("Aplicar __**{}**__ al resto de __**Joyeria**__?", &feats.enchantment.clone().unwrap());
            if confirm_dialog(&question, user, ctx).await {
                default_enchantment = Some(feats.enchantment.clone().unwrap());
            }
        }
        let question = format!("Aplicar __**{}**__ al resto de __**Joyeria**__?", &feats.quality);
        if confirm_dialog(&question, user, ctx).await {
            default_quality = Some(feats.quality.clone());
        }
        let piece = SetPiece::Jewelry(feats);
        show_piece(&piece, user, ctx).await;
        processed_jewelry.push(piece);
    }

    for jewelry in jewelries {
        let selected_trait = if let Some(default_trait) = &default_trait {default_trait.clone()} else {select_jewelry_trait(&jewelry, user, ctx).await};
        let quality = if let Some(default_quality) = &default_quality {default_quality.clone()} else {select_quality(&jewelry, user, ctx).await};

        let piece = SetPiece::Jewelry(Jewelry {
            kind: jewelry.clone(),
            jewelry_trait: selected_trait,
            enchantment: if with_enchantments {
                Some(if let Some(default_enchantment) = &default_enchantment {default_enchantment.clone()} else {select_jewelry_enchantment(&jewelry, user, ctx).await})
            } else { None },
            quality,
        });
        show_piece(&piece, user, ctx).await;
        processed_jewelry.push(piece);
    }

    processed_jewelry
}

async fn select_jewelry_feats(jewelry: &Jewelries, user: &User, ctx: &Context, with_enchantments: bool) -> Jewelry {
    let selected_trait = select_jewelry_trait(&jewelry, user, ctx).await;
    let quality = select_quality(&jewelry, user, ctx).await;

    Jewelry {
        kind: jewelry.clone(),
        jewelry_trait: selected_trait,
        enchantment: if with_enchantments {Some(select_jewelry_enchantment(&jewelry, user, ctx).await)} else { None },
        quality,
    }
}

async fn process_armour_research(armour_parts: Vec<ArmourParts>, user: &User, ctx: &Context) -> Vec<SetPiece> {
    let mut processed_armour: Vec<SetPiece> = Vec::new();

    for armour in armour_parts {
        let selected_trait = select_armour_trait(&armour, user, ctx).await;
        let weight = select_weight(&armour, user, ctx).await;

        let piece = SetPiece::Armour(Armour {
            kind: armour.clone(),
            armour_trait: selected_trait,
            enchantment: None,
            weight,
            quality: GearQuality::White,
        });
        show_piece(&piece, user, ctx).await;
        processed_armour.push(piece);
    }

    processed_armour
}

async fn process_armour(armour_parts: Vec<ArmourParts>, user: &User, ctx: &Context, with_enchantments: bool) -> Vec<SetPiece> {
    let mut processed_armour: Vec<SetPiece> = Vec::new();

    let mut default_trait: Option<ArmourTraits> = None;
    let mut default_enchantment: Option<ArmourEnchantments> = None;
    let mut default_weight: Option<ArmourWeights> = None;
    let mut default_quality: Option<GearQuality> = None;

    let mut armour_parts = armour_parts.clone();
    if armour_parts.len() > 1 {
        let sample = armour_parts.pop().unwrap();
        let feats = select_armour_feats(&sample, user, ctx, with_enchantments).await;
        let question = format!("Aplicar __**{}**__ al resto de la __**Armadura**__?", &feats.armour_trait);
        if confirm_dialog(&question, user, ctx).await {
            default_trait = Some(feats.armour_trait.clone());
        }
        if with_enchantments {
            let question = format!("Aplicar __**{}**__ al resto de la __**Armadura**__?", &feats.enchantment.clone().unwrap());
            if confirm_dialog(&question, user, ctx).await {
                default_enchantment = Some(feats.enchantment.clone().unwrap());
            }
        }
        let question = format!("Aplicar __**{}**__ al resto de la __**Armadura**__?", &feats.weight);
        if confirm_dialog(&question, user, ctx).await {
            default_weight = Some(feats.weight.clone());
        }
        let question = format!("Aplicar __**{}**__ al resto de la __**Armadura**__?", &feats.quality);
        if confirm_dialog(&question, user, ctx).await {
            default_quality = Some(feats.quality.clone());
        }
        let piece = SetPiece::Armour(feats);
        show_piece(&piece, user, ctx).await;
        processed_armour.push(piece);
    }

    for armour in armour_parts {
        let selected_trait = if let Some(default_trait) = &default_trait {default_trait.clone()} else {select_armour_trait(&armour, user, ctx).await};
        let weight = if let Some(default_weight) = &default_weight {default_weight.clone()} else {select_weight(&armour, user, ctx).await};
        let quality = if let Some(default_quality) = &default_quality {default_quality.clone()} else {select_quality(&armour, user, ctx).await};

        let piece = SetPiece::Armour(Armour {
            kind: armour.clone(),
            armour_trait: selected_trait,
            enchantment: if with_enchantments {
                Some(if let Some(default_enchantment) = &default_enchantment {default_enchantment.clone()} else {select_armour_enchantment(&armour, user, ctx).await})
            } else { None },
            weight,
            quality,
        });
        show_piece(&piece, user, ctx).await;
        processed_armour.push(piece);
    }

    processed_armour
}

async fn select_armour_feats(armour: &ArmourParts, user: &User, ctx: &Context, with_enchantments: bool) -> Armour {
    let selected_trait = select_armour_trait(&armour, user, ctx).await;
    let quality = select_quality(&armour, user, ctx).await;
    let weight = select_weight(&armour, user, ctx).await;

    Armour {
        kind: armour.clone(),
        armour_trait: selected_trait,
        enchantment: if with_enchantments {Some(select_armour_enchantment(&armour, user, ctx).await)} else { None },
        weight,
        quality,
    }
}

async fn select_parts(set: &str, user: &User, ctx: &Context) -> Vec<Gear> {
    let msg = user.dm(&ctx.http, CreateMessage::new()
        .embed(components::gear_set_embed(set))
        .components(vec![CreateActionRow::SelectMenu(components::gear_set_parts("gear_set_parts"))])
    ).await.unwrap();

    let interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&interaction, ctx).await;

    if let ComponentInteractionDataKind::StringSelect {values} = interaction.data.kind {
        values.iter()
            .filter_map(|f| Gear::from_str(f).ok())
            .collect()
    } else { vec![] }
}

async fn select_weapon_enchantment(weapon: &WeaponKind, user: &User, ctx: &Context) -> WeaponEnchantments {
    let menu = components::enchantments::gear_weapon_enchantments("weapon_enchantment");
    let selected_enchantment = select_feat(user, ctx, "encantamiento", weapon, menu).await;

    WeaponEnchantments::from_str(&selected_enchantment).unwrap()
}

async fn select_armour_enchantment(armour: &ArmourParts, user: &User, ctx: &Context) -> ArmourEnchantments {
    let menu = components::enchantments::gear_armour_enchantments("armour_enchantment");
    let selected_enchantment = select_feat(user, ctx, "encantamiento", armour, menu).await;

    ArmourEnchantments::from_str(&selected_enchantment).unwrap()
}

async fn select_jewelry_enchantment(jewelry: &Jewelries, user: &User, ctx: &Context) -> JewelryEnchantments {
    let menu = components::enchantments::gear_jewelry_enchantments("jewelry_enchantment");
    let selected_enchantment = select_feat(user, ctx, "encantamiento", jewelry, menu).await;

    JewelryEnchantments::from_str(&selected_enchantment).unwrap()
}

async fn select_weapon_trait(weapon: &WeaponKind, user: &User, ctx: &Context) -> WeaponTraits {
    let menu = components::traits::gear_weapon_traits("weapon_trait");
    let selected_trait = select_feat(user, ctx, "rasgo", weapon, menu).await;

    WeaponTraits::from_str(&selected_trait).unwrap()
}

async fn select_armour_trait(armour: &ArmourParts, user: &User, ctx: &Context) -> ArmourTraits {
    let menu = components::traits::gear_armour_traits("armour_trait");
    let selected_trait = select_feat(user, ctx, "rasgo", armour, menu).await;

    ArmourTraits::from_str(&selected_trait).unwrap()
}

async fn select_jewelry_trait(jewelry: &Jewelries, user: &User, ctx: &Context) -> JewelryTraits {
    let menu = components::traits::gear_jewelry_traits("jewelry_trait");
    let selected_trait = select_feat(user, ctx, "rasgo", jewelry, menu).await;

    JewelryTraits::from_str(&selected_trait).unwrap()
}

async fn select_quality<F>(part: F, user: &User, ctx: &Context) -> GearQuality
    where F: Display
{
    let menu = components::gear_quality("quality");
    let selected_quality = select_feat(user, ctx, "calidad", part, menu).await;

    GearQuality::from_str(&selected_quality).unwrap()
}

async fn select_weight(part: &ArmourParts, user: &User, ctx: &Context) -> ArmourWeights {
    let msg = user.dm(&ctx.http, CreateMessage::new()
        .content(format!("Selecciona **peso** de la pieza: __**{}**__", part))
        .components(vec![CreateActionRow::SelectMenu(components::armor_weight("weight"))])
    ).await.unwrap();

    let weight_interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&weight_interaction, ctx).await;

    if let ComponentInteractionDataKind::StringSelect {values} = weight_interaction.data.kind {
        ArmourWeights::from_str(&values.get(0).unwrap().to_string()).unwrap()
    } else {
        panic!("rework pending")
    }

}

async fn select_feat<F: Display>(user: &User, ctx: &Context, feat: &str, item: F, menu: CreateSelectMenu) -> String {
    let msg = user.dm(&ctx.http, CreateMessage::new()
        .content(format!("Selecciona **{}** para __**{}**__", feat, item))
        .components(vec![CreateActionRow::SelectMenu(menu)])
    ).await.unwrap();

    let interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&interaction, ctx).await;

    if let ComponentInteractionDataKind::StringSelect {values} = interaction.data.kind {
        values.get(0).unwrap().to_string()
    } else { String::new() }
}

async fn show_piece(piece: &SetPiece, user: &User, ctx: &Context) {
    user.dm(&ctx.http, CreateMessage::new()
        .embed(components::gear_piece_embed(piece))
    ).await.unwrap();
}

async fn confirm_dialog(question: &str, user: &User, ctx: &Context) -> bool {
    let msg = user.dm(&ctx.http, CreateMessage::new()
        .content(question)
        .components(vec![components::confirmation_row("confirm_yes")])
    ).await.unwrap();

    let interaction = await_component_interaction(msg, ctx).await.unwrap();
    ok_response(&interaction, ctx).await;

    interaction.data.custom_id == "confirm_yes"
}