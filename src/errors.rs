pub enum LogError {
    InvArg(f64),
    InvBase(f64),
    DiffBase(f64, f64),
    NumNegExp(f64),

    Unknown
}

pub enum MatError {
    E1
}

pub enum VecError {
    E1
}