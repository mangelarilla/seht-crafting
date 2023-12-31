use serenity::builder::CreateSelectMenu;
use strum::{EnumMessage, IntoEnumIterator};
use crate::entities::armour::{ArmourEnchantments};
use crate::entities::jewelry::{JewelryEnchantments};
use crate::entities::weapon::{WeaponEnchantments};

pub fn gear_armour_enchantments(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el encantamiento que quieres");
    b.options(|opts| {
        for armour_enchantment in ArmourEnchantments::iter() {
            opts.create_option(|o| o
                .label(armour_enchantment.to_string())
                .value(armour_enchantment.to_string())
                .description(armour_enchantment.get_documentation().unwrap()));
        }
        opts
    });
    b
}

pub fn gear_jewelry_enchantments(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el encantamiento que quieres");
    b.options(|opts| {
        for jewelry_enchantment in JewelryEnchantments::iter() {
            opts.create_option(|o| o
                .label(jewelry_enchantment.to_string())
                .value(jewelry_enchantment.to_string())
                .description(jewelry_enchantment.get_documentation().unwrap()));
        }
        opts
    });
    b
}

pub fn gear_weapon_enchantments(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el encantamiento que quieres");
    b.options(|opts| {
        for weapon_enchantment in WeaponEnchantments::iter() {
            opts.create_option(|o| o
                .label(weapon_enchantment.to_string())
                .value(weapon_enchantment.to_string())
                .description(weapon_enchantment.get_documentation().unwrap()));
        }
        opts
    });
    b
}