use serenity::builder::CreateSelectMenu;
use strum::IntoEnumIterator;
use crate::entities::armour::ArmourTraits;
use crate::entities::{Gear, ItemInfo};
use crate::entities::jewelry::JewelryTraits;
use crate::entities::weapon::WeaponTraits;

pub fn get_trait(part: &Gear, name: &str) -> CreateSelectMenu {
    if let Gear::Armour(_) = part {
        gear_armour_traits(name)
    } else if let Gear::Jewelry(_) = part {
        gear_jewelry_traits(name)
    } else {
        gear_weapon_traits(name)
    }
}

fn gear_armour_traits(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el rasgo que quieres");
    b.options(|opts| {
        for armour_trait in ArmourTraits::iter() {
            opts.create_option(|o| o
                .label(armour_trait.to_string())
                .value(armour_trait.to_string())
                .description(armour_trait.description()));
        }
        opts
    });
    b
}

fn gear_jewelry_traits(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el rasgo que quieres");
    b.options(|opts| {
        for jewelry_trait in JewelryTraits::iter() {
            opts.create_option(|o| o
                .label(jewelry_trait.to_string())
                .value(jewelry_trait.to_string())
                .description(jewelry_trait.description()));
        }
        opts
    });
    b
}

fn gear_weapon_traits(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el rasgo que quieres");
    b.options(|opts| {
        for weapon_trait in WeaponTraits::iter() {
            opts.create_option(|o| o
                .label(weapon_trait.to_string())
                .value(weapon_trait.to_string())
                .description(weapon_trait.description()));
        }
        opts
    });
    b
}