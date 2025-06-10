use std::fmt::Display;

use regex::Regex;
use serde::{Deserialize, Serialize, de::Visitor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents a Kin. To represent the Kin Tree, some variants have an Option. The None variant represents the parent Kin, while the Some variant represents a child.
pub enum Kin {
    Bird,
    Assassin,
    Undead,
    Reptile,
    CultOfNa,
    Sorcery,
    Their,
    Insect(Option<InsectKin>),
    Piezan(Option<PiezanKin>),
    Machine(Option<MachineKin>),
}

impl Display for Kin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undead => write!(f, "Undead Kin"),
            Self::Bird => write!(f, "Bird Kin"),
            Self::Assassin => write!(f, "Assassin Kin"),
            Self::Reptile => write!(f, "Reptile Kin"),
            Self::CultOfNa => write!(f, "Cult of Nä Kin"),
            Self::Sorcery => write!(f, "Sorcery Kin"),
            Self::Their => write!(f, "THEIR_KIN"),
            Self::Insect(insect_kin) => match insect_kin {
                Some(kin) => write!(f, "{kin}"),
                None => write!(f, "Insect Kin"),
            },
            Self::Piezan(piezan_kin) => match piezan_kin {
                Some(kin) => write!(f, "{kin}"),
                None => write!(f, "Insect Kin"),
            },
            Self::Machine(machine_kin) => match machine_kin {
                Some(kin) => write!(f, "{kin}"),
                None => write!(f, "Insect Kin"),
            },
        }
    }
}

impl Kin {
    pub fn get_name(self) -> &'static str {
        match self {
            Self::Assassin => "assassin",
            Self::Bird => "bird",
            Self::CultOfNa => "cult of na",
            Self::Reptile => "reptile",
            Self::Sorcery => "sorcery",
            Self::Undead => "undead",
            Self::Insect(insect_kin) => insect_kin.map_or("insect", InsectKin::get_name),
            Self::Piezan(piezan_kin) => piezan_kin.map_or("piezan", PiezanKin::get_name),
            Self::Machine(machine_kin) => machine_kin.map_or("machine", MachineKin::get_name),
            Self::Their => "THEIR",
        }
    }

    /// Returns true if `other` is the same kin or a child kin of `self`
    #[must_use]
    pub fn is_same_or_child(self, other: Self) -> bool {
        #[allow(clippy::match_same_arms)]
        match (self, other) {
            // Childless kins
            (Self::Undead, Self::Undead) => true,
            (Self::Assassin, Self::Assassin) => true,
            (Self::Reptile, Self::Reptile) => true,
            (Self::Sorcery, Self::Sorcery) => true,
            (Self::Their, Self::Their) => true,
            (_, Self::Sorcery) | (Self::Sorcery, _) => false,
            (_, Self::Their) | (Self::Their, _) => false,
            // Insect kins
            (Self::Insect(_), Self::Insect(None)) => true,
            (Self::Insect(None), Self::Insect(_)) => false,
            (Self::Insect(Some(a)), Self::Insect(Some(b))) => a == b,
            // Piezan kins
            (Self::Piezan(_), Self::Piezan(None)) => true,
            (Self::Piezan(None), Self::Piezan(_)) => false,
            (Self::Piezan(Some(a)), Self::Piezan(Some(b))) => a == b,
            // Machine kins
            (Self::Machine(_), Self::Machine(None)) => true,
            (Self::Machine(None), Self::Machine(_)) => false,
            (Self::Machine(Some(a)), Self::Machine(Some(b))) => a == b,
            _ => false,
        }
    }

    #[must_use]
    pub fn get_equalness(self, other: Self) -> f64 {
        if self.is_same_or_child(other) {
            if self == other { 1.0 } else { 0.5 }
        } else {
            0.0
        }
    }

    #[must_use]
    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            "THEIR" | "their" | "THEY" | "they" => Some(Self::Their),
            "sorcery" => Some(Self::Sorcery),
            "assassin" => Some(Self::Assassin),
            "reptile" => Some(Self::Reptile),
            "cult of na" | "cult of nä" => Some(Self::CultOfNa),
            "undead" => Some(Self::Undead),
            "insect" => Some(Self::Insect(None)),
            "piezan" => Some(Self::Piezan(None)),
            "machine" => Some(Self::Machine(None)),
            "ant" => Some(Self::Insect(Some(InsectKin::Ant))),
            "bee" => Some(Self::Insect(Some(InsectKin::Bee))),
            "blight" => Some(Self::Machine(Some(MachineKin::Blight))),
            "red kingdom" => Some(Self::Piezan(Some(PiezanKin::RedKingdom))),
            "blue kingdom" => Some(Self::Piezan(Some(PiezanKin::BlueKingdom))),
            "green kingdom" => Some(Self::Piezan(Some(PiezanKin::GreenKingdom))),
            "black kingdom" => Some(Self::Piezan(Some(PiezanKin::BlackKingdom))),
            _ => None,
        }
    }
}

impl Serialize for Kin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.get_name())
    }
}

struct KinVisitor;
impl Visitor<'_> for KinVisitor {
    type Value = Kin;

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::Value::from_string(v).ok_or_else(|| E::custom(format!("{v} is not a valid kin")))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a valid kin name")
    }
}

impl<'de> Deserialize<'de> for Kin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(KinVisitor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsectKin {
    Ant,
    Bee,
}

impl Display for InsectKin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ant => write!(f, "Ant Kin"),
            Self::Bee => write!(f, "Bee Kin"),
        }
    }
}

impl InsectKin {
    #[must_use]
    pub const fn get_name(self) -> &'static str {
        match self {
            Self::Ant => "ant",
            Self::Bee => "bee",
        }
    }

    #[must_use]
    pub fn get_equalness(self, other: Self) -> f64 {
        if self == other { 1.0 } else { 0.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PiezanKin {
    RedKingdom,
    BlueKingdom,
    BlackKingdom,
    GreenKingdom,
}

impl Display for PiezanKin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RedKingdom => write!(f, "Red Kingdom Kin"),
            Self::BlueKingdom => write!(f, "Blue Kingdom Kin"),
            Self::BlackKingdom => write!(f, "Black Kingdom Kin"),
            Self::GreenKingdom => write!(f, "Green Kingdom Kin"),
        }
    }
}

impl PiezanKin {
    #[must_use]
    pub const fn get_name(self) -> &'static str {
        match self {
            Self::RedKingdom => "red kingdom",
            Self::BlueKingdom => "blue kingdom",
            Self::BlackKingdom => "black kingdom",
            Self::GreenKingdom => "green kingdom",
        }
    }
    #[must_use]
    pub fn get_equalness(self, other: Self) -> f64 {
        if self == other { 1.0 } else { 0.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineKin {
    Blight,
}

impl Display for MachineKin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blight => write!(f, "Blight Kin"),
        }
    }
}

impl MachineKin {
    #[must_use]
    pub const fn get_name(self) -> &'static str {
        match self {
            Self::Blight => "blight",
        }
    }
    #[must_use]
    pub fn get_equalness(self, other: Self) -> f64 {
        if self == other { 1.0 } else { 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deser() {
        println!(
            "{}",
            serde_json::to_string_pretty(&Kin::Insect(Some(InsectKin::Ant))).unwrap()
        );
        panic!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Represents a comparison between Kins.
pub enum KinComparison {
    Equal(Kin),
    Similar(Kin),
    TextContains(String),
    TextEqual(String),
    #[serde(with = "serde_regex")]
    RegexMatch(Regex),
}

impl KinComparison {
    /// `Self::Equal` will match only exactly the same kin.
    /// `Self::Similar` will match the same kin as well as child kins (see: `Kin::is_same_or_child`)
    #[must_use]
    pub fn is_match(&self, other: Kin) -> bool {
        match self {
            Self::Equal(this) => other == *this,
            Self::Similar(this) => other.is_same_or_child(*this),
            Self::TextContains(this) => other.get_name().contains(this.as_str()),
            Self::TextEqual(this) => other.get_name().to_lowercase() == this.to_lowercase(),
            Self::RegexMatch(regex) => regex.is_match(other.get_name()),
        }
    }
}
