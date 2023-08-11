use std::fmt::{Display, Formatter};
use serenity::model::prelude::ReactionType;
use strum::EnumIter;
use crate::entities::armour::ArmourParts;
use crate::entities::jewelry::Jewelries;
use crate::entities::weapon::WeaponKind;

pub mod armour;
pub mod weapon;
pub mod jewelry;

#[derive(Clone)]
pub enum Gear {
    Weapon(WeaponKind),
    Armour(ArmourParts),
    Jewelry(Jewelries)
}

#[derive(EnumIter)]
pub enum GearQuality {
    Green, Blue, Purple, Yellow
}

impl Display for GearQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            GearQuality::Green => write!(f, "Verde"),
            GearQuality::Blue => write!(f, "Azul"),
            GearQuality::Purple => write!(f, "Morada"),
            GearQuality::Yellow => write!(f, "Amarilla"),
        }
    }
}

impl ItemEmoji for GearQuality {
    fn emoji(&self) -> ReactionType {
        match *self {
            GearQuality::Green => ReactionType::Unicode("🟢".to_string()),
            GearQuality::Blue => ReactionType::Unicode("🔵".to_string()),
            GearQuality::Purple => ReactionType::Unicode("🟣".to_string()),
            GearQuality::Yellow => ReactionType::Unicode("🟡".to_string())
        }
    }
}

impl Display for Gear {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Gear::Weapon(w) => write!(f, "{}", w),
            Gear::Armour(a) => write!(f, "{}", a),
            Gear::Jewelry(j) => write!(f, "{}", j),
        }
    }
}

impl TryFrom<String> for Gear {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let armour = ArmourParts::try_from(value.to_string()).map(|a| Self::Armour(a));
        if armour.is_err() {
            let weapon = WeaponKind::try_from(value.to_string()).map(|w| Self::Weapon(w));
            if weapon.is_err() {
                Jewelries::try_from(value).map(|j| Self::Jewelry(j))
            } else { weapon }
        } else { armour }
    }
}

pub trait ItemInfo {
    fn description(&self) -> String;
}

pub trait ItemEmoji {
    fn emoji(&self) -> ReactionType;
}