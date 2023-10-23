use nih_plug::prelude::Enum;

#[derive(Copy, Clone, PartialEq)]
pub enum DelayTime {
    _1_32,
    _1_16,
    _1_8,
    _1_4,
    _1_2,
    _1,
    _2,
    _4,
}

impl Enum for DelayTime {
    fn variants() -> &'static [&'static str] {
        &["1/32", "1/16", "1/8", "1/4", "1/2", "1", "2", "4"]
    }

    fn ids() -> Option<&'static [&'static str]> {
        Some(&[
            "1/32",
            "1/16",
            "1/8",
            "1/4",
            "1/2",
            "1",
            "2",
            "4",
        ])
    }

    fn to_index(self) -> usize {
        match self {
            DelayTime::_1_32 => 0,
            DelayTime::_1_16 => 1,
            DelayTime::_1_8 => 2,
            DelayTime::_1_4 => 3,
            DelayTime::_1_2 => 4,
            DelayTime::_1 => 5,
            DelayTime::_2 => 6,
            DelayTime::_4 => 7,
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => DelayTime::_1_32,
            1 => DelayTime::_1_16,
            2 => DelayTime::_1_8,
            3 => DelayTime::_1_4,
            4 => DelayTime::_1_2,
            5 => DelayTime::_1,
            6 => DelayTime::_2,
            7 => DelayTime::_4,
            _ => DelayTime::_1_4,
        }
    }
}

impl DelayTime {
    pub fn get_delay_time_in_bars(&self) -> f32 {
        match self {
            DelayTime::_1_32 => 1.0 / 32.0,
            DelayTime::_1_16 => 1.0 / 16.0,
            DelayTime::_1_8 => 1.0 / 8.0,
            DelayTime::_1_4 => 1.0 / 4.0,
            DelayTime::_1_2 => 1.0 / 2.0,
            DelayTime::_1 => 1.0,
            DelayTime::_2 => 2.0,
            DelayTime::_4 => 4.0,
        }
    }

    pub fn get_delay_time_in_beats(&self) -> f32 {
        match self {
            DelayTime::_1_32 => 1.0 / 8.0,
            DelayTime::_1_16 => 1.0 / 4.0,
            DelayTime::_1_8 => 1.0 / 2.0,
            DelayTime::_1_4 => 1.0,
            DelayTime::_1_2 => 2.0,
            DelayTime::_1 => 4.0,
            DelayTime::_2 => 8.0,
            DelayTime::_4 => 16.0,
        }
    }

    pub fn get_max_in_beats() -> f32 {
        16.0
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum DelayTiming {
    Dotted,
    Triplet,
    Straight,
}

impl Enum for DelayTiming {
    fn variants() -> &'static [&'static str] {
        &["Dotted", "Triplet", "Straight"]
    }

    fn ids() -> Option<&'static [&'static str]> {
        Some(&["dotted", "triplet", "straight"])
    }

    fn to_index(self) -> usize {
        match self {
            DelayTiming::Dotted => 0,
            DelayTiming::Triplet => 1,
            DelayTiming::Straight => 2,
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => DelayTiming::Dotted,
            1 => DelayTiming::Triplet,
            2 => DelayTiming::Straight,
            _ => DelayTiming::Straight,
        }
    }
}