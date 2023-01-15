use crate::errors::NumError;

pub struct Number<T> {
    pub val: T,
    pub exp: Option<T>
}

impl Number<i32> {
    pub fn evaluate(&self) -> Result<i32, NumError<i32>> {
        match self.exp {
            Some(exp) => match exp.signum() {
                0 => Ok(1),
                1 => Ok(self.val),
                -1 => Err(NumError::NegExp(exp)),
                _ => Ok(self.val.pow(exp as u32)) // i32::MIN..=-2_i32 | 2_i32..=i32::MAX
            }

            None => Ok(self.val)
        }
    }
}

impl Number<f64> {
    pub fn evaluate(&self) -> Result<f64, NumError<f64>> {
        match self.exp {
            Some(exp) => Ok(self.val.powf(exp)),
            None => Ok(self.val)
        }
    }
}