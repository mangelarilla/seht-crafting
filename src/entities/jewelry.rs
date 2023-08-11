use std::fmt::{Display, Formatter};
use strum::EnumIter;
use crate::entities::ItemInfo;

#[derive(Clone, EnumIter)]
pub enum Jewelries {
    Necklace, Ring
}

#[derive(EnumIter)]
pub enum JewelryTraits {
    Arcane, Bloodthirsty, Harmony, Healthy, Infused, Protective, Robust, Swift, Triune
}

#[derive(EnumIter)]
pub enum JewelryEnchantments {
    IncreasePhysicalHarm, IncreaseMagicalHarm, HealthRecovery, MagickaRecovery, StaminaRecovery,
    ReduceSpellCost, ReduceFeatCost, Shielding, Bashing, DecreasePhysicalHarm, DecreaseSpellHarm,
    FlameResist, FrostResist, ShockResist, PoisonResist, DiseaseResist, PotionResist, PotionBoost,
    ReduceSkillCost, PrismaticRecovery
}

pub struct Jewelry {
    pub kind: Jewelries,
    pub jewelry_trait: JewelryTraits,
    pub enchantment: JewelryEnchantments
}

impl TryFrom<String> for Jewelries {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == Jewelries::Necklace.to_string() { Ok(Jewelries::Necklace) }
        else if value == Jewelries::Ring.to_string() { Ok(Jewelries::Ring) }
        else { Err(format!("{} is not a jewelry", value)) }
    }
}

impl Display for Jewelries {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Jewelries::Necklace => write!(f, "Collar"),
            Jewelries::Ring => write!(f, "Anillo"),
        }
    }
}

impl Display for JewelryTraits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            JewelryTraits::Arcane => write!(f, "Arcanidad"),
            JewelryTraits::Bloodthirsty => write!(f, "Sed de sangre"),
            JewelryTraits::Harmony => write!(f, "Armonía"),
            JewelryTraits::Healthy => write!(f, "Saludable"),
            JewelryTraits::Infused => write!(f, "Imbuido"),
            JewelryTraits::Protective => write!(f, "Protección"),
            JewelryTraits::Robust => write!(f, "Robustez"),
            JewelryTraits::Swift => write!(f, "Agilidad"),
            JewelryTraits::Triune => write!(f, "Trinidad"),
        }
    }
}

impl ItemInfo for JewelryTraits {
    fn description(&self) -> String {
        match *self {
            JewelryTraits::Arcane => "Aumenta la magia máxima".to_string(),
            JewelryTraits::Bloodthirsty => "Aumenta el daño en enemigos por debajo del 90%".to_string(),
            JewelryTraits::Harmony => "Al activar una sinergia, restaura salud, magia y aguante".to_string(),
            JewelryTraits::Healthy => "Aumenta la salud máxima".to_string(),
            JewelryTraits::Infused => "Aumenta la eficacia del encantamiento de la joyería".to_string(),
            JewelryTraits::Protective => "Aumenta la resistencia a hechizos y física".to_string(),
            JewelryTraits::Robust => "Aumenta el aguante máximo".to_string(),
            JewelryTraits::Swift => "Aumenta la velocidad de movimiento".to_string(),
            JewelryTraits::Triune => "Aumenta la magia, aguante y salud máximas".to_string(),
        }
    }
}

impl Display for JewelryEnchantments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            JewelryEnchantments::IncreasePhysicalHarm => write!(f, "Glifo de aumento de daño físico"),
            JewelryEnchantments::IncreaseMagicalHarm => write!(f, "Glifo de aumento de daño mágico"),
            JewelryEnchantments::HealthRecovery => write!(f, "Glifo de regeneración de salud"),
            JewelryEnchantments::MagickaRecovery => write!(f, "Glifo de regeneración de magia"),
            JewelryEnchantments::StaminaRecovery => write!(f, "Glifo de regeneración de aguante"),
            JewelryEnchantments::ReduceSpellCost => write!(f, "Glifo de reducción de coste de magia"),
            JewelryEnchantments::ReduceFeatCost => write!(f, "Glifo de reducción de coste de aguante"),
            JewelryEnchantments::Shielding => write!(f, "Glifo de bloqueo"),
            JewelryEnchantments::Bashing => write!(f, "Glifo de percusión"),
            JewelryEnchantments::DecreasePhysicalHarm => write!(f, "Glifo de resistencia al daño físico"),
            JewelryEnchantments::DecreaseSpellHarm => write!(f, "Glifo de resistencia al daño mágico"),
            JewelryEnchantments::FlameResist => write!(f, "Glifo de resistencia al fuego"),
            JewelryEnchantments::FrostResist => write!(f, "Glifo de resistencia a la congelación"),
            JewelryEnchantments::ShockResist => write!(f, "Glifo de resistencia a las descargas"),
            JewelryEnchantments::PoisonResist => write!(f, "Glifo de resistencia al veneno"),
            JewelryEnchantments::DiseaseResist => write!(f, "Glifo de resistencia a las enfermedades"),
            JewelryEnchantments::PotionResist => write!(f, "Glifo de amplificación alquímica"),
            JewelryEnchantments::PotionBoost => write!(f, "Glifo de aceleración alquímica"),
            JewelryEnchantments::ReduceSkillCost => write!(f, "Glifo de reducción de coste de habilidades"),
            JewelryEnchantments::PrismaticRecovery => write!(f, "Glifo de regeneración prismática"),
        }
    }
}

impl ItemInfo for JewelryEnchantments {
    fn description(&self) -> String {
        match *self {
            JewelryEnchantments::IncreasePhysicalHarm => "Añade daño de arma y hechizo, y recuperación de aguante".to_string(),
            JewelryEnchantments::IncreaseMagicalHarm => "Añade daño de arma y hechizo, y recuperación de magia".to_string(),
            JewelryEnchantments::HealthRecovery => "Añade recuperación de salud".to_string(),
            JewelryEnchantments::MagickaRecovery => "Añade recuperación de magia".to_string(),
            JewelryEnchantments::StaminaRecovery => "Añade recuperación de aguante".to_string(),
            JewelryEnchantments::ReduceSpellCost => "Reduce el coste de magia de las habilidades".to_string(),
            JewelryEnchantments::ReduceFeatCost => "Reduce el coste de aguante de las habilidades".to_string(),
            JewelryEnchantments::Shielding => "Reduce el coste de bloquear".to_string(),
            JewelryEnchantments::Bashing => "Añade daño a tus ataques de aporreo".to_string(),
            JewelryEnchantments::DecreasePhysicalHarm => "Añade resistencia física".to_string(),
            JewelryEnchantments::DecreaseSpellHarm => "Añade resistencia a los hechizos".to_string(),
            JewelryEnchantments::FlameResist => "Añade resistencia a las llamas".to_string(),
            JewelryEnchantments::FrostResist => "Añade resistencia a la escarcha".to_string(),
            JewelryEnchantments::ShockResist => "Añade resistencia a descargas eléctricas".to_string(),
            JewelryEnchantments::PoisonResist => "Añade resistencia a venenos".to_string(),
            JewelryEnchantments::DiseaseResist => "Añade resistencia a enfermedades".to_string(),
            JewelryEnchantments::PotionResist => "Aumenta la duración de los efectos de las pociones".to_string(),
            JewelryEnchantments::PotionBoost => "Reduce la reutilización de las pociones".to_string(),
            JewelryEnchantments::ReduceSkillCost => "Reduce el coste de salud, magia y aguante de las habilidades".to_string(),
            JewelryEnchantments::PrismaticRecovery => "Añade recuperación de magia, salud y aguante".to_string(),
        }
    }
}