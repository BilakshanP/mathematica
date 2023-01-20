pub enum LogarithimError {
    DifferentBase(f64, f64),
    InvalidArgument(f64),
    InvalidBase(f64),
    NumberNegativeExpopent(f64),

    UnknownError
}

pub enum MatrixError {
    InvalidMatrix,
    InvalidMatrixArray,
    MismatchedSize {
        expected: usize,
        provided: usize
    },
    NonSquareMatrix,
    SingularMatrix,
    NullMatrix,
    NullDeterminant,

    UnknownError
}

pub enum VectorError {

    UnknownError
}