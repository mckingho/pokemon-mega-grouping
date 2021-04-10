use std::collections::{HashMap, HashSet};

use crate::pokemon::MegaPokemons;
use crate::r#type::TypeBitFlag;

use gamma::graph::DefaultGraph;
use gamma::matching::{maximum_matching, Pairing};

pub fn find_min_mega_for_type_coverage() {
    let megas = MegaPokemons::new();
    find_min_for_type_coverage(megas);
}

pub fn find_min_mega_for_type_coverage_with_primals() {
    let megas = MegaPokemons::new_with_primals();
    find_min_for_type_coverage(megas);
}

fn find_min_for_type_coverage(megas: MegaPokemons) {
    let type_mons_map = build_type_mons_map(&megas);

    let mut types_graph = DefaultGraph::new();
    let types: Vec<TypeBitFlag> = type_mons_map.keys().cloned().collect();
    for t in types.into_iter() {
        let _ = types_graph.add_node(t as usize);
    }
    for mon in megas.0.iter() {
        if let Some(type_2) = mon.type_2 {
            let _ = types_graph.add_edge(mon.type_1 as usize, type_2 as usize);
        }
    }
    let mut types_pairing = Pairing::new();
    maximum_matching(&types_graph, &mut types_pairing);
    // There is perfect matching in the graph of mega types.
    // Therefore, no need to greedily extend to single node(type).
    for (pair_type_a, pair_type_b) in types_pairing.edges() {
        let type_a = TypeBitFlag::num_to_enum(pair_type_a).unwrap();
        let type_b = TypeBitFlag::num_to_enum(pair_type_b).unwrap();
        let type_a_megas: HashSet<&str> = type_mons_map
            .get(&type_a)
            .unwrap()
            .iter()
            .cloned()
            .collect();
        let type_b_megas: HashSet<&str> = type_mons_map
            .get(&type_b)
            .unwrap()
            .iter()
            .cloned()
            .collect();
        let intersection_megas = type_a_megas.intersection(&type_b_megas);
        println!("{:?},{:?}: {:?}", type_a, type_b, intersection_megas);
    }
}

fn build_type_mons_map(megas: &MegaPokemons) -> HashMap<TypeBitFlag, Vec<&str>> {
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
