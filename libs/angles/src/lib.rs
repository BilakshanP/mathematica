#![allow(dead_code)]

use std::fmt;
use std::f64::consts::PI;

pub enum AngleTypes {
    Deg(f64),
    Rad(f64)
}

pub struct Angle {
    deg: f64,
    rad: f64
}

// getting values
impl Angle {
    pub fn get_deg(&self) -> f64 {
        self.deg
    }

    pub fn get_rad(&self) -> f64 {
        self.rad
    }
}

// initialisation
impl Angle {
    pub fn new(val: AngleTypes) -> Angle {
        match val {
            AngleTypes::Deg(deg) => Angle { deg, rad: deg * PI / 180.0 },
            AngleTypes::Rad(rad) => Angle { deg: rad * 180.0 / PI, rad }
        }
    }

    pub fn new_rad(rad: f64) -> Angle {
        Angle {
            deg: rad * 180.0 /PI,
            rad
        }
    }

    pub fn new_deg(deg: f64) -> Angle {
        Angle {
            deg,
            rad: deg * PI / 180.0
        }
    }
}

// impl for
impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}deg", self.deg)
    }
}

impl fmt::Debug for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}deg {}rad>", self.deg, self.rad)
    }
}