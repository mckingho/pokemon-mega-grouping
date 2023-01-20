use std::collections::HashSet;

use crate::core::build_type_mons_map;
use crate::pokemon::{MegaPokemons, Pokemon};
use crate::r#type::TypeBitFlag;

use gamma::graph::DefaultGraph;
use gamma::matching::{maximum_matching, Pairing};

pub fn find_min_mega_for_type_coverage() {
    let megas = MegaPokemons::new();
    find_one(megas);
}

pub fn find_min_mega_for_type_coverage_with_primals() {
    let megas = MegaPokemons::new_with_primals();
    find_one(megas);
}

fn find_one(megas: MegaPokemons) {
    let type_mons_map = build_type_mons_map(&megas);
    let types: Vec<TypeBitFlag> = TypeBitFlag::vec();

    let types_graph = build_types_graph(&types, &megas.0);

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

/// build graph of types,
/// with edges representing pokemon dual types
fn build_types_graph(types: &Vec<TypeBitFlag>, mega_mons: &Vec<Pokemon>) -> DefaultGraph {
    let mut types_graph = DefaultGraph::new();
    for t in types.iter() {
        let _ = types_graph.add_node(t.clone() as usize);
    }
    for mon in mega_mons.iter() {
        if let Some(type_2) = mon.type_2 {
            let _ = types_graph.add_edge(mon.type_1 as usize, type_2 as usize);
        }
    }

    types_graph
}
