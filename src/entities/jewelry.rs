use strum_macros::{Display, EnumIter, EnumMessage, EnumString};
use std::string::ToString;
use crate::entities::{GearQuality, get_enchantment_quality_cost, MaterialCost};
use crate::entities::materials::{EssenceRunes, JewelryQualityMaterials, JewelryTraitMaterials, PartMaterials, PotencyRunes};

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, EnumString, Display, EnumMessage)]
pub enum Jewelries {
    /// Solo uno
    #[strum(serialize = "Collar")]
    Necklace,
    /// Se asumen dos anillos
    #[strum(serialize = "Anillo")]
    Ring
}

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, EnumString, Display, EnumMessage)]
pub enum JewelryTraits {
    /// Aumenta la magia máxima
    #[strum(serialize = "Arcanidad")]
    Arcane,
    /// Aumenta el daño en enemigos por debajo del 90%
    #[strum(serialize = "Sed de sangre")]
    Bloodthirsty,
    /// Al activar una sinergia, restaura salud, magia y aguante
    #[strum(serialize = "Armonía")]
    Harmony,
    /// Aumenta la salud máxima
    #[strum(serialize = "Saludable")]
    Healthy,
    /// Aumenta la eficacia del encantamiento de la joyería
    #[strum(serialize = "Imbuido")]
    Infused,
    /// Aumenta la resistencia a hechizos y física
    #[strum(serialize = "Protección")]
    Protective,
    /// Aumenta el aguante máximo
    #[strum(serialize = "Robustez")]
    Robust,
    /// Aumenta la velocidad de movimiento
    #[strum(serialize = "Agilidad")]
    Swift,
    /// Aumenta la magia, aguante y salud máximas
    #[strum(serialize = "Trinidad")]
    Triune
}

#[derive(Clone, EnumIter, Ord, PartialOrd, Eq, PartialEq, EnumString, Display, EnumMessage)]
pub enum JewelryEnchantments {
    /// Añade daño de arma y hechizo, y recuperación de aguante
    #[strum(serialize = "Glifo de aumento de daño físico")]
    IncreasePhysicalHarm,
    /// Añade daño de arma y hechizo, y recuperación de magia
    #[strum(serialize = "Glifo de aumento de daño mágico")]
    IncreaseMagicalHarm,
    /// Añade recuperación de salud
    #[strum(serialize = "Glifo de regeneración de salud")]
    HealthRecovery,
    /// Añade recuperación de magia
    #[strum(serialize = "Glifo de regeneración de magia")]
    MagickaRecovery,
    /// Añade recuperación de aguante
    #[strum(serialize = "Glifo de regeneración de aguante")]
    StaminaRecovery,
    /// Reduce el coste de magia de las habilidades
    #[strum(serialize = "Glifo de reducción de coste de magia")]
    ReduceSpellCost,
    /// Reduce el coste de aguante de las habilidades
    #[strum(serialize = "Glifo de reducción de coste de aguante")]
    ReduceFeatCost,
    /// Reduce el coste de bloquear
    #[strum(serialize = "Glifo de bloqueo")]
    Shielding,
    /// Añade daño a tus ataques de aporreo
    #[strum(serialize = "Glifo de percusión")]
    Bashing,
    /// Añade resistencia física
    #[strum(serialize = "Glifo de resistencia al daño físico")]
    DecreasePhysicalHarm,
    /// Añade resistencia a los hechizos
    #[strum(serialize = "Glifo de resistencia al daño mágico")]
    DecreaseSpellHarm,
    /// Añade resistencia a las llamas
    #[strum(serialize = "Glifo de resistencia al fuego")]
    FlameResist,
    /// Añade resistencia a la escarcha
    #[strum(serialize = "Glifo de resistencia a la congelación")]
    FrostResist,
    /// Añade resistencia a descargas eléctricas
    #[strum(serialize = "Glifo de resistencia a las descargas")]
    ShockResist,
    /// Añade resistencia a venenos
    #[strum(serialize = "Glifo de resistencia al veneno")]
    PoisonResist,
    /// Añade resistencia a enfermedades
    #[strum(serialize = "Glifo de resistencia a las enfermedades")]
    DiseaseResist,
    /// Aumenta la duración de los efectos de las pociones
    #[strum(serialize = "Glifo de amplificación alquímica")]
    PotionResist,
    /// Reduce la reutilización de las pociones
    #[strum(serialize = "Glifo de aceleración alquímica")]
    PotionBoost,
    /// Reduce el coste de salud, magia y aguante de las habilidades
    #[strum(serialize = "Glifo de reducción de coste de habilidades")]
    ReduceSkillCost,
    /// Añade recuperación de magia, salud y aguante
    #[strum(serialize = "Glifo de regeneración prismática")]
    PrismaticRecovery
}

pub struct Jewelry {
    pub kind: Jewelries,
    pub jewelry_trait: JewelryTraits,
    pub enchantment: Option<JewelryEnchantments>,
    pub quality: GearQuality
}

impl MaterialCost for Jewelries {
    fn cost(&self) -> Vec<(i32, String)> {
        match *self {
            Jewelries::Necklace => vec![(150, PartMaterials::PlatinumOunces.to_string())],
            Jewelries::Ring => vec![(100, PartMaterials::PlatinumOunces.to_string())]
        }
    }
}

impl MaterialCost for JewelryTraits {
    fn cost(&self) -> Vec<(i32, String)> {
        match *self {
            JewelryTraits::Arcane => vec![(1, JewelryTraitMaterials::Cobalt.to_string())],
            JewelryTraits::Bloodthirsty => vec![(1, JewelryTraitMaterials::Slaughterstone.to_string())],
            JewelryTraits::Harmony => vec![(1, JewelryTraitMaterials::Dibellium.to_string())],
            JewelryTraits::Healthy => vec![(1, JewelryTraitMaterials::Antimony.to_string())],
            JewelryTraits::Infused => vec![(1, JewelryTraitMaterials::AurbicAmber.to_string())],
            JewelryTraits::Protective => vec![(1, JewelryTraitMaterials::Titanium.to_string())],
            JewelryTraits::Robust => vec![(1, JewelryTraitMaterials::Zinc.to_string())],
            JewelryTraits::Swift => vec![(1, JewelryTraitMaterials::GildingWax.to_string())],
            JewelryTraits::Triune => vec![(1, JewelryTraitMaterials::DawnPrism.to_string())],
        }
    }
}

impl MaterialCost for JewelryEnchantments {
    fn cost(&self) -> Vec<(i32, String)> {
        match *self {
            JewelryEnchantments::IncreasePhysicalHarm => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Taderi.to_string())],
            JewelryEnchantments::IncreaseMagicalHarm => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Makderi.to_string())],
            JewelryEnchantments::HealthRecovery => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Okoma.to_string())],
            JewelryEnchantments::MagickaRecovery => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Makkoma.to_string())],
            JewelryEnchantments::StaminaRecovery => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Denima.to_string())],
            JewelryEnchantments::ReduceSpellCost => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Makkoma.to_string())],
            JewelryEnchantments::ReduceFeatCost => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Denima.to_string())],
            JewelryEnchantments::Shielding => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Kaderi.to_string())],
            JewelryEnchantments::Bashing => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Kaderi.to_string())],
            JewelryEnchantments::DecreasePhysicalHarm => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Taderi.to_string())],
            JewelryEnchantments::DecreaseSpellHarm => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Makderi.to_string())],
            JewelryEnchantments::FlameResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Rakeipa.to_string())],
            JewelryEnchantments::FrostResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Dekeipa.to_string())],
            JewelryEnchantments::ShockResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Meip.to_string())],
            JewelryEnchantments::PoisonResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Kuoko.to_string())],
            JewelryEnchantments::DiseaseResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Haoko.to_string())],
            JewelryEnchantments::PotionResist => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Oru.to_string())],
            JewelryEnchantments::PotionBoost => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Oru.to_string())],
            JewelryEnchantments::ReduceSkillCost => vec![(1, PotencyRunes::Itade.to_string()), (1, EssenceRunes::Indeko.to_string())],
            JewelryEnchantments::PrismaticRecovery => vec![(1, PotencyRunes::Repora.to_string()), (1, EssenceRunes::Indeko.to_string())],
        }
    }
}

impl MaterialCost for Jewelry {
    fn cost(&self) -> Vec<(i32, String)> {
        let mut vec = Vec::new();
        vec.append(&mut self.kind.cost());
        vec.append(&mut self.jewelry_trait.cost());
        if let Some(e) = &self.enchantment {
            vec.append(&mut e.cost());
        }
        vec.append(&mut get_enchantment_quality_cost(&self.quality));
        vec.append(&mut match self.quality {
            GearQuality::White => vec![],
            GearQuality::Green => vec![(1, JewelryQualityMaterials::TernePlating.to_string())],
            GearQuality::Blue => vec![(2, JewelryQualityMaterials::IridiumPlating.to_string())],
            GearQuality::Purple => vec![(3, JewelryQualityMaterials::ZirconPlating.to_string())],
            GearQuality::Yellow => vec![(4, JewelryQualityMaterials::ChromiumPlating.to_string())],
        });
        vec
    }
}