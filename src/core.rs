use std::collections::HashMap;

use crate::pokemon::MegaPokemons;
use crate::r#type::TypeBitFlag;

pub fn build_type_mons_map(megas: &MegaPokemons) -> HashMap<TypeBitFlag, Vec<&str>> {
    let mut type_megas: HashMap<TypeBitFlag, Vec<&str>> = HashMap::new();
    for mon in megas.0.iter() {
        let mut types = vec![mon.type_1];
        if let Some(type_2) = mon.type_2 {
            types.push(type_2);
        }
        for t in types.into_iter() {
            match type_megas.get_mut(&t) {
                Some(mons) => {
                    mons.push(&mon.name);
                }
                None => {
                    type_megas.insert(t, vec![&mon.name]);
                }
            }
        }
    }
    type_megas
}
