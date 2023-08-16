use strum_macros::{Display, EnumIter, EnumMessage, EnumString};
use std::string::ToString;
use crate::entities::{GearQuality, get_enchantment_quality_cost, MaterialCost};
use crate::entities::materials::{ArmourTraitMaterials, BlacksmithQualityMaterials, EssenceRunes, PartMaterials, PotencyRunes, TailoringQualityMaterials};

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString)]
pub enum ArmourParts {
    #[strum(serialize = "Cabeza")]
    Head,
    #[strum(serialize = "Hombros")]
    Shoulder,
    #[strum(serialize = "Cuerpo")]
    Body,
    #[strum(serialize = "Manos")]
    Hands,
    #[strum(serialize = "Cintura")]
    Waist,
    #[strum(serialize = "Piernas")]
    Legs,
    #[strum(serialize = "Pies")]
    Feet,
    #[strum(serialize = "Escudo")]
    Shield
}

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString)]
pub enum ArmourWeights {
    #[strum(serialize = "Ligera")]
    Light,
    #[strum(serialize = "Media")]
    Medium,
    #[strum(serialize = "Pesada")]
    Heavy
}

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, EnumMessage)]
pub enum ArmourTraits {
    /// Aumenta los efectos de las piedras de Mundus
    #[strum(serialize = "Divinidad")]
    Divines,
    /// Aumenta la recuperación de salud, magia y aguante
    #[strum(serialize = "Vigorización")]
    Invigorating,
    /// Aumenta la resistencia a críticos y la durabilidad
    #[strum(serialize = "Impenetrabilidad")]
    Impenetrable,
    /// Aumenta el efecto del encantamiento de la armadura
    #[strum(serialize = "Imbuición")]
    Infused,
    /// Aumenta la resistencia física y a hechizos
    #[strum(serialize = "Temple de Nirn")]
    Nirnhoned,
    /// Aumenta el valor de armadura de este objeto
    #[strum(serialize = "Refuerzo")]
    Reinforced,
    /// Reduce el coste de bloquear
    #[strum(serialize = "Solidez")]
    Sturdy,
    /// Aumenta la experiencia ganada con cada muerte
    #[strum(serialize = "Entrenamiento")]
    Training,
    /// Reduce el coste de esquivar rodando y esprintar
    #[strum(serialize = "Buen ajuste")]
    WellFitted
}

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, EnumMessage)]
pub enum ArmourEnchantments {
    /// Aumenta la salud máxima
    #[strum(serialize = "Glifo de salud")]
    Health,
    /// Aumenta la magia máxima
    #[strum(serialize = "Glifo de magia")]
    Magicka,
    /// Aumenta el aguante máximo
    #[strum(serialize = "Glifo de aguante")]
    Stamina,
    /// Aumente la magia, salud y aguante máximos
    #[strum(serialize = "Glifo de defensa prismática")]
    PrismaticDefense
}

pub struct Armour {
    pub kind: ArmourParts,
    pub weight: ArmourWeights,
    pub armour_trait: ArmourTraits,
    pub enchantment: Option<ArmourEnchantments>,
    pub quality: GearQuality
}

impl MaterialCost for ArmourTraits {
    fn cost(&self) -> Vec<(i32, String)> {
        match *self {
            ArmourTraits::Divines => vec![(1, ArmourTraitMaterials::Sapphire.to_string())],
            ArmourTraits::Invigorating => vec![(1, ArmourTraitMaterials::Garnet.to_string())],
            ArmourTraits::Impenetrable => vec![(1, ArmourTraitMaterials::Diamond.to_string())],
            ArmourTraits::Infused => vec![(1, ArmourTraitMaterials::Bloodstone.to_string())],
            ArmourTraits::Nirnhoned => vec![(1, ArmourTraitMaterials::FortifiedNirncrux.to_string())],
            ArmourTraits::Reinforced => vec![(1, ArmourTraitMaterials::Sardonyx.to_string())],
            ArmourTraits::Sturdy => vec![(1, ArmourTraitMaterials::Quartz.to_string())],
            ArmourTraits::Training => vec![(1, ArmourTraitMaterials::Emerald.to_string())],
            ArmourTraits::WellFitted => vec![(1, ArmourTraitMaterials::Almandine.to_string())],
        }
    }
}

impl MaterialCost for ArmourEnchantments {
    fn cost(&self) -> Vec<(i32, String)> {
        match *self {
            ArmourEnchantments::Health => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Oko.to_string())],
            ArmourEnchantments::Magicka => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Makko.to_string())],
            ArmourEnchantments::Stamina => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Deni.to_string())],
            ArmourEnchantments::PrismaticDefense => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Hakeijo.to_string())],
        }
    }
}

fn get_quality_mats(weight: &ArmourWeights, quality: &GearQuality) -> Vec<(i32, String)> {
    match quality {
        GearQuality::White => vec![],
        GearQuality::Green => match weight {
            ArmourWeights::Heavy => vec![(2, BlacksmithQualityMaterials::HoningStone.to_string())],
            _ => vec![(2, TailoringQualityMaterials::Hemming.to_string())]
        }
        GearQuality::Blue => match weight {
            ArmourWeights::Heavy => vec![(3, BlacksmithQualityMaterials::DwarvenOil.to_string())],
            _ => vec![(3, TailoringQualityMaterials::Embroidery.to_string())]
        }
        GearQuality::Purple => match weight {
            ArmourWeights::Heavy => vec![(4, BlacksmithQualityMaterials::GrainSolvent.to_string())],
            _ => vec![(4, TailoringQualityMaterials::ElegantLining.to_string())]
        }
        GearQuality::Yellow => match weight {
            ArmourWeights::Heavy => vec![(8, BlacksmithQualityMaterials::TemperingAlloy.to_string())],
            _ => vec![(8, TailoringQualityMaterials::DreughWax.to_string())]
        }
    }
}

fn get_part_mats(part: &ArmourParts, weight: &ArmourWeights) -> Vec<(i32, String)> {
    match part {
        ArmourParts::Body => match weight {
            ArmourWeights::Heavy => vec![(150, PartMaterials::RubediteIngots.to_string())],
            ArmourWeights::Light => vec![(150, PartMaterials::AncestorSilk.to_string())],
            ArmourWeights::Medium => vec![(150, PartMaterials::RubedoLeather.to_string())],
        }
        ArmourParts::Legs => match weight {
            ArmourWeights::Heavy => vec![(140, PartMaterials::RubediteIngots.to_string())],
            ArmourWeights::Light => vec![(140, PartMaterials::AncestorSilk.to_string())],
            ArmourWeights::Medium => vec![(140, PartMaterials::RubedoLeather.to_string())],
        }
        ArmourParts::Shield => vec![(140, PartMaterials::SandedRubyAsh.to_string())],
        _ => match weight {
            ArmourWeights::Heavy => vec![(130, PartMaterials::RubediteIngots.to_string())],
            ArmourWeights::Light => vec![(130, PartMaterials::AncestorSilk.to_string())],
            ArmourWeights::Medium => vec![(130, PartMaterials::RubedoLeather.to_string())],
        }
    }
}

impl MaterialCost for Armour {
    fn cost(&self) -> Vec<(i32, String)> {
        let mut vec = Vec::new();
        vec.append(&mut get_part_mats(&self.kind, &self.weight));
        vec.append(&mut self.armour_trait.cost());
        if let Some(e) = &self.enchantment {
            vec.append(&mut e.cost());
        }
        vec.append(&mut get_enchantment_quality_cost(&self.quality));
        vec.append(&mut get_quality_mats(&self.weight, &self.quality));
        vec
    }
}