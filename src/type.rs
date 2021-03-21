use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
        match v {
            1 => Some(TypeBitFlag::Normal),
            2 => Some(TypeBitFlag::Fighting),
            4 => Some(TypeBitFlag::Flying),
            8 => Some(TypeBitFlag::Poison),
            16 => Some(TypeBitFlag::Ground),
            32 => Some(TypeBitFlag::Rock),
            64 => Some(TypeBitFlag::Bug),
            128 => Some(TypeBitFlag::Ghost),
            256 => Some(TypeBitFlag::Steel),
            512 => Some(TypeBitFlag::Fire),
            1024 => Some(TypeBitFlag::Water),
            2048 => Some(TypeBitFlag::Grass),
            4096 => Some(TypeBitFlag::Electric),
            8192 => Some(TypeBitFlag::Psychic),
            16384 => Some(TypeBitFlag::Ice),
            32768 => Some(TypeBitFlag::Dragon),
            65536 => Some(TypeBitFlag::Dark),
            131072 => Some(TypeBitFlag::Fairy),
            _ => None,
        }
    }
}