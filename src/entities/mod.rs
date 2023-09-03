use std::fmt::{Display, Formatter};
use strum_macros::{Display, EnumIter, EnumProperty, EnumString};
use crate::entities::armour::ArmourParts;
use crate::entities::jewelry::Jewelries;
use crate::entities::materials::{BlacksmithQualityMaterials, RuneQualityMaterials, TailoringQualityMaterials, WoodworkingQualityMaterials};
use crate::entities::weapon::{WeaponKind};

pub mod armour;
pub mod weapon;
pub mod jewelry;
pub mod materials;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Gear {
    Weapon(WeaponKind),
    Armour(ArmourParts),
    Jewelry(Jewelries)
}

#[derive(EnumIter, Clone, Ord, PartialOrd, Eq, PartialEq, EnumString, Display, EnumProperty)]
pub enum GearQuality {
    #[strum(serialize = "Blanco")]
    #[strum(props(Emoji = "⚪"))]
    White,
    #[strum(serialize = "Verde")]
    #[strum(props(Emoji = "🟢"))]
    Green,
    #[strum(serialize = "Azul")]
    #[strum(props(Emoji = "🔵"))]
    Blue,
    #[strum(serialize = "Morada")]
    #[strum(props(Emoji = "🟣"))]
    Purple,
    #[strum(serialize = "Amarilla")]
    #[strum(props(Emoji = "🟡"))]
    Yellow
}

pub trait MaterialCost {
    fn cost(&self) -> Vec<(i32, String)>;
}

fn get_enchantment_quality_cost(quality: &GearQuality) -> Vec<(i32, String)> {
    match quality {
        GearQuality::White => vec![(1, RuneQualityMaterials::Ta.to_string())],
        GearQuality::Green => vec![(1, RuneQualityMaterials::Jejota.to_string())],
        GearQuality::Blue => vec![(1, RuneQualityMaterials::Denata.to_string())],
        GearQuality::Purple => vec![(1, RuneQualityMaterials::Rekuta.to_string())],
        GearQuality::Yellow => vec![(1, RuneQualityMaterials::Kuta.to_string())],
    }
}

fn get_blacksmith_quality_cost(quality: &GearQuality) -> Vec<(i32, String)> {
    match quality {
        GearQuality::White => vec![],
        GearQuality::Green => vec![
            (2, BlacksmithQualityMaterials::HoningStone.to_string()),
        ],
        GearQuality::Blue => vec![
            (2, BlacksmithQualityMaterials::HoningStone.to_string()),
            (3, BlacksmithQualityMaterials::DwarvenOil.to_string()),
        ],
        GearQuality::Purple => vec![
            (2, BlacksmithQualityMaterials::HoningStone.to_string()),
            (3, BlacksmithQualityMaterials::DwarvenOil.to_string()),
            (4, BlacksmithQualityMaterials::GrainSolvent.to_string()),
        ],
        GearQuality::Yellow => vec![
            (2, BlacksmithQualityMaterials::HoningStone.to_string()),
            (3, BlacksmithQualityMaterials::DwarvenOil.to_string()),
            (4, BlacksmithQualityMaterials::GrainSolvent.to_string()),
            (8, BlacksmithQualityMaterials::TemperingAlloy.to_string())
        ],
    }
}

fn get_tailoring_quality_cost(quality: &GearQuality) -> Vec<(i32, String)> {
    match quality {
        GearQuality::White => vec![],
        GearQuality::Green => vec![
            (2, TailoringQualityMaterials::Hemming.to_string()),
        ],
        GearQuality::Blue => vec![
            (2, TailoringQualityMaterials::Hemming.to_string()),
            (3, TailoringQualityMaterials::Embroidery.to_string()),
        ],
        GearQuality::Purple => vec![
            (2, TailoringQualityMaterials::Hemming.to_string()),
            (3, TailoringQualityMaterials::Embroidery.to_string()),
            (4, TailoringQualityMaterials::ElegantLining.to_string()),
        ],
        GearQuality::Yellow => vec![
            (2, TailoringQualityMaterials::Hemming.to_string()),
            (3, TailoringQualityMaterials::Embroidery.to_string()),
            (4, TailoringQualityMaterials::ElegantLining.to_string()),
            (8, TailoringQualityMaterials::DreughWax.to_string())
        ]
    }
}

fn get_woodworking_quality_cost(quality: &GearQuality) -> Vec<(i32, String)> {
    match quality {
        GearQuality::White => vec![],
        GearQuality::Green => vec![
            (2, WoodworkingQualityMaterials::Pitch.to_string()),
        ],
        GearQuality::Blue => vec![
            (2, WoodworkingQualityMaterials::Pitch.to_string()),
            (3, WoodworkingQualityMaterials::Turpen.to_string()),
        ],
        GearQuality::Purple => vec![
            (2, WoodworkingQualityMaterials::Pitch.to_string()),
            (3, WoodworkingQualityMaterials::Turpen.to_string()),
            (4, WoodworkingQualityMaterials::Mastic.to_string()),
        ],
        GearQuality::Yellow => vec![
            (2, WoodworkingQualityMaterials::Pitch.to_string()),
            (3, WoodworkingQualityMaterials::Turpen.to_string()),
            (4, WoodworkingQualityMaterials::Mastic.to_string()),
            (8, WoodworkingQualityMaterials::Rosin.to_string())
        ],
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

impl std::str::FromStr for Gear {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(a) = ArmourParts::from_str(s) {
            Ok(Gear::Armour(a))
        } else if let Ok(j) = Jewelries::from_str(s) {
            Ok(Gear::Jewelry(j))
        } else if let Ok(w) = WeaponKind::from_str(s) {
            Ok(Gear::Weapon(w))
        } else {
            Err(strum::ParseError::VariantNotFound)
        }
    }
}