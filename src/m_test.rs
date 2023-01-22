pub struct Matrix {
    mat: Vec<Vec<f64>>,
    row: usize,
    col: usize,

    is_square: bool,
    is_identity: bool,

    is_lazy_initialized: bool,

    determinant: Option<f64>,
    transpose: Option<Vec<Vec<f64>>>,

    minors: Option<Vec<Vec<f64>>>,
    cofactors: Option<Vec<Vec<f64>>>,
    adjoint: Option<Vec<Vec<f64>>>,
    inverse: Option<Vec<Vec<f64>>>,

    is_up_to_date: bool
}

//initialisations
impl Matrix {
    pub fn new() -> Self {
        Self {
            mat: Vec::new(),
            row: 0,
            col: 0,

            is_square: false,
            is_identity: false,

            is_lazy_initialized: true,
            
            determinant: None,
            transpose: None,
            minors: None,
            cofactors: None,
            adjoint: None,
            inverse: None,
            
            is_up_to_date: true
        }
    }
}