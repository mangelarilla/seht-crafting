pub mod traits;
pub mod enchantments;

use std::collections::HashMap;
use serenity::builder::{CreateActionRow, CreateEmbed, CreateInputText, CreateSelectMenu};
use serenity::model::prelude::component::*;
use serenity::model::prelude::*;
use strum::{EnumMessage, EnumProperty, IntoEnumIterator};
use crate::entities::armour::{Armour, ArmourParts, ArmourWeights};
use crate::entities::{GearQuality, MaterialCost};
use crate::entities::jewelry::{Jewelries, Jewelry};
use crate::entities::weapon::{OneHandedWeapons, TwoHandedWeapons, Weapon};
use std::string::ToString;

pub enum SetPiece {
    Weapon(Weapon),
    Armour(Armour),
    Jewelry(Jewelry)
}

impl MaterialCost for SetPiece {
    fn cost(&self) -> Vec<(i32, String)> {
        match self {
            SetPiece::Weapon(w) => w.cost(),
            SetPiece::Armour(a) => a.cost(),
            SetPiece::Jewelry(j) => j.cost()
        }
    }
}

trait ResearchCost {
    fn research_cost(&self) -> Vec<(i32, String)>;
}

impl ResearchCost for SetPiece {
    fn research_cost(&self) -> Vec<(i32, String)> {
        match self {
            SetPiece::Weapon(w) => w.weapon_trait.cost(),
            SetPiece::Armour(a) => a.armour_trait.cost(),
            SetPiece::Jewelry(j) => j.jewelry_trait.cost()
        }
    }
}

pub fn display_research_cost(pieces: &Vec<SetPiece>) -> CreateEmbed {
    let mut b = CreateEmbed::default();
    let mut costs: HashMap<String, i32> = HashMap::new();
    for (amount, material) in pieces.iter()
        .map(|p| p.research_cost())
        .flatten()
    {
        if let Some(old_amount) = costs.insert(material.clone(), amount) {
            costs.insert(material.clone(), amount + old_amount);
        }
    }
    b.title("Materiales");
    b.description("Lista de los materiales de investigación necesarios para este equipo");
    for (material, amount) in costs {
        b.field(material, amount, true);
    }
    b
}

pub fn display_cost(pieces: &Vec<SetPiece>) -> CreateEmbed {
    let mut b = CreateEmbed::default();
    let mut costs: HashMap<String, i32> = HashMap::new();
    for (amount, material) in pieces.iter()
        .map(|p| p.cost())
        .flatten()
    {
        if let Some(old_amount) = costs.insert(material.clone(), amount) {
            costs.insert(material.clone(), amount + old_amount);
        }
    }
    b.title("Materiales");
    b.description("Lista de los materiales necesarios para este equipo asumiendo CP160");
    for (material, amount) in costs {
        b.field(material, amount, true);
    }
    b
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
                .value(jewelry.to_string())
                .description(jewelry.get_documentation().unwrap()));
        }
        for weapon in TwoHandedWeapons::iter() {
            opts.create_option(|o| o
                .label(weapon.to_string())
                .value(weapon.to_string())
                .description(weapon.get_documentation().unwrap()));
        }
        for weapon in OneHandedWeapons::iter() {
            opts.create_option(|o| o
                .label(weapon.to_string())
                .value(weapon.to_string())
                .description(weapon.get_documentation().unwrap()));
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
                .emoji(ReactionType::Unicode(quality.get_str("Emoji").unwrap().to_string())));
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

pub fn gear_research_piece_embed(pieces: &Vec<SetPiece>) -> CreateEmbed {
    let mut b = CreateEmbed::default();
    b.title("🛠️ Investigación 🛠️️");
    b.color((127,255,0));
    for piece in pieces {
        match piece {
            SetPiece::Weapon(w) => {
                b.field(&w.kind.to_string(), "", false);
                b.field(":hourglass: Rasgo", &w.weapon_trait.to_string(), true);
            }
            SetPiece::Armour(a) => {
                b.field(&a.kind.to_string(), "", false);
                b.color((127,255,0));
                b.field(":lifter: Peso", &a.weight.to_string(), true);
                b.field(":hourglass: Rasgo", &a.armour_trait.to_string(), true);
            }
            SetPiece::Jewelry(j) => {
                b.field(&j.kind.to_string(), "", false);
                b.color((127,255,0));
                b.field(":hourglass: Rasgo", &j.jewelry_trait.to_string(), true);
            }
        }
    }
    b
}


pub fn gear_set_piece_embed(set: &str, pieces: &Vec<SetPiece>) -> CreateEmbed {
    let mut b = CreateEmbed::default();
    b.title(format!("🛠️ {} 🛠️️", set));
    b.color((127,255,0));
    for piece in pieces {
        match piece {
            SetPiece::Weapon(w) => {
                b.field(&w.kind.to_string(), "", false);
                b.field(":hourglass: Rasgo", &w.weapon_trait.to_string(), true);
                b.field(":gem: Calidad", &w.quality.to_string(), true);
                if let Some(enchantment) = &w.enchantment {
                    b.field(":magic_wand: Encantamiento", enchantment, false);
                }
            }
            SetPiece::Armour(a) => {
                b.field(&a.kind.to_string(), "", false);
                b.color((127,255,0));
                b.field(":lifter: Peso", &a.weight.to_string(), true);
                b.field(":hourglass: Rasgo", &a.armour_trait.to_string(), true);
                b.field(":gem: Calidad", &a.quality.to_string(), true);
                if let Some(enchantment) = &a.enchantment {
                    b.field(":magic_wand: Encantamiento", enchantment, false);
                }
            }
            SetPiece::Jewelry(j) => {
                b.field(&j.kind.to_string(), "", false);
                b.color((127,255,0));
                b.field(":hourglass: Rasgo", &j.jewelry_trait.to_string(), true);
                b.field(":gem: Calidad", &j.quality.to_string(), true);
                if let Some(enchantment) = &j.enchantment {
                    b.field(":magic_wand: Encantamiento", enchantment, false);
                }
            }
        }
    }
    b
}

pub fn gear_piece_embed(part: &SetPiece) -> CreateEmbed {
    let mut b = CreateEmbed::default();
    match part {
        SetPiece::Weapon(w) => {
            b.title(format!("🛠️ {} 🛠️️", &w.kind.to_string()));
            b.field("Rasgo", &w.weapon_trait.to_string(), true);
            b.field("Calidad", &w.quality.to_string(), true);
            if let Some(enchantment) = &w.enchantment {
                b.field("Encantamiento", enchantment, true);
            }
        }
        SetPiece::Armour(a) => {
            b.title(format!("🛠️ {} 🛠️", &a.kind.to_string()));
            b.field("Peso", &a.weight.to_string(), true);
            b.field("Rasgo", &a.armour_trait.to_string(), true);
            b.field("Calidad", &a.quality.to_string(), true);
            if let Some(enchantment) = &a.enchantment {
                b.field("Encantamiento", enchantment, true);
            }
        }
        SetPiece::Jewelry(j) => {
            b.title(format!("🛠️ {} 🛠️️", &j.kind.to_string()));
            b.field("Rasgo", &j.jewelry_trait.to_string(), true);
            b.field("Calidad", &j.quality.to_string(), true);
            if let Some(enchantment) = &j.enchantment {
                b.field("Encantamiento", enchantment, true);
            }
        }
    }
    b
}

pub fn gear_result_embed(set: &Vec<SetPiece>, name: &str) -> CreateEmbed {
    let mut b = CreateEmbed::default();
    b.title(format!("🛠️ {} 🛠️️", name));
    for piece in set {
        match piece {
            SetPiece::Weapon(w) => {
                b.field(&w.kind, "", false);
                b.field("Rasgo", &w.weapon_trait.to_string(), true);
                b.field("Calidad", &w.quality.to_string(), true);
                if let Some(enchantment) = &w.enchantment {
                    b.field("Encantamiento", enchantment, true);
                }
            }
            SetPiece::Armour(a) => {
                b.field(&a.kind, "", false);
                b.field("Peso", &a.weight.to_string(), true);
                b.field("Rasgo", &a.armour_trait.to_string(), true);
                b.field("Calidad", &a.quality.to_string(), true);
                if let Some(enchantment) = &a.enchantment {
                    b.field("Encantamiento", enchantment, true);
                }
            }
            SetPiece::Jewelry(j) => {
                b.field(&j.kind, "", false);
                b.field("Rasgo", &j.jewelry_trait.to_string(), true);
                b.field("Calidad", &j.quality.to_string(), true);
                if let Some(enchantment) = &j.enchantment {
                    b.field("Encantamiento", enchantment, true);
                }
            }
        }
    }
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

De cara a solicitar un crafting a los {} en el siguiente canal se deberá rellenar mediante los tres botones que aparecen al final de este mensaje, vease:
- **Equipamiento:** Para solicitar diversas piezas de set, incluyendo armas, armaduras y joyeria.
- **Consumibles:** Para 🍖 comida y 🧪 pociones.
- **Encantamientos:** Para los distintos glifos de armas, armadura y joyeria.

Se deberan enviar los materiales al fabricante que se encargue y, __**en caso de tener nivel CP300+ y solo en los encargos de equipamiento**__, abonar el pago al banco del gremio de __**{}**__ de oro por pieza, __**{}**__ de oro por set",
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
        .emoji(ReactionType::Custom {name: Some("potion".to_string()), id: EmojiId(1138123617482322031), animated: false})
        .label("Consumibles")
        .style(ButtonStyle::Success));
    b.create_button(|b| b
        .custom_id("Enchantment")
        .emoji(ReactionType::Unicode("🪄".to_string()))
        .label("Encantamientos")
        .style(ButtonStyle::Secondary));
    b.create_button(|b| b
        .custom_id("GearResearch")
        .emoji(ReactionType::Unicode("🔬".to_string()))
        .label("Investigar Rasgos")
        .style(ButtonStyle::Primary));
    b
}