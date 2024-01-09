use serenity::all::{CreateSelectMenu};
use crate::entities::armour::{ArmourEnchantments};
use crate::entities::jewelry::{JewelryEnchantments};
use crate::entities::weapon::{WeaponEnchantments};

pub fn gear_armour_enchantments(name: &str) -> CreateSelectMenu {
    super::get_enum_as_menu::<ArmourEnchantments>(name, "Selecciona el encantamiento que quieres")
}

pub fn gear_jewelry_enchantments(name: &str) -> CreateSelectMenu {
    super::get_enum_as_menu::<JewelryEnchantments>(name, "Selecciona el encantamiento que quieres")
}

pub fn gear_weapon_enchantments(name: &str) -> CreateSelectMenu {
    super::get_enum_as_menu::<WeaponEnchantments>(name, "Selecciona el encantamiento que quieres")
}