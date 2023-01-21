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
#[derive(Clone)]
struct Matching {
    pub edges: Vec<(usize, usize)>,

    // sum of tuple elements in edges, sorted
    pub sorted_sums: Vec<usize>,
}

impl Matching {
    pub fn from_pairing(pairing: Pairing) -> Self {
        let edges: Vec<(usize, usize)> = pairing.edges().collect();
        let mut sorted_sums: Vec<usize> = edges.iter().map(|(a, b)| a + b).collect();
        sorted_sums.sort();
        Self { edges, sorted_sums }
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

pub fn find_min_mega_for_type_coverage() {
    let megas = MegaPokemons::new();
    find_all(&megas);
}

pub fn find_min_mega_for_type_coverage_with_primals() {
    let megas = MegaPokemons::new_with_primals();
    find_one(&megas);
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
            let types_graph = build_types_graph_skip_edges(&types, &megas.0, skips);

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
/// skip edges if provided
fn build_types_graph_skip_edges(
    types: &Vec<TypeBitFlag>,
    mega_mons: &Vec<Pokemon>,
    skips: Vec<(usize, usize)>,
) -> DefaultGraph {
    let mut types_graph = DefaultGraph::new();
    for t in types.iter() {
        let _ = types_graph.add_node(t.clone() as usize);
    }
    let sum_skips: Vec<usize> = skips.iter().map(|(a, b)| a + b).collect();
    for mon in mega_mons.iter() {
        if let Some(type_2) = mon.type_2 {
            let types_sum = mon.type_1 as usize + type_2 as usize;
            if sum_skips.contains(&types_sum) {
                continue;
            }
            let _ = types_graph.add_edge(mon.type_1 as usize, type_2 as usize);
        }
    }

    types_graph
}
