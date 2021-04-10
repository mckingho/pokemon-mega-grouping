use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::r#type::TypeBitFlag;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pokemon {
    pub name: String,
    pub type_1: TypeBitFlag,
    pub type_2: Option<TypeBitFlag>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MegaPokemons(pub Vec<Pokemon>);

impl MegaPokemons {
    pub fn new() -> MegaPokemons {
        let path = Path::new("./data/megas.json");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let list: Vec<Pokemon> = serde_json::from_reader(reader).unwrap();
        MegaPokemons(list)
    }

    pub fn new_with_primals() -> MegaPokemons {
        let mut megas = MegaPokemons::new();

        let path = Path::new("./data/primals.json");
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let primals_list: Vec<Pokemon> = serde_json::from_reader(reader).unwrap();

        megas.0.extend(primals_list);
        megas
    }
}
