use std::fmt::{Display, Formatter};
use crate::entities::ItemInfo;

pub enum OneHandedWeapons {
    Mace, Dagger, Sword, Axe
}

pub enum TwoHandedWeapons {
    Mace, Sword, Axe, FrostStaff, FireStaff, LightningStaff, RestorationStaff, Bow
}

pub enum WeaponKind {
    OneHanded(OneHandedWeapons),
    TwoHanded(TwoHandedWeapons)
}

pub enum WeaponTraits {
    Charged, Defending, Powered, Infused, Nirnhoned, Precise, Sharpened, Training, Decisive
}

pub enum WeaponEnchantments {
    Fire, Frost, Shock, Poison, Foulness, DecreaseHealth, Hardening, AbsorbHealth, AbsorbMagicka,
    AbsorbStamina, WeaponDamage, Weakening, Crushing, PrismaticOnslaught
}

pub struct Weapon {
    pub kind: WeaponKind,
    pub weapon_trait: WeaponTraits,
    pub enchantment: WeaponEnchantments
}

impl Display for OneHandedWeapons {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            OneHandedWeapons::Mace => write!(f, "Maza"),
            OneHandedWeapons::Dagger => write!(f, "Daga"),
            OneHandedWeapons::Sword => write!(f, "Espada"),
            OneHandedWeapons::Axe => write!(f, "Hacha"),
        }
    }
}

impl ItemInfo for OneHandedWeapons {
    fn description(&self) -> String {
        "A una mano".to_string()
    }
}

impl Display for TwoHandedWeapons {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            TwoHandedWeapons::Mace => write!(f, "Mazo"),
            TwoHandedWeapons::Sword => write!(f, "Mandoble"),
            TwoHandedWeapons::Axe => write!(f, "Hacha de combate"),
            TwoHandedWeapons::FrostStaff => write!(f, "Bastón eléctrico"),
            TwoHandedWeapons::FireStaff => write!(f, "Bastón infernal"),
            TwoHandedWeapons::LightningStaff => write!(f, "Bastón de restauración"),
            TwoHandedWeapons::RestorationStaff => write!(f, "Bastón glacial"),
            TwoHandedWeapons::Bow => write!(f, "Arco"),
        }
    }
}

impl ItemInfo for TwoHandedWeapons {
    fn description(&self) -> String {
        "A dos manos".to_string()
    }
}

impl Display for WeaponTraits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            WeaponTraits::Charged => write!(f, "Carga"),
            WeaponTraits::Defending => write!(f, "Defensa"),
            WeaponTraits::Powered => write!(f, "Potencia"),
            WeaponTraits::Infused => write!(f, "Imbuido"),
            WeaponTraits::Nirnhoned => write!(f, "Temple de Nirn"),
            WeaponTraits::Precise => write!(f, "Precisión"),
            WeaponTraits::Sharpened => write!(f, "Filo"),
            WeaponTraits::Training => write!(f, "Entrenamiento"),
            WeaponTraits::Decisive => write!(f, "Decisivo"),
        }
    }
}

impl ItemInfo for WeaponTraits {
    fn description(&self) -> String {
        match *self {
            WeaponTraits::Charged => "Aumenta la probabilidad de aplicar efectos de estado".to_string(),
            WeaponTraits::Defending => "Aumenta la resistencia física y a hechizos".to_string(),
            WeaponTraits::Powered => "Aumenta la curación realizada".to_string(),
            WeaponTraits::Infused => "Aumenta el efecto de encantamiento de las armas y reduce el tiempo de reutilización del encantamiento".to_string(),
            WeaponTraits::Nirnhoned => "Aumenta el daño del arma".to_string(),
            WeaponTraits::Precise => "Aumenta el crítico de arma y hechizo".to_string(),
            WeaponTraits::Sharpened => "Aumenta la penetración física y mágica".to_string(),
            WeaponTraits::Training => "Aumenta la experiencia ganada con cada muerte".to_string(),
            WeaponTraits::Decisive => "Al ganar puntos de máxima, tendras posibilidad de ganar 1 punto extra".to_string(),
        }
    }
}

impl Display for WeaponEnchantments {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            WeaponEnchantments::Fire => write!(f, "Glifo de fuego"),
            WeaponEnchantments::Frost => write!(f, "Glifo de escarcha"),
            WeaponEnchantments::Shock => write!(f, "Glifo de descarga"),
            WeaponEnchantments::Poison => write!(f, "Glifo de veneno"),
            WeaponEnchantments::Foulness => write!(f, "Glifo de podredumbre"),
            WeaponEnchantments::DecreaseHealth => write!(f, "Glifo de disminución de salud"),
            WeaponEnchantments::Hardening => write!(f, "Glifo de robustez"),
            WeaponEnchantments::AbsorbHealth => write!(f, "Glifo de absorción de salud"),
            WeaponEnchantments::AbsorbMagicka => write!(f, "Glifo de absorción de magia"),
            WeaponEnchantments::AbsorbStamina => write!(f, "Glifo de absorción de aguante"),
            WeaponEnchantments::WeaponDamage => write!(f, "Glifo de daño por arma"),
            WeaponEnchantments::Weakening => write!(f, "Glifo de debilidad"),
            WeaponEnchantments::Crushing => write!(f, "Glifo de aplastamiento"),
            WeaponEnchantments::PrismaticOnslaught => write!(f, "Glifo de asalto prismático"),
        }
    }
}

impl ItemInfo for WeaponEnchantments {
    fn description(&self) -> String {
        match *self {
            WeaponEnchantments::Fire => "Inflige daño de llamas".to_string(),
            WeaponEnchantments::Frost => "Inflige daño de escarcha".to_string(),
            WeaponEnchantments::Shock => "Inflige daño de descarga eléctica".to_string(),
            WeaponEnchantments::Poison => "Inflige daño de veneno".to_string(),
            WeaponEnchantments::Foulness => "Inflige daño de enfermedad".to_string(),
            WeaponEnchantments::DecreaseHealth => "Inflige daño de Oblivion según una parte de la salud máxima del enemigo".to_string(),
            WeaponEnchantments::Hardening => "Otorga un escudo de daño que protege del daño".to_string(),
            WeaponEnchantments::AbsorbHealth => "Inglige daño de magia y restablece salud".to_string(),
            WeaponEnchantments::AbsorbMagicka => "Inflige daño de magia y recuperas magia".to_string(),
            WeaponEnchantments::AbsorbStamina => "Inflige daño físico y recuperas aguante".to_string(),
            WeaponEnchantments::WeaponDamage => "Aumenta el daño de arma y hechizo".to_string(),
            WeaponEnchantments::Weakening => "Reduce el daño de arma y hechizo del objetivo".to_string(),
            WeaponEnchantments::Crushing => "Reduce la resistencia física y a hechizos del objetivo".to_string(),
            WeaponEnchantments::PrismaticOnslaught => "Inflige daño de magia y restablece salud, magia y aguante".to_string(),
        }
    }
}