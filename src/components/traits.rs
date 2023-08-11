use serenity::builder::CreateSelectMenu;

const ARMOR_TRAITS: [&str; 8] = ["Cabeza", "Hombros", "Pecho", "Manos", "Cintura", "Piernas", "Pies", "Escudo"];
const JEWELRY_TRAITS: [&str; 3] = ["Amuleto", "Anillo #1", "Anillo #2"];

pub fn has_armor_trait(name: &str) -> bool {
    ARMOR_TRAITS.contains(&name)
}

pub fn has_jewelry_trait(name: &str) -> bool {
    JEWELRY_TRAITS.contains(&name)
}

pub fn get_trait(part: &str, name: &str) -> CreateSelectMenu {
    if has_armor_trait(part) {
        gear_armor_traits(name)
    } else if has_jewelry_trait(part) {
        gear_jewelry_traits(part)
    } else {
        gear_weapon_traits(part)
    }
}

pub fn gear_armor_traits(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el rasgo que quieres");
    b.options(|opts| opts
        .create_option(|o| o.label("Divines").value("Divines").description("Aumenta el efecto de la piedra Mundus"))
        .create_option(|o| o.label("Invigorating").value("Invigorating").description("Aumenta la regeneracion de recursos"))
        .create_option(|o| o.label("Impenetrable").value("Impenetrable").description("Reistencia a daño critico y reduce la mitad el desgaste"))
        .create_option(|o| o.label("Infused").value("Infused").description("Aumenta el efecto del encantamiento equipado"))
        .create_option(|o| o.label("Nirnhoned").value("Nirnhoned").description("Aumenta las resistencias"))
        .create_option(|o| o.label("Reinforced").value("Reinforced").description("Aumenta la armadura"))
        .create_option(|o| o.label("Sturdy").value("Sturdy").description("Reduce coste de bloqueo"))
        .create_option(|o| o.label("Training").value("Training").description("Incrementa la experiencia por muertes"))
        .create_option(|o| o.label("Well-Fitted").value("Well-Fitted").description("Reduce el coste de esquivar"))
    );
    b
}

pub fn gear_jewelry_traits(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el rasgo que quieres");
    b.options(|opts| opts
        .create_option(|o| o.label("Arcane").value("Arcane").description("Incrementa el Magicka maximo"))
        .create_option(|o| o.label("Bloodthirsty").value("Bloodthirsty"). description("Aumenta el daño de arma/hechizo contra objetivos por debajo del 90%"))
        .create_option(|o| o.label("Harmony").value("Harmony").description("Recupera recursos cada vez que activas una sinergia"))
        .create_option(|o| o.label("Healthy").value("Healthy").description("Incrementa la Salud maxima"))
        .create_option(|o| o.label("Infused").value("Infused").description("Aumenta el efecto del encantamiento equipado"))
        .create_option(|o| o.label("Protective").value("Protective").description("Aumenta las resistencias"))
        .create_option(|o| o.label("Robust").value("Robust").description("Incrementa el Aguante maximo"))
        .create_option(|o| o.label("Swift").value("Swift").description("Aumenta la velocidad de movimiento"))
        .create_option(|o| o.label("Triune").value("Triune").description("Incremetna vida, aguante y magicka maximas"))
    );
    b
}

pub fn gear_weapon_traits(name: &str) -> CreateSelectMenu {
    let mut b = CreateSelectMenu::default();
    b.custom_id(name);
    b.placeholder("Selecciona el rasgo que quieres");
    b.options(|opts| opts
        .create_option(|o| o.label("Charged").value("Charged").description("Mayor probabilidad de causar efectos de estado"))
        .create_option(|o| o.label("Defending").value("Defending").description("Aumenta la armadura"))
        .create_option(|o| o.label("Powered").value("Powered").description("Aumenta la curacion realizada"))
        .create_option(|o| o.label("Infused").value("Infused").description("Reduce el enfriamiento del encantamiento equipado"))
        .create_option(|o| o.label("Nirnhoned").value("Nirnhoned").description("Aumenta el daño de arma"))
        .create_option(|o| o.label("Precise").value("Precise").description("Aumenta probabilidad de critico"))
        .create_option(|o| o.label("Sharpened").value("Sharpened").description("Aumenta la penetracion"))
        .create_option(|o| o.label("Training").value("Training").description("Incrementa la experiencia por muertes"))
        .create_option(|o| o.label("Decisive").value("Decisive").description("Mayor obtencion de puntos de maxima"))
    );
    b
}