use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, TryFromPrimitive)]
#[repr(usize)]
pub enum TypeBitFlag {
    Normal = 1,
    Fighting = 2,
    Flying = 4,
    Poison = 8,
    Ground = 16,
    Rock = 32,
    Bug = 64,
    Ghost = 128,
    Steel = 256,
    Fire = 512,
    Water = 1024,
    Grass = 2048,
    Electric = 4096,
    Psychic = 8192,
    Ice = 16384,
    Dragon = 32768,
    Dark = 65536,
    Fairy = 131072,
}

impl TypeBitFlag {
    pub fn num_to_enum(v: usize) -> Option<TypeBitFlag> {
        match Self::try_from(v) {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }
}
