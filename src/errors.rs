pub enum LogError<T> {
    InvArg(T),
    InvBase(T),
    DiffBase(T, T),
    NumNegExp(T),

    Unknown
}

pub enum MatError {
    E1
}

pub enum VecError {
    E1
}

pub enum NumError<T> {
    NegExp(T)
}