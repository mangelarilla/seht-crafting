use strum_macros::{Display, EnumString};

#[derive(EnumString, Display)]
pub enum PartMaterials {
    #[strum(serialize = "Seda ancestral")] AncestorSilk,
    #[strum(serialize = "Cuero rubedo")] RubedoLeather,
    #[strum(serialize = "Lingote de rubedita")] RubediteIngots,
    #[strum(serialize = "Madera de fresno rubí lijado")] SandedRubyAsh,
    #[strum(serialize = "Onza de platino")] PlatinumOunces
}

#[derive(EnumString, Display)]
pub enum RuneQualityMaterials {
    Ta, Jejota, Denata, Rekuta, Kuta
}

#[derive(EnumString, Display)]
pub enum TailoringQualityMaterials {
    #[strum(serialize = "Hilo de coser")] Hemming,
    #[strum(serialize = "Bordado")] Embroidery,
    #[strum(serialize = "Revestimiento elegante")] ElegantLining,
    #[strum(serialize = "Cera de dreugh")] DreughWax
}

#[derive(EnumString, Display)]
pub enum BlacksmithQualityMaterials {
    #[strum(serialize = "Piedra de esmeril")] HoningStone,
    #[strum(serialize = "Aceite enano")] DwarvenOil,
    #[strum(serialize = "Disolvente granulado")] GrainSolvent,
    #[strum(serialize = "Aleación de temple")] TemperingAlloy
}

#[derive(EnumString, Display)]
pub enum WoodworkingQualityMaterials {
    #[strum(serialize = "Brea")] Pitch,
    #[strum(serialize = "Turpen")] Turpen,
    #[strum(serialize = "Masilla")] Mastic,
    #[strum(serialize = "Colofonia")] Rosin
}

#[derive(EnumString, Display)]
pub enum JewelryQualityMaterials {
    #[strum(serialize = "Chapado de terne")] TernePlating,
    #[strum(serialize = "Chapado de iridio")] IridiumPlating,
    #[strum(serialize = "Chapado de circón")] ZirconPlating,
    #[strum(serialize = "Chapado de cromo")] ChromiumPlating
}

#[derive(EnumString, Display)]
pub enum ArmourTraitMaterials {
    #[strum(serialize = "Zafiro")] Sapphire,
    #[strum(serialize = "Diamante")] Diamond,
    #[strum(serialize = "Piedra de sangre")] Bloodstone,
    #[strum(serialize = "Granate")] Garnet,
    #[strum(serialize = "Nirncrux fortificado")] FortifiedNirncrux,
    #[strum(serialize = "Sardónice")] Sardonyx,
    #[strum(serialize = "Cuarzo")] Quartz,
    #[strum(serialize = "Esmeralda")] Emerald,
    #[strum(serialize = "Almandino")] Almandine
}

#[derive(EnumString, Display)]
pub enum WeaponTraitMaterials {
    #[strum(serialize = "Amatista")] Amethyst,
    #[strum(serialize = "Citrina")] Citrine,
    #[strum(serialize = "Turquesa")] Turquoise,
    Jade,
    #[strum(serialize = "Temple de Nirn")] PotentNirncrux,
    #[strum(serialize = "Crisolita")] Chysolite,
    #[strum(serialize = "Rubí")] Ruby,
    #[strum(serialize = "Ópalo de fuego")] FireOpal,
    #[strum(serialize = "Cornalina")] Carnelian
}

#[derive(EnumString, Display)]
pub enum JewelryTraitMaterials {
    #[strum(serialize = "Cobalto")] Cobalt,
    #[strum(serialize = "Piedra masacre")] Slaughterstone,
    #[strum(serialize = "Dibelio")] Dibellium,
    #[strum(serialize = "Antimonio")] Antimony,
    #[strum(serialize = "Ámbar aúrbico")] AurbicAmber,
    #[strum(serialize = "Titanio")] Titanium,
    #[strum(serialize = "Cinc")] Zinc,
    #[strum(serialize = "Cera dorada")] GildingWax,
    #[strum(serialize = "Prisma del alba")] DawnPrism
}

#[derive(EnumString, Display)]
pub enum PotencyRunes {
    Repora, Itade
}

#[derive(EnumString, Display)]
pub enum EssenceRunes {
    Dekeipa, Deni, Denima, Deteri, Hakeijo, Haoko, Indeko, Kaderi, Kuoko,
    Makderi, Makko, Makkoma, Meip, Oko, Okoma, Okori, Oru, Rakeipa, Taderi
}

