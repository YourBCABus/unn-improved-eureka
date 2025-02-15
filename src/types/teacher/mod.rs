pub mod pronouns;

use std::fmt::{Debug, Display};

use uuid::Uuid;
use serde::{ Serialize, Deserialize };

use self::pronouns::PronounSet;

use async_graphql::{ InputObject, InputValueError, InputValueResult, Scalar, ScalarType, Value };


#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Honorific {
    Ms, Mx, Mr, Dr, Mrs, Prof,
    MrDr, MsDr, MxDr,
    Sir, Dame,
    Madame, Mademoiselle, Monsieur,
    Profe, Señora, Señor, Señorita,
}
impl Honorific {
    pub fn try_from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "ms" | "miss" => Some(Self::Ms),
            "mx" | "mix" => Some(Self::Mx),
            "mr" | "mister" => Some(Self::Mr),
            "dr" | "doctor" => Some(Self::Dr),
            "mrs" | "missus" => Some(Self::Mrs),
            
            "mr dr" | "mr. dr" | "mister doctor" => Some(Self::MrDr),
            
            | "ms dr" | "ms. dr"
            | "mrs dr" | "mrs. dr"
            | "miss doctor" | "missus doctor" => Some(Self::MsDr),
            
            "mx dr" | "mx. dr" | "mix doctor" => Some(Self::MxDr),

            
            "prof" | "professor" => Some(Self::Prof),
            "sir" => Some(Self::Sir),
            "dame" => Some(Self::Dame),

            "mme" | "madame" => Some(Self::Madame),
            "mlle" | "mademoiselle" => Some(Self::Mademoiselle),
            "m" | "monsieur" => Some(Self::Monsieur),

            "profe" | "profesor" | "profesora" | "profesore" => Some(Self::Profe),
            "sra" | "senora" | "señora" => Some(Self::Señora),
            "sr" | "senor" | "señor" => Some(Self::Señor),
            "srta" | "senorita" | "señorita" => Some(Self::Señorita),

            _ => None
        }
    }
    pub fn str(&self) -> &'static str {
        match self {
            Self::Ms => "Ms",
            Self::Mx => "Mx",
            Self::Mr => "Mr",
            Self::Dr => "Dr",
            Self::Mrs => "Mrs",
            Self::Prof => "Prof",

            Self::MrDr => "Mr. Dr",
            Self::MsDr => "Ms. Dr",
            Self::MxDr => "Mx. Dr",

            Self::Sir => "Sir",
            Self::Dame => "Dame",

            Self::Madame => "Mme",
            Self::Mademoiselle => "Mlle",
            Self::Monsieur => "M",

            Self::Profe => "Profe",
            Self::Señora => "Sra",
            Self::Señor => "Sr",
            Self::Señorita => "Srta",
        }
    }
    pub fn is_abbreviation(&self) -> bool {
        !matches!(self, Self::Sir | Self::Dame | Self::Madame | Self::Mademoiselle)
    }
}
impl Debug for Honorific {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Honorific<{}>", self.str())
    }
}
impl Display for Honorific {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str())?;
        if self.is_abbreviation() {
            write!(f, ".")?;
        }
        Ok(())
    }
}

#[Scalar]
impl ScalarType for Honorific {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            if let Some(honorific) = Self::try_from_str(value) {
                Ok(honorific)
            } else {
                Err(InputValueError::custom(format!("Invalid honorific: {value:?}")))
            }
        } else {
            // If the type does not match
            Err(InputValueError::expected_type(value))
        }
    }
    fn to_value(&self) -> Value {
        self.str().into()
    }
}


#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, InputObject)]
pub struct MiddleName {
    pub vis: bool,
    pub name: String,
}


#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, InputObject)]
#[graphql(input_name = "TeacherNameInput")]
pub struct TeacherName {
    pub (super) honorific: Honorific,
    pub (super) first: String,
    pub (super) last: String,
    pub (super) middle: Vec<MiddleName>,
}
impl TeacherName {
    pub fn new(honorific: Honorific, first: String, last: String, middle: Vec<MiddleName>) -> Self {
        Self { honorific, first, last, middle }
    }

    
    pub fn get_honorific(&self) -> Honorific { self.honorific }
    pub fn get_first(&self) -> &str { &self.first }
    pub fn get_last(&self) -> &str { &self.last }

    pub fn all_middles(&self) -> impl Iterator<Item = &MiddleName> {
        self.middle.iter()
    }
    pub fn visible_middles(&self) -> impl Iterator<Item = &'_ str> + '_ {
        self.middle.iter().filter_map(|middle| middle.vis.then_some(middle.name.as_str()))
    }

    pub fn short(&self) -> String {
        format!("{} {}", self.honorific, self.last)
    }
    pub fn mid_len(&self) -> String {
        format!("{} {} {}", self.honorific, self.first, self.last)
    }
    pub fn longest(&self) -> String {
        let mut output = format!("{} {} ", self.honorific, self.first);
        for name in self.visible_middles() {
            output.push_str(name);
            output.push(' ');
        }
        output.push_str(&self.last);
        output
    }
}
impl Debug for TeacherName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {:?} ", self.honorific, self.first)?;
        for name in self.visible_middles() {
            write!(f, "{name:?} ")?;
        }
        write!(f, "{}", self.last)?;
        Ok(())
    }
}
impl Display for TeacherName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ", self.honorific, self.first)?;
        for name in self.visible_middles() {
            write!(f, "{name} ")?;
        }
        write!(f, "{}", self.last)?;
        Ok(())
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Teacher {
    pub (super) id: Uuid,
    pub (super) name: TeacherName,
    pub (super) pronouns: PronounSet,
    pub (super) fully_absent: bool,
    pub (super) comments: Option<String>
}

impl Teacher {
    pub fn new(id: Uuid, name: TeacherName, pronouns: PronounSet, comments: Option<String>) -> Self {
        Self { id, name, pronouns, fully_absent: false, comments }
    }
    pub fn with_fully_absence(self, fully_absent: bool) -> Self {
        Self { fully_absent, ..self }
    }

    pub fn get_id(&self) -> Uuid { self.id }
    pub fn get_name(&self) -> &TeacherName { &self.name }
    pub fn get_pronouns(&self) -> &PronounSet { &self.pronouns }
    pub fn get_fully_absent(&self) -> bool { self.fully_absent }
    pub fn get_comments(&self) -> Option<&str> { self.comments.as_deref() }
}
