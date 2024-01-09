pub mod traits;
pub mod enchantments;

use std::collections::HashMap;
use std::fmt::Display;
use serenity::builder::{CreateActionRow, CreateEmbed, CreateInputText, CreateSelectMenu};
use strum::{EnumMessage, EnumProperty, IntoEnumIterator};
use crate::entities::armour::{Armour, ArmourParts, ArmourWeights};
use crate::entities::{GearQuality, MaterialCost};
use crate::entities::jewelry::{Jewelries, Jewelry};
use crate::entities::weapon::{OneHandedWeapons, TwoHandedWeapons, Weapon};
use std::string::ToString;
use serenity::all::{ButtonStyle, CreateButton, CreateSelectMenuKind, CreateSelectMenuOption, EmojiId, InputTextStyle, Mention, ReactionType, RoleId};

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
    display_material_cost(pieces.iter()
        .map(|p| p.research_cost())
        .flatten().collect())
}

pub fn display_cost(pieces: &Vec<SetPiece>) -> CreateEmbed {
    display_material_cost(pieces.iter()
        .map(|p| p.cost())
        .flatten().collect())
}

fn display_material_cost(cost: Vec<(i32, String)>) -> CreateEmbed {
    let mut b = CreateEmbed::new()
        .title("Materiales")
        .description("Lista de los materiales necesarios para este equipo");

    let mut costs: HashMap<String, i32> = HashMap::new();
    for (amount, material) in cost {
        if let Some(old_amount) = costs.insert(material.clone(), amount) {
            costs.insert(material.clone(), amount + old_amount);
        }
    }

    for (material, amount) in costs {
        b = b.field(material, amount.to_string(), true);
    }
    b
}

fn enum_to_options<T: IntoEnumIterator + EnumMessage + Display>() -> Vec<CreateSelectMenuOption> {
    T::iter()
        .map(|i| CreateSelectMenuOption::new(i.to_string(), i.to_string())
            .description(i.get_documentation().unwrap()))
        .collect()
}

pub fn gear_set_parts(name: &str) -> CreateSelectMenu {
    let options = CreateSelectMenuKind::String {
        options: [
            enum_to_options::<ArmourParts>().as_slice(),
            enum_to_options::<Jewelries>().as_slice(),
            enum_to_options::<OneHandedWeapons>().as_slice(),
            enum_to_options::<TwoHandedWeapons>().as_slice()
        ].concat()
    };

    CreateSelectMenu::new(name, options)
        .placeholder("Selecciona las partes del set que quieres")
        .max_values(12)
}

pub fn armor_weight(name: &str) -> CreateSelectMenu {
    let options = CreateSelectMenuKind::String {
        options: enum_to_options::<ArmourWeights>()
    };

    CreateSelectMenu::new(name, options)
        .placeholder("Selecciona el peso de la armadura")
}

pub fn gear_quality(name: &str) -> CreateSelectMenu {
    let options = CreateSelectMenuKind::String {
        options: GearQuality::iter()
            .map(|opt| CreateSelectMenuOption::new(opt.to_string(), opt.to_string())
                .emoji(ReactionType::Unicode(opt.get_str("Emoji").unwrap().to_string())))
            .collect()
    };

    CreateSelectMenu::new(name, options)
        .placeholder("Selecciona la calidad de la pieza")
}

pub fn gear_set_embed(set: &str) -> CreateEmbed {
    CreateEmbed::new()
        .title(format!("🛡️ {} 🛡️", set))
        .description("Configura el equipo que deseas con las opciones")
}

pub fn gear_research_piece_embed(pieces: &Vec<SetPiece>) -> CreateEmbed {
    let mut b = CreateEmbed::new()
        .title("🛠️ Investigación 🛠️️")
        .color((127,255,0));

    for piece in pieces {
        match piece {
            SetPiece::Weapon(w) => {
                b = b.field(&w.kind.to_string(), "", false);
                b = b.field(":hourglass: Rasgo", &w.weapon_trait.to_string(), true);
            }
            SetPiece::Armour(a) => {
                b = b.field(&a.kind.to_string(), "", false);
                b = b.color((127,255,0));
                b = b.field(":lifter: Peso", &a.weight.to_string(), true);
                b = b.field(":hourglass: Rasgo", &a.armour_trait.to_string(), true);
            }
            SetPiece::Jewelry(j) => {
                b = b.field(&j.kind.to_string(), "", false);
                b = b.color((127,255,0));
                b = b.field(":hourglass: Rasgo", &j.jewelry_trait.to_string(), true);
            }
        }
    }
    b
}


pub fn gear_set_piece_embed(set: &str, pieces: &Vec<SetPiece>) -> CreateEmbed {
    let mut b = CreateEmbed::new()
        .title(format!("🛠️ {} 🛠️️", set))
        .color((127,255,0));

    for piece in pieces {
        match piece {
            SetPiece::Weapon(w) => {
                b = b.field(&w.kind.to_string(), "", false);
                b = b.field(":hourglass: Rasgo", &w.weapon_trait.to_string(), true);
                b = b.field(":gem: Calidad", &w.quality.to_string(), true);
                if let Some(enchantment) = &w.enchantment {
                    b = b.field(":magic_wand: Encantamiento", enchantment.to_string(), false);
                }
            }
            SetPiece::Armour(a) => {
                b = b.field(&a.kind.to_string(), "", false);
                b = b.color((127,255,0));
                b = b.field(":lifter: Peso", &a.weight.to_string(), true);
                b = b.field(":hourglass: Rasgo", &a.armour_trait.to_string(), true);
                b = b.field(":gem: Calidad", &a.quality.to_string(), true);
                if let Some(enchantment) = &a.enchantment {
                    b = b.field(":magic_wand: Encantamiento", enchantment.to_string(), false);
                }
            }
            SetPiece::Jewelry(j) => {
                b = b.field(&j.kind.to_string(), "", false);
                b = b.color((127,255,0));
                b = b.field(":hourglass: Rasgo", &j.jewelry_trait.to_string(), true);
                b = b.field(":gem: Calidad", &j.quality.to_string(), true);
                if let Some(enchantment) = &j.enchantment {
                    b = b.field(":magic_wand: Encantamiento", enchantment.to_string(), false);
                }
            }
        }
    }
    b
}

pub fn gear_piece_embed(part: &SetPiece) -> CreateEmbed {
    let mut b = CreateEmbed::new();
    match part {
        SetPiece::Weapon(w) => {
            b = b.title(format!("🛠️ {} 🛠️️", &w.kind.to_string()));
            b = b.field("Rasgo", &w.weapon_trait.to_string(), true);
            b = b.field("Calidad", &w.quality.to_string(), true);
            if let Some(enchantment) = &w.enchantment {
                b = b.field("Encantamiento", enchantment.to_string(), true);
            }
        }
        SetPiece::Armour(a) => {
            b = b.title(format!("🛠️ {} 🛠️", &a.kind.to_string()));
            b = b.field("Peso", &a.weight.to_string(), true);
            b = b.field("Rasgo", &a.armour_trait.to_string(), true);
            b = b.field("Calidad", &a.quality.to_string(), true);
            if let Some(enchantment) = &a.enchantment {
                b = b.field("Encantamiento", enchantment.to_string(), true);
            }
        }
        SetPiece::Jewelry(j) => {
            b = b.title(format!("🛠️ {} 🛠️️", &j.kind.to_string()));
            b = b.field("Rasgo", &j.jewelry_trait.to_string(), true);
            b = b.field("Calidad", &j.quality.to_string(), true);
            if let Some(enchantment) = &j.enchantment {
                b = b.field("Encantamiento", enchantment.to_string(), true);
            }
        }
    }
    b
}

pub fn gear_result_embed(set: &Vec<SetPiece>, name: &str) -> CreateEmbed {
    let mut b = CreateEmbed::new();
    b = b.title(format!("🛠️ {} 🛠️️", name));
    for piece in set {
        match piece {
            SetPiece::Weapon(w) => {
                b = b.field(&w.kind.to_string(), "", false);
                b = b.field("Rasgo", &w.weapon_trait.to_string(), true);
                b = b.field("Calidad", &w.quality.to_string(), true);
                if let Some(enchantment) = &w.enchantment {
                    b = b.field("Encantamiento", enchantment.to_string(), true);
                }
            }
            SetPiece::Armour(a) => {
                b = b.field(&a.kind.to_string(), "", false);
                b = b.field("Peso", &a.weight.to_string(), true);
                b = b.field("Rasgo", &a.armour_trait.to_string(), true);
                b = b.field("Calidad", &a.quality.to_string(), true);
                if let Some(enchantment) = &a.enchantment {
                    b = b.field("Encantamiento", enchantment.to_string(), true);
                }
            }
            SetPiece::Jewelry(j) => {
                b = b.field(&j.kind.to_string(), "", false);
                b = b.field("Rasgo", &j.jewelry_trait.to_string(), true);
                b = b.field("Calidad", &j.quality.to_string(), true);
                if let Some(enchantment) = &j.enchantment {
                    b = b.field("Encantamiento", enchantment.to_string(), true);
                }
            }
        }
    }
    b
}

pub fn gear_set_modal(name: &str) -> CreateInputText {
    CreateInputText::new(InputTextStyle::Short, "Nombre del Set/Conjunto", name)
        .placeholder("Cólera de la orden")
}

pub fn consumables_modal(name: &str) -> CreateInputText {
    CreateInputText::new(InputTextStyle::Paragraph, "Pociones y Comida", name)
        .placeholder("- Poción de poder de hechizo (x20)\n- Sopa de zanahorioa (x10)")
}

pub fn enchantments_modal(name: &str) -> CreateInputText {
    CreateInputText::new(InputTextStyle::Paragraph, "Glifos", name)
        .placeholder("- Glifo Realmente Soberbio de Magia (x2)\n- Glifo Realmente Soberbio de Vida (x2)")
}

pub fn menu_description(price: &f64, crafters: RoleId) -> String {
    format!(
        "⚒️ __**Solicitud de crafting de equipamiento, consumibles o encantamientos**__ ⚒️

De cara a solicitar un crafting a los {} en el siguiente canal se deberá rellenar mediante los tres botones que aparecen al final de este mensaje, vease:
- **Equipamiento:** Para solicitar diversas piezas de set, incluyendo armas, armaduras y joyeria.
- **Consumibles:** Para 🍖 comida y 🧪 pociones.
- **Encantamientos:** Para los distintos glifos de armas, armadura y joyeria.

Se deberan enviar los materiales al fabricante que se encargue y, __**en caso de tener nivel CP300+ y solo en los encargos de equipamiento**__, abonar el pago al banco del gremio de __**{}**__ de oro por pieza, __**{}**__ de oro por set",
        Mention::Role(crafters), price, *price * 5.0)
}

pub fn confirmation_row(name: &str) -> CreateActionRow {
    CreateActionRow::Buttons(vec![
        CreateButton::new(name)
            .emoji(ReactionType::Unicode("👍".to_string()))
            .label("Si")
            .style(ButtonStyle::Success),
        CreateButton::new(name)
            .custom_id(format!("{}_no", name))
            .emoji(ReactionType::Unicode("👎".to_string()))
            .label("No")
            .style(ButtonStyle::Danger)
    ])
}

pub fn menu_action_row() -> CreateActionRow {
    CreateActionRow::Buttons(vec![
        CreateButton::new("Gear")
            .emoji(ReactionType::Unicode("⚔️".to_string()))
            .label("Equipamiento")
            .style(ButtonStyle::Primary),
        CreateButton::new("Consumables")
            .emoji(ReactionType::Custom {name: Some("potion".to_string()), id: EmojiId::new(1138123617482322031), animated: false})
            .label("Consumibles")
            .style(ButtonStyle::Success),
        CreateButton::new("Enchantment")
            .emoji(ReactionType::Unicode("🪄".to_string()))
            .label("Encantamientos")
            .style(ButtonStyle::Secondary),
        CreateButton::new("GearResearch")
            .emoji(ReactionType::Unicode("🔬".to_string()))
            .label("Investigar Rasgos")
            .style(ButtonStyle::Primary)
    ])
}

fn get_enum_as_menu<T: IntoEnumIterator + EnumMessage + Display>(name: &str, placeholder: &str) -> CreateSelectMenu {
    let options = CreateSelectMenuKind::String {
        options: T::iter()
            .map(|e| CreateSelectMenuOption::new(e.to_string(), e.to_string())
                .description(e.get_documentation().unwrap_or("")))
            .collect()
    };

    CreateSelectMenu::new(name, options)
        .placeholder(placeholder)
}