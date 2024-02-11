use std::{collections::HashMap, hash::{Hash, Hasher}};

#[derive(Debug, Clone)]
pub struct ResponseTimeMap {
    counts: HashMap<ResponseTime, u64, ResponseTimeHasherBuilder>,
}

impl ResponseTimeMap {
    pub fn new() -> Self {
        ResponseTimeMap {
            counts: std::collections::HashMap::with_hasher(ResponseTimeHasherBuilder),
        }
    }

    pub fn record_nanos(&mut self, time: u64) {
        let count = self.counts
            .entry(ResponseTime::from_nanos(time))
            .or_default();
        *count += 1;
    }

    pub fn iter(&self) -> impl Iterator<Item = (ResponseTime, u64)> + '_ {
        self.counts.iter().map(|(k, v)| (*k, *v))
    }

    pub fn recorded(&self) -> u64 {
        self.iter().map(|(_, count)| count).sum()
    }

    pub fn clear(&mut self) {
        self.counts.clear();
    }
}


#[derive(Debug, Clone, Copy, Eq)]
// pub struct ResponseTime {
//     multiplier: u8,
//     unit: TimeUnit,
// }
pub struct ResponseTime(u64);

#[allow(dead_code)]
mod consts {
    pub const NANO: u64 = 1;
    pub const MICRO: u64 = NANO * 1000;
    pub const MILLI: u64 = MICRO * 1000;
    pub const SEC: u64 = MILLI * 1000;


    pub const NANO1: u64 = NANO;
    pub const NANO10: u64 = NANO * 10;
    pub const NANO100: u64 = NANO * 100;

    pub const MICRO1: u64 = MICRO;
    pub const MICRO10: u64 = MICRO * 10;
    pub const MICRO100: u64 = MICRO * 100;

    pub const MILLI1: u64 = MILLI;
    pub const MILLI10: u64 = MILLI * 10;
    pub const MILLI100: u64 = MILLI * 100;

    pub const SEC1: u64 = SEC;
    pub const SEC10: u64 = SEC * 10;
    pub const SEC100: u64 = SEC * 100;
}

// TODO: Figure out this whole mess
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Time1Nanos,
    Time100Nanos,
    Time10Micros,
    Time1Millis,
    Time100Millis,
    Time10Secs,
}

// TODO: Figure out this whole mess
#[allow(dead_code)]
impl TimeUnit {
    pub fn as_nanos(&self) -> u64 {
        use TimeUnit::*;
        use consts::*;
        match self {
            Time1Nanos    => NANO1,
            Time100Nanos  => NANO100,
            Time10Micros  => MICRO10,
            Time1Millis   => MILLI1,
            Time100Millis => MILLI100,
            Time10Secs    => SEC10,
        }
    }

    pub fn best_unit_for(time_in_nanos: u64) -> Self {
        use TimeUnit::*;
        use consts::*;

        const NANO100M1: u64 = NANO100 - 1;
        const MICRO10M1: u64 = MICRO10 - 1;
        const MILLI1M1: u64 = MILLI1 - 1;
        const MILLI100M1: u64 = MILLI100 - 1;
        const SEC10M1: u64 = SEC10 - 1;

        match time_in_nanos {
            0..=NANO100M1 => Time1Nanos,
            NANO100..=MICRO10M1 => Time100Nanos,
            MICRO10..=MILLI1M1 => Time10Micros,
            MILLI1..=MILLI100M1 => Time1Millis,
            MILLI100..=SEC10M1 => Time100Millis,
            _ => Time10Secs,
        }
    }
}

// impl ResponseTime {
//     pub fn as_nanos(&self) -> u64 {
//         self.multiplier as u64 * self.unit.as_nanos()
//     }

//     pub fn from_nanos(time_in_nanos: u64) -> Self {
//         let unit = TimeUnit::best_unit_for(time_in_nanos);
//         let multiplier = time_in_nanos / unit.as_nanos();
//         ResponseTime { multiplier: multiplier.min(255) as u8, unit }
//     }
// }
impl ResponseTime {
    pub fn as_nanos(&self) -> u64 {
        self.0
    }

    pub fn from_nanos(time_in_nanos: u64) -> Self {
        // Round to a microsecond 
        // TODO: Maybe make this a little better
        Self(time_in_nanos / 1000 * 1000)
    }
}

impl PartialEq for ResponseTime {
    fn eq(&self, other: &Self) -> bool {
        self.as_nanos() == other.as_nanos()
    }
}

impl PartialOrd for ResponseTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ResponseTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_nanos().cmp(&other.as_nanos())
    }
}

impl Hash for ResponseTime {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.as_nanos());
    }
}





#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResponseTimeHasher(u64);
impl Hasher for ResponseTimeHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_u64(byte as u64);
        }
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = self.0
            .wrapping_mul(u32::MAX as u64) // Little prime
            .wrapping_add(i)
            .rem_euclid(u64::MAX - 58); // Big prime
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResponseTimeHasherBuilder;

impl std::hash::BuildHasher for ResponseTimeHasherBuilder {
    type Hasher = ResponseTimeHasher;

    fn build_hasher(&self) -> Self::Hasher {
        ResponseTimeHasher(0)
    }
}
