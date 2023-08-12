pub mod traits;
pub mod enchantments;

use serenity::builder::{CreateActionRow, CreateEmbed, CreateInputText, CreateSelectMenu};
use serenity::model::prelude::component::*;
use serenity::model::prelude::*;
use strum::IntoEnumIterator;
use crate::entities::armour::{ArmourParts, ArmourWeights};
use crate::entities::{GearQuality, ItemEmoji, ItemInfo};
use crate::entities::jewelry::Jewelries;
use crate::entities::weapon::{OneHandedWeapons, TwoHandedWeapons};

#[derive(Debug)]
pub struct SetPiece {
    pub part: String,
    pub part_trait: String,
    pub weight: Option<String>,
    pub quality: String,
    pub enchantment: String
}

pub fn gear_set_parts(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona las partes del set que quieres");
    b.max_values(12);
    b.options(|opts| {
        for armour_part in ArmourParts::iter() {
            opts.create_option(|o| o
                .label(armour_part.to_string())
                .value(armour_part.to_string()));
        }
        for jewelry in Jewelries::iter() {
            opts.create_option(|o| o
                .label(jewelry.to_string())
                .value(jewelry.to_string()));
        }
        for weapon in TwoHandedWeapons::iter() {
            opts.create_option(|o| o
                .label(weapon.to_string())
                .value(weapon.to_string())
                .description(weapon.description()));
        }
        for weapon in OneHandedWeapons::iter() {
            opts.create_option(|o| o
                .label(weapon.to_string())
                .value(weapon.to_string())
                .description(weapon.description()));
        }
        opts
    });
    b
}

pub fn armor_weight(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el peso de la armadura");
    b.options(|opts| {
        for weight in ArmourWeights::iter() {
            opts.create_option(|o| o
                .label(weight.to_string())
                .value(weight.to_string()));
        }
        opts
    });
    b
}

pub fn gear_quality(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona la calidad de la pieza");
    b.options(|opts| {
        for quality in GearQuality::iter() {
            opts.create_option(|o| o
                .label(quality.to_string())
                .value(quality.to_string())
                .emoji(quality.emoji()));
        }
        opts
    });
    b
}

pub fn gear_set_embed(set: &str) -> CreateEmbed {
    let mut b = CreateEmbed::default();
    b.title(format!("🛡️ {} 🛡️", set));
    b.description("Configura el equipo que deseas con las opciones");
    b
}

pub fn gear_set_piece_embed(set: &str, part: &SetPiece) -> CreateEmbed {
    let mut b = CreateEmbed::default();
    b.title(format!("⚒️ {}: {} ️", set, &part.part));
    b.color((127,255,0));
    b.field(":hourglass: Rasgo", &part.part_trait, true);
    if let Some(weight) = &part.weight {
        b.field(":lifter: Peso", weight, true);
    }
    b.field(":gem: Calidad", &part.quality, true);
    b.field(":magic_wand: Encantamiento", &part.enchantment, false);
    b
}

pub fn gear_piece_embed(part: &SetPiece) -> CreateEmbed {
    let mut b = CreateEmbed::default();
    b.title(format!("🛠️ {} 🛠️", &part.part));
    b.field("Rasgo", &part.part_trait, true);
    if let Some(weight) = &part.weight {
        b.field("Peso", weight, true);
    }
    b.field("Calidad", &part.quality, true);
    b.field("Encantamiento", &part.enchantment, true);
    b
}

pub fn gear_set_modal(name: &str) -> CreateInputText {
    let mut b = CreateInputText::default();
    b.custom_id(name);
    b.placeholder("Cólera de la orden");
    b.label("Nombre del Set/Conjunto");
    b.style(InputTextStyle::Short);
    b
}

pub fn consumables_modal(name: &str) -> CreateInputText {
    let mut b = CreateInputText::default();
    b.custom_id(name);
    b.placeholder("- Poción de poder de hechizo (x20)\n- Sopa de zanahorioa (x10)");
    b.label("Pociones y Comida");
    b.style(InputTextStyle::Paragraph);
    b
}

pub fn enchantments_modal(name: &str) -> CreateInputText {
    let mut b = CreateInputText::default();
    b.custom_id(name);
    b.placeholder("- Glifo Realmente Soberbio de Magia (x2)\n- Glifo Realmente Soberbio de Vida (x2)");
    b.label("Glifos");
    b.style(InputTextStyle::Paragraph);
    b
}

pub fn menu_description(price: &f64, crafters: &Role) -> String {
    format!(
        "⚒️ __**Solicitud de crafting de equipamiento, consumibles o encantamientos**__ ⚒️

De cara a solicitar un crafting a los {} en el siguiente canal se deberá rellenar mediantes los tres botones que aparecen al final de este mensaje, vease:
- **Equipamiento:** Para solicitar diversas piezas de set, incluyendo armas, armaduras y joyeria.
- **Consumibles:** Para 🍖 comida y 🧪 pociones.
- **Encantamientos:** Para los distintos glifos de armas, armadura y joyeria.

Se deberan enviar los materiales al fabricante que se encargue y abonar el pago al banco del gremio de __**{}**__ de oro por pieza, __**{}**__ de oro por set",
        Mention::Role(crafters.id), price, *price * 5.0)
}

pub fn confirmation_row(name: &str) -> CreateActionRow {
    let mut b = CreateActionRow::default();

    b.create_button(|b| b
        .custom_id(name)
        .emoji(ReactionType::Unicode("👍".to_string()))
        .label("Si")
        .style(ButtonStyle::Success));
    b.create_button(|b| b
        .custom_id(format!("{}_no", name))
        .emoji(ReactionType::Unicode("👎".to_string()))
        .label("No")
        .style(ButtonStyle::Danger));
    b
}

pub fn menu_action_row() -> CreateActionRow {
    let mut b = CreateActionRow::default();

    b.create_button(|b| b
        .custom_id("Gear")
        .emoji(ReactionType::Unicode("⚔️".to_string()))
        .label("Equipamiento")
        .style(ButtonStyle::Primary));
    b.create_button(|b| b
        .custom_id("Consumables")
        .emoji(ReactionType::Unicode("🍖".to_string()))
        .label("Consumibles")
        .style(ButtonStyle::Success));
    b.create_button(|b| b
        .custom_id("Enchantment")
        .emoji(ReactionType::Unicode("🪄".to_string()))
        .label("Encantamientos")
        .style(ButtonStyle::Secondary));
    b
}