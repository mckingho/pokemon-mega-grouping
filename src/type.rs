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

    pub fn vec() -> Vec<TypeBitFlag> {
        vec![
            Self::Normal,
            Self::Fighting,
            Self::Flying,
            Self::Poison,
            Self::Ground,
            Self::Rock,
            Self::Bug,
            Self::Ghost,
            Self::Steel,
            Self::Fire,
            Self::Water,
            Self::Grass,
            Self::Electric,
            Self::Psychic,
            Self::Ice,
            Self::Dragon,
            Self::Dark,
            Self::Fairy,
        ]
    }

    pub fn pogo_primals_bonus_vec() -> Vec<TypeBitFlag> {
        vec![
            Self::Ground,   // Groudon
            Self::Bug,      // Kyogre
            Self::Fire,     // Groudon
            Self::Water,    // Kyogre
            Self::Grass,    // Groudon
            Self::Electric, // Kyogre
        ]
    }
}
