use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGameModeError;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl GameMode {
    pub fn to_optional(&self) -> OptionalGameMode {
        match &self {
            Self::Survival => OptionalGameMode::Survival,
            Self::Creative => OptionalGameMode::Creative,
            Self::Adventure => OptionalGameMode::Adventure,
            Self::Spectator => OptionalGameMode::Spectator,
        }
    }
}

impl From<u8> for GameMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Survival,
            1 => Self::Creative,
            2 => Self::Adventure,
            3 => Self::Spectator,
            _ => Self::Survival,
        }
    }
}

impl FromStr for GameMode {
    type Err = ParseGameModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "survival" => Ok(Self::Survival),
            "creative" => Ok(Self::Creative),
            "adventure" => Ok(Self::Adventure),
            "spectator" => Ok(Self::Spectator),
            _ => Err(ParseGameModeError),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[repr(i8)]
pub enum OptionalGameMode {
    Undefined = -1,
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl From<i8> for OptionalGameMode {
    fn from(value: i8) -> Self {
        match value {
            -1 => Self::Undefined,
            0 => Self::Survival,
            1 => Self::Creative,
            2 => Self::Adventure,
            3 => Self::Spectator,
            _ => Self::Survival,
        }
    }
}
