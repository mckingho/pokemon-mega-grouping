use crate::core::build_type_mons_map;
use crate::pokemon::MegaPokemons;
use crate::r#type::TypeBitFlag;

pub fn print_per_type_ascending() {
    let megas = MegaPokemons::new();

    let type_mons_map = build_type_mons_map(&megas);

    let mut types: Vec<TypeBitFlag> = type_mons_map.keys().cloned().collect();
    types.sort_by(|a, b| {
        type_mons_map
            .get(a)
            .unwrap()
            .len()
            .cmp(&type_mons_map.get(b).unwrap().len())
    });

    for t in types.iter() {
        let mons = type_mons_map.get(t).unwrap();
        println!("{:?} ({}): {}", t, mons.len(), mons.join(", "));
    }
}
