pub mod pronouns;

use std::fmt::{Debug, Display};

use uuid::Uuid;
use serde::{ Serialize, Deserialize };

use self::pronouns::PronounSet;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Honorific { Ms, Mx, Mr, Dr, Mrs, Prof, Sir, Dame }
impl Honorific {
    pub fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "Ms" => Some(Self::Ms),
            "Mx" => Some(Self::Mx),
            "Mr" => Some(Self::Mr),
            "Dr" => Some(Self::Dr),
            "Mrs" => Some(Self::Mrs),
            "Prof" => Some(Self::Prof),
            "Sir" => Some(Self::Sir),
            "Dame" => Some(Self::Dame),
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
            Self::Sir => "Sir",
            Self::Dame => "Dame",
        }
    }
    pub fn is_abbreviation(&self) -> bool {
        match self {
            Self::Mx | Self::Ms | Self::Mr | Self::Dr | Self::Mrs | Self::Prof => true,
            Self::Sir | Self::Dame => false,
        }
    }
}
impl Debug for Honorific {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Honorary<{}>", self.str())
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




#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeacherName {
    pub (super) honorific: Honorific,
    pub (super) first: String,
    pub (super) last: String,
    pub (super) middle: Vec<(bool, String)>,
}
impl TeacherName {
    pub fn new(honorific: Honorific, first: String, last: String, middle: Vec<(bool, String)>) -> Self {
        Self { honorific, first, last, middle }
    }

    
    pub fn get_honorific(&self) -> Honorific { self.honorific }
    pub fn get_first(&self) -> &str { &self.first }
    pub fn get_last(&self) -> &str { &self.last }

    pub fn all_middles(&self) -> impl Iterator<Item = (bool, &str)> {
        self.middle.iter().map(|(display, text)| (*display, text.as_str()))
    }
    pub fn visible_middles(&self) -> impl Iterator<Item = &str> {
        self.middle.iter().filter_map(|(display, text)| display.then_some(text.as_str()))
    }

    pub fn short(&self) -> String {
        format!("{} {}", self.honorific, self.last)
    }
    pub fn mid_len(&self) -> String {
        format!("{} {} {}", self.honorific, self.first, self.last)
    }
    pub fn longest(&self) -> String {
        let mut output = format!("{} {} ", self.honorific, self.first);
        for (display, name) in self.middle.iter() {
            if *display {
                output.push_str(name);
                output.push(' ');
            }
        }
        output.push_str(&self.last);
        output
    }
}
impl Debug for TeacherName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {:?} ", self.honorific, self.first)?;
        for (display, name) in self.middle.iter() {
            if *display {
                write!(f, "{:?} ", name)?;
            }
        }
        write!(f, "{}", self.last)?;
        Ok(())
    }
}
impl Display for TeacherName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {:?} ", self.honorific, self.first)?;
        for (display, name) in self.middle.iter() {
            if *display {
                write!(f, "{:?} ", name)?;
            }
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
}

impl Teacher {
    pub fn new(id: Uuid, name: TeacherName, pronouns: PronounSet) -> Self {
        Self { id, name, pronouns }
    }
    pub fn get_id(&self) -> Uuid { self.id }
    pub fn get_name(&self) -> &TeacherName { &self.name }
    pub fn get_pronouns(&self) -> &PronounSet { &self.pronouns }
}
