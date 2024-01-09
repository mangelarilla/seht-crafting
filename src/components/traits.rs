use serenity::builder::CreateSelectMenu;
use crate::entities::armour::ArmourTraits;
use crate::entities::jewelry::JewelryTraits;
use crate::entities::weapon::WeaponTraits;

pub fn gear_armour_traits(name: &str) -> CreateSelectMenu {
    super::get_enum_as_menu::<ArmourTraits>(name, "Selecciona el rasgo que quieres")
}

pub fn gear_jewelry_traits(name: &str) -> CreateSelectMenu {
    super::get_enum_as_menu::<JewelryTraits>(name, "Selecciona el rasgo que quieres")
}

pub fn gear_weapon_traits(name: &str) -> CreateSelectMenu {
    super::get_enum_as_menu::<WeaponTraits>(name, "Selecciona el rasgo que quieres")
}