use armour::Armour;
use jewelry::Jewelry;
use weapon::Weapon;

pub mod armour;
pub mod weapon;
pub mod jewelry;

pub enum Gear {
    Weapon(Weapon),
    Armour(Armour),
    Jewelry(Jewelry)
}

pub trait ItemInfo {
    fn description(&self) -> String;
}