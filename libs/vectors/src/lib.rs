#![allow(dead_code)]

use std::fmt;

use angles::Angle;

pub struct Vector {
    x: f64,
    y: f64,
    z: f64
}

// operations
impl Vector {
    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn times(&self, n: f64) -> Vector {
        Vector {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n
        }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

// getting values
impl Vector {
    pub fn get_comp(&self, n: usize) -> Option<f64> {
        match n {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            _ => None
        }
    }

    pub fn get_all(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn mag_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn pow2(&self) -> f64 {
        self.mag_sq()
    }

    pub fn unit(&self) -> Vector {
        self.times(1.0/&self.magnitude())
    }

    // pub fn unit_alt(&self) -> Vector {
    //     self.times((1.0/&self.mag_sq()).sqrt())
    // }

    pub fn on(&self, other: &Vector) -> f64 {
        self.dot(&other.unit())
    }

    pub fn angle(&self, other: &Vector) -> Angle {
        Angle::new_rad((self.dot(other) / (self.mag_sq() * other.mag_sq()).sqrt()).acos())
    }

    // pub fn angle_alt(&self, other: &Vector) -> Angle {
    //     Angle::new_rad(self.unit_alt().dot(&other.unit_alt()).sqrt().acos())
    // }
}

// initialisation
impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            x, y, z
        }
    }

    pub fn new_from_array(coords: [f64; 3]) -> Vector {
        Vector {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

// impl for
impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self.x, self.y, self.z))
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", print(self.x, self.y, self.z))
    }
}

fn print(x: f64, y: f64, z: f64) -> String {
    format!("( {} {} {} )", x, y, z)
}

fn pretty_print(x: f64, y: f64, z: f64) -> String {
    if [x, y, z] == [0.0; 3] {
        return "0.0i+0.0j+0.0k".to_string();
    }

    let fx: String = {
        if x == 1.0 {
            "i".to_string()
        } else if x == -1.0 {
            "-i".to_string()
        } else if x > 0.0 {
            format!("{}i", x)
        } else if x < 0.0 {
            format!("{:+}i", x)
        } else {
            "".to_string()
        }
    };

    let fy: String = {
        if y == 1.0 {
            if x != 0.0 {
                "+j".to_string()
            } else {
                "j".to_string()
            }
        } else if y == -1.0 {
            "-j".to_string()             
        } else if y == 0.0 {
            "".to_string()
        } else {
            format!("{:+}j", y)
        }
    };

    let fz: String = {
        if z == 1.0 {
            if (x != 0.0) | (y != 0.0) {
                "+k".to_string()
            } else {
                "k".to_string()
            }
        } else if z == -1.0 {
            "-k".to_string()             
        } else if z == 0.0 {
            "".to_string()
        } else {
            format!("{:+}k", z)
        }
    };

    fx + &fy + &fz
}