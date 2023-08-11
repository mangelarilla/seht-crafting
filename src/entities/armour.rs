use std::fmt::{Display, Formatter, write};
use crate::entities::ItemInfo;

pub enum ArmourParts {
    Head, Shoulder, Body, Hands, Waist, Legs, Feet, Shield
}

pub enum ArmourWeights {
    Light, Medium, Heavy
}

pub enum ArmourTraits {
    Divines, Invigorating, Impenetrable, Infused, Nirnhoned, Reinforced, Sturdy, Training, WellFitted
}

pub enum ArmourEnchantments {
    Health, Magicka, Stamina, PrismaticDefense
}

pub struct Armour {
    pub kind: ArmourParts,
    pub weight: ArmourWeights,
    pub armour_trait: ArmourTraits,
    pub enchantment: ArmourEnchantments
}

impl Display for ArmourParts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ArmourParts::Head => write!(f, "Cabeza"),
            ArmourParts::Shoulder => write!(f, "Hombros"),
            ArmourParts::Body => write!(f, "Cuerpo"),
            ArmourParts::Hands => write!(f, "Manos"),
            ArmourParts::Waist =>write!(f, "Cintura"),
            ArmourParts::Legs => write!(f, "Piernas"),
            ArmourParts::Feet => write!(f, "Pies"),
            ArmourParts::Shield => write!(f, "Escudo")
        }
    }
}

impl Display for ArmourWeights {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ArmourWeights::Light => write!(f, "Ligera"),
            ArmourWeights::Medium => write!(f, "Media"),
            ArmourWeights::Heavy => write!(f, "Pesada"),
        }
    }
}

impl Display for ArmourTraits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ArmourTraits::Divines => write!(f, "Divinidad"),
            ArmourTraits::Invigorating => write!(f, "Vigorización"),
            ArmourTraits::Impenetrable => write!(f, "Impenetrabilidad"),
            ArmourTraits::Infused => write!(f, "Imbuición"),
            ArmourTraits::Nirnhoned => write!(f, "Temple de Nirn"),
            ArmourTraits::Reinforced => write!(f, "Refuerzo"),
            ArmourTraits::Sturdy => write!(f, "Solidez"),
            ArmourTraits::Training => write!(f, "Entrenamiento"),
            ArmourTraits::WellFitted => write!(f, "Buen ajuste"),
        }
    }
}

impl ItemInfo for ArmourTraits {
    fn description(&self) -> String {
        match *self {
            ArmourTraits::Divines => "Aumenta los efectos de las piedras de Mundus".to_string(),
            ArmourTraits::Invigorating => "Aumenta la recuperación de salud, magia y aguante".to_string(),
            ArmourTraits::Impenetrable => "Aumenta la resistencia a críticos y el objeto recibe un 50% menos de reducción de durabilidad".to_string(),
            ArmourTraits::Infused => "Aumenta el efecto del encantamiento de la armadura".to_string(),
            ArmourTraits::Nirnhoned => "Aumenta la resistencia física y a hechizos".to_string(),
            ArmourTraits::Reinforced => "Aumenta el valor de armadura de este objeto".to_string(),
            ArmourTraits::Sturdy => "Reduce el coste de bloquear".to_string(),
            ArmourTraits::Training => "Aumenta la experiencia ganada con cada muerte".to_string(),
            ArmourTraits::WellFitted => "Reduce el coste de esquivar rodando y esprintar".to_string(),
        }
    }
}

impl Display for ArmourEnchantments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ArmourEnchantments::Health => write!(f, "Glifo de salud"),
            ArmourEnchantments::Magicka => write!(f, "Glifo de magia"),
            ArmourEnchantments::Stamina => write!(f, "Glifo de aguante"),
            ArmourEnchantments::PrismaticDefense => write!(f, "Glifo de defensa prismática"),
        }
    }
}

impl ItemInfo for ArmourEnchantments {
    fn description(&self) -> String {
        match *self {
            ArmourEnchantments::Health => "Aumenta la salud máxima".to_string(),
            ArmourEnchantments::Magicka => "Aumenta la magia máxima".to_string(),
            ArmourEnchantments::Stamina => "Aumenta el aguante máximo".to_string(),
            ArmourEnchantments::PrismaticDefense => "Aumente la magia, salud y aguante máximos".to_string(),
        }
    }
}