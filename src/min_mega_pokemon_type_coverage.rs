use std::cmp::{min, Ordering};
use std::collections::{HashMap, HashSet};

use crate::core::build_type_mons_map;
use crate::pokemon::{MegaPokemons, Pokemon};
use crate::r#type::TypeBitFlag;

use combinations::Combinations;
use gamma::graph::DefaultGraph;
use gamma::matching::{maximum_matching, Pairing};

/// Multiple Matchings in types graph
struct MultipleMatchings {
    pub matchings: Vec<Matching>,
}

impl MultipleMatchings {
    pub fn push(&mut self, m: Matching) {
        self.matchings.push(m);
    }

    pub fn is_added(&self, m: &Matching) -> bool {
        for matching in self.matchings.iter() {
            if m == matching {
                return true;
            }
        }
        false
    }

    pub fn print(&self, type_mons_map: &HashMap<TypeBitFlag, Vec<&str>>) {
        for (i, matching) in self.matchings.iter().enumerate() {
            println!("## Group {}", i + 1);
            println!("```");
            matching.print(type_mons_map);
            println!("```");
        }
    }
}

/// Matching of types graph
/// edge: pokemon with dual types
/// node: type
#[derive(Clone)]
struct Matching {
    pub edges: Vec<(usize, usize)>,

    // sum of tuple elements in edges, sorted
    pub sorted_sums: Vec<usize>,

    pub isolated_nodes: Vec<usize>,
}

impl Matching {
    pub fn from_pairing(pairing: Pairing) -> Self {
        let edges: Vec<(usize, usize)> = pairing.edges().collect();
        let mut sorted_sums: Vec<usize> = edges.iter().map(|(a, b)| a + b).collect();
        sorted_sums.sort();
        Self {
            edges,
            sorted_sums,
            isolated_nodes: vec![],
        }
    }

    pub fn from_pairing_for_pogo_primals(pairing: Pairing) -> Self {
        let edges: Vec<(usize, usize)> = pairing.edges().collect();
        let mut sorted_sums: Vec<usize> = vec![];
        let mut types_set: HashSet<TypeBitFlag> = TypeBitFlag::vec().iter().cloned().collect();
        for (a, b) in edges.iter() {
            sorted_sums.push(a + b);

            // remove connected nodes
            let type_a = TypeBitFlag::num_to_enum(*a).unwrap();
            let type_b = TypeBitFlag::num_to_enum(*b).unwrap();
            types_set.remove(&type_a);
            types_set.remove(&type_b);
        }
        sorted_sums.sort();
        let primals_bonus_types: HashSet<TypeBitFlag> = TypeBitFlag::pogo_primals_bonus_vec()
            .iter()
            .cloned()
            .collect();
        let diff = types_set.difference(&primals_bonus_types);
        let isolated_nodes: Vec<usize> = diff.map(|t| *t as usize).collect();
        Self {
            edges,
            sorted_sums,
            isolated_nodes,
        }
    }

    pub fn print(&self, type_mons_map: &HashMap<TypeBitFlag, Vec<&str>>) {
        for (pair_type_a, pair_type_b) in self.edges.iter() {
            let type_a = TypeBitFlag::num_to_enum(*pair_type_a).unwrap();
            let type_b = TypeBitFlag::num_to_enum(*pair_type_b).unwrap();
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

        // print isolated type and respective mons
        for t in self.isolated_nodes.iter() {
            let isolated_type = TypeBitFlag::num_to_enum(*t).unwrap();
            let megas = type_mons_map.get(&isolated_type).unwrap();
            println!("{:?},*: {:?}", isolated_type, megas);
        }
    }
}

impl Ord for Matching {
    fn cmp(&self, other: &Self) -> Ordering {
        // use sorted_sums to compare
        let self_len = self.sorted_sums.len();
        let other_len = other.sorted_sums.len();
        for i in 0..min(self_len, other_len) {
            if self.sorted_sums[i] < other.sorted_sums[i] {
                return Ordering::Less;
            } else if self.sorted_sums[i] < other.sorted_sums[i] {
                return Ordering::Greater;
            } else {
                continue;
            }
        }
        if self_len < other_len {
            Ordering::Less
        } else if self_len > other_len {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Matching {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Matching {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Matching {}

/*
 * Functions
 */

pub fn find_min_mega_for_type_coverage() {
    let megas = MegaPokemons::new();
    find_all(&megas);
}

#[allow(dead_code)]
pub fn find_min_mega_for_type_coverage_with_primals() {
    let megas = MegaPokemons::new_with_primals();
    find_one(&megas);
}

/// Find type coverage such that Primal Kyogre and Primal Groudon are assumed.
/// Primal Kyogre and Primal Groudon cover additional types,
/// so some types are skipped directly in this function.
pub fn find_min_mega_for_type_coverage_pogo_primals() {
    let megas = MegaPokemons::new();
    find_all_for_pogo_primals(&megas);
}

fn find_one(megas: &MegaPokemons) -> Pairing {
    let types: Vec<TypeBitFlag> = TypeBitFlag::vec();

    let types_graph = build_types_graph(&types, &megas.0);

    let mut types_pairing = Pairing::new();
    maximum_matching(&types_graph, &mut types_pairing);
    // There is perfect matching in the graph of mega types.
    // Therefore, no need to greedily extend to single node(type).

    types_pairing
}

/// find all possible matching.
fn find_all(megas: &MegaPokemons) {
    let types: Vec<TypeBitFlag> = TypeBitFlag::vec();

    // find one matching
    let mat = Matching::from_pairing(find_one(megas));
    let mat_len = mat.edges.len();
    let indices: Vec<usize> = (1..mat_len).collect(); // for combinations generation
    let mut all = MultipleMatchings { matchings: vec![] };
    all.push(mat.clone());

    let mut is_found_in_i = true;
    // remove combinations of first matching's edges, and find new matching.
    // loop i such that nCi edge(s) removal from first matching.
    for i in 1..mat_len {
        if !is_found_in_i {
            // break if no more new matching is found in last iteration
            break;
        }

        let com: Vec<Vec<usize>> = Combinations::new(indices.clone(), i).collect();

        is_found_in_i = false;
        for c in com.iter() {
            // rebuild graph by skipping combinations of edges from first matching
            let skips: Vec<(usize, usize)> = c.iter().map(|idx| mat.edges[idx - 1]).collect();
            let types_graph = build_types_graph_skip_edges(&types, &megas.0, &skips, &vec![]);

            // find new matching
            let mut types_pairing = Pairing::new();
            maximum_matching(&types_graph, &mut types_pairing);
            let new_mat = Matching::from_pairing(types_pairing);

            if new_mat.edges.len() < mat_len {
                continue;
            }
            if !all.is_added(&new_mat) {
                all.push(new_mat);
                is_found_in_i = true;
            }
        }
    }

    let type_mons_map = build_type_mons_map(megas);
    all.print(&type_mons_map);
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

/// build graph of types,
/// skip edges if skips is provided,
/// skip nodes if skip_types is provided
fn build_types_graph_skip_edges(
    types: &Vec<TypeBitFlag>,
    mega_mons: &Vec<Pokemon>,
    skips: &Vec<(usize, usize)>,
    skip_types: &Vec<TypeBitFlag>,
) -> DefaultGraph {
    let mut types_graph = DefaultGraph::new();
    for t in types.iter() {
        if skip_types.contains(t) {
            continue;
        }
        let _ = types_graph.add_node(t.clone() as usize);
    }
    let sum_skips: Vec<usize> = skips.iter().map(|(a, b)| a + b).collect();
    for mon in mega_mons.iter() {
        if skip_types.contains(&mon.type_1) {
            continue;
        }
        if let Some(type_2) = mon.type_2 {
            if skip_types.contains(&type_2) {
                continue;
            }
            let types_sum = mon.type_1 as usize + type_2 as usize;
            if sum_skips.contains(&types_sum) {
                continue;
            }
            let _ = types_graph.add_edge(mon.type_1 as usize, type_2 as usize);
        }
    }

    types_graph
}

/// find one matching for pogo primals bonus types.
fn find_one_for_pogo_primals(megas: &MegaPokemons) -> Pairing {
    let types: Vec<TypeBitFlag> = TypeBitFlag::vec();
    let skip_types: Vec<TypeBitFlag> = TypeBitFlag::pogo_primals_bonus_vec();

    let types_graph = build_types_graph_skip_edges(&types, &megas.0, &vec![], &skip_types);

    let mut types_pairing = Pairing::new();
    maximum_matching(&types_graph, &mut types_pairing);

    types_pairing
}

/// find all possible matching for pogo primals bonus types.
fn find_all_for_pogo_primals(megas: &MegaPokemons) {
    let types: Vec<TypeBitFlag> = TypeBitFlag::vec();
    let skip_types: Vec<TypeBitFlag> = TypeBitFlag::pogo_primals_bonus_vec();

    // find one matching
    let mat = Matching::from_pairing_for_pogo_primals(find_one_for_pogo_primals(megas));
    let mat_len = mat.edges.len();
    let indices: Vec<usize> = (1..mat_len).collect(); // for combinations generation
    let mut all = MultipleMatchings { matchings: vec![] };
    all.push(mat.clone());

    let mut is_found_in_i = true;
    // remove combinations of first matching's edges, and find new matching.
    // loop i such that nCi edge(s) removal from first matching.
    for i in 1..mat_len {
        if !is_found_in_i {
            // break if no more new matching is found in last iteration
            break;
        }

        let com: Vec<Vec<usize>> = Combinations::new(indices.clone(), i).collect();

        is_found_in_i = false;
        for c in com.iter() {
            // rebuild graph by skipping combinations of edges from first matching
            let skips: Vec<(usize, usize)> = c.iter().map(|idx| mat.edges[idx - 1]).collect();
            let types_graph = build_types_graph_skip_edges(&types, &megas.0, &skips, &skip_types);

            // find new matching
            let mut types_pairing = Pairing::new();
            maximum_matching(&types_graph, &mut types_pairing);
            let new_mat = Matching::from_pairing_for_pogo_primals(types_pairing);

            if new_mat.edges.len() < mat_len {
                continue;
            }
            if !all.is_added(&new_mat) {
                all.push(new_mat);
                is_found_in_i = true;
            }
        }
    }

    let type_mons_map = build_type_mons_map(megas);
    all.print(&type_mons_map);
}
