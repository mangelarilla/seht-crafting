pub mod traits;

use serenity::builder::{CreateActionRow, CreateEmbed, CreateInputText, CreateSelectMenu};
use serenity::model::prelude::component::*;
use serenity::model::prelude::*;

const ARMOR_PARTS: [&str; 7] = ["Cabeza", "Hombros", "Pecho", "Manos", "Cintura", "Piernas", "Pies"];
const ONE_HANDED: [&str; 4] = ["Maza", "Daga", "Hachuela", "Espada"];

#[derive(Debug)]
pub struct SetPiece {
    pub part: String,
    pub part_trait: String,
    pub weight: Option<String>,
    pub quality: String
}

pub fn is_armor(name: &str) -> bool {
    ARMOR_PARTS.contains(&name)
}

pub fn is_one_handed(name: &str) -> bool {
    ONE_HANDED.contains(&name)
}

pub fn gear_set_parts(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona las partes del set que quieres");
    b.max_values(12);
    b.options(|opts| opts
        .create_option(|o| o.label("Cabeza").value("Cabeza"))
        .create_option(|o| o.label("Hombros").value("Hombros"))
        .create_option(|o| o.label("Pecho").value("Pecho"))
        .create_option(|o| o.label("Manos").value("Manos"))
        .create_option(|o| o.label("Cintura").value("Cintura"))
        .create_option(|o| o.label("Piernas").value("Piernas"))
        .create_option(|o| o.label("Pies").value("Pies"))
        .create_option(|o| o.label("Amuleto").value("Amuleto"))
        .create_option(|o| o.label("Anillo #1").value("Anillo #1"))
        .create_option(|o| o.label("Anillo #2").value("Anillo #2"))
        .create_option(|o| o.label("Escudo").value("Escudo"))
        .create_option(|o| o.label("Baston fuego").value("Baston fuego"))
        .create_option(|o| o.label("Baston electrico").value("Baston electrico"))
        .create_option(|o| o.label("Baston hielo").value("Baston hielo"))
        .create_option(|o| o.label("Arco").value("Arco"))
        .create_option(|o| o.label("Mandoble").value("Mandoble").description("Dos manos"))
        .create_option(|o| o.label("Hacha").value("Hacha").description("Dos manos"))
        .create_option(|o| o.label("Mazo").value("Mazo").description("Dos manos"))
        .create_option(|o| o.label("Maza").value("Maza").description("Una mano"))
        .create_option(|o| o.label("Daga").value("Daga").description("Una mano"))
        .create_option(|o| o.label("Espada").value("Espada").description("Una mano"))
        .create_option(|o| o.label("Hachuela").value("Hachuela").description("Una mano"))
    );
    b
}

pub fn armor_weight(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el peso de la armadura");
    b.options(|opts| opts
        .create_option(|o| o.label("Pesada").value("Pesada"))
        .create_option(|o| o.label("Media").value("Media"))
        .create_option(|o| o.label("Ligera").value("Ligera"))
    );
    b
}



pub fn gear_quality(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona la calidad de la pieza");
    b.options(|opts| opts
        .create_option(|o| o.label("Verde").value("Verde").emoji(ReactionType::Unicode("🟢".to_string())))
        .create_option(|o| o.label("Azul").value("Azul").emoji(ReactionType::Unicode("🔵".to_string())))
        .create_option(|o| o.label("Morada").value("Morada").emoji(ReactionType::Unicode("🟣".to_string())))
        .create_option(|o| o.label("Amarilla").value("Amarilla").emoji(ReactionType::Unicode("🟡".to_string())))
    );
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

Se deberan enviar los materiales al fabricante que se encargue y abonarle el pago de __**{}**__ de oro.",
        Mention::Role(crafters.id), price)
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
        .label("Consumables")
        .style(ButtonStyle::Success));
    b.create_button(|b| b
        .custom_id("Enchantment")
        .emoji(ReactionType::Unicode("🪄".to_string()))
        .label("Encantamientos")
        .style(ButtonStyle::Secondary));
    b
}