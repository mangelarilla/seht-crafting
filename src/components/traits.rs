use serenity::builder::CreateSelectMenu;
use strum::{EnumMessage, IntoEnumIterator};
use crate::entities::armour::ArmourTraits;
use crate::entities::jewelry::JewelryTraits;
use crate::entities::weapon::WeaponTraits;

pub fn gear_armour_traits(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el rasgo que quieres");
    b.options(|opts| {
        for armour_trait in ArmourTraits::iter() {
            opts.create_option(|o| o
                .label(armour_trait.to_string())
                .value(armour_trait.to_string())
                .description(armour_trait.get_documentation().unwrap()));
        }
        opts
    });
    b
}

pub fn gear_jewelry_traits(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el rasgo que quieres");
    b.options(|opts| {
        for jewelry_trait in JewelryTraits::iter() {
            opts.create_option(|o| o
                .label(jewelry_trait.to_string())
                .value(jewelry_trait.to_string())
                .description(jewelry_trait.get_documentation().unwrap()));
        }
        opts
    });
    b
}

pub fn gear_weapon_traits(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el rasgo que quieres");
    b.options(|opts| {
        for weapon_trait in WeaponTraits::iter() {
            opts.create_option(|o| o
                .label(weapon_trait.to_string())
                .value(weapon_trait.to_string())
                .description(weapon_trait.get_documentation().unwrap()));
        }
        opts
    });
    b
}