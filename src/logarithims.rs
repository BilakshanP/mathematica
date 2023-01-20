use crate::errors::LogarithimError;

pub struct Log {
    arg: f64,
    base: f64
}

impl Log {
    pub fn evaluate(&self) -> Result<f64, LogarithimError> {
        if self.arg < 0.0 {
            return Err(LogarithimError::InvalidArgument(self.arg));
        }

        if self.base <= 0.0 {
            return Err(LogarithimError::InvalidBase(self.base.signum()));
        }

        Ok(self.arg.log(self.base))
    }

    pub fn add(&self, other: &Self) -> Result<Self, LogarithimError> {
        if self.base != other.base {
            return Err(LogarithimError::DifferentBase(self.base, other.base));
        }

        Ok(Self { arg: self.arg * other.arg, base: self.base })
    }

    pub fn sub(&self, other: &Self) -> Result<Self, LogarithimError> {
        if self.base != other.base {
            return Err(LogarithimError::DifferentBase(self.base, other.base));
        }

        Ok(Self { arg: self.arg / other.arg, base: self.base })
    }
}

impl Log {
    pub fn new(arg: f64, base: f64) -> Self {
        Self { arg, base }
    }
}