use crate::errors::{ LogError, NumError };
use crate::numbers::Number as Num;

pub struct Log<T> {
    arg: Num<T>,
    base: Num<T>
}

// impl Log<f64> {
//     pub fn get_val(&self) -> Result<f64, LogError> {
//         if self.arg < 0.0 {
//             return Err(LogError::InvArg(self.arg));
//         }
// 
//         if self.base <= 0.0 {
//             return Err(LogError::InvBase(self.base.signum()));
//         }
// 
//         Ok(self.arg.log(self.base))
//     }
// 
//     pub fn add(&self, other: &Self) -> Result<Self, LogError> {
//         if self.base != other.base {
//             return Err(LogError::DiffBase(self.base, other.base));
//         }
// 
//         Ok(Self { arg: self.arg * other.arg, base: self.base })
//     }
// 
//     pub fn sub(&self, other: &Self) -> Result<Self, LogError> {
//         if self.base != other.base {
//             return Err(LogError::DiffBase(self.base, other.base));
//         }
// 
//         Ok(Self { arg: self.arg / other.arg, base: self.base })
//     }
// 
//     //pub fn simplify(&self) -> (f64, Self<f64>) {
//     //
//     //}
// }

impl Log<f64> {
    pub fn get_val(&self) -> Result<f64, LogError<f64>> {
        match self.arg.evaluate() {
            Ok(arg) => {
                if arg < 0.0 {
                    return Err(LogError::InvArg(arg));
                };

                match self.base.evaluate() {
                    Ok(base) => if base <= 0.0 {
                            return Err(LogError::InvBase(base.signum()));
                        } else {
                            return Ok(arg.log(base));
                      },
                    
                    Err(err) => match err {
                        NumError::NegExp(val) => return Err(LogError::NumNegExp(val as f64))
                    }
                }
            },

            Err(_) => return Err(LogError::InvBase(9.0))
        }
    }
}