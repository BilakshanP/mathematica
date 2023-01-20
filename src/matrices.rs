#![allow(dead_code)]

use std::fmt;

use crate::errors::MatrixError;

// use self::internals;

pub struct Matrix {
    mat: Vec<Vec<f64>>,
    row: usize,
    col: usize,
    issquare: bool
}

//initialisations
impl Matrix {
    pub fn new() -> Self {
        Self { mat: Vec::new(), row: 0, col: 0, issquare: true }
    }

    pub fn new_from_vec_sized(vec: Vec<f64>, row: usize, col: usize) -> Result<Self, MatrixError> {
        let (expected, provided) = (vec.len(), row * col);

        if expected == provided {
            return Ok(Self { mat: internals::craft_new(vec, col), row, col, issquare: row == col })
        }

        Err(MatrixError::MismatchedSize { expected, provided })
    }

    pub fn new_from_vec_unsized(vec: Vec<Vec<f64>>) -> Matrix {
        let (row, col) = (vec.len(), vec[0].len());

        Self { mat: vec, row, col, issquare: row == col }
    }

    pub fn new_identity_matrix(num: usize) -> Self {
        let mut out_vec: Vec<Vec<f64>> = Vec::new();

        for i in 0..num {
            let mut tmp_vec: Vec<f64> = Vec::new();

            for j in 0..num {
                if i == j {
                    tmp_vec.push(1.0);

                    continue
                } 

                else {
                    tmp_vec.push(0.0)
                }
            }

            out_vec.push(tmp_vec)
        }

        Self { mat: out_vec, row: num, col: num, issquare: true }
    }
}

//getting singular values
impl Matrix {
    pub fn matrix(&self) -> &Vec<Vec<f64>> {
        &self.mat
    }

    pub fn order(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    pub fn determinant(&self) -> Result<f64, MatrixError> {
        if !self.issquare {
            return Err(MatrixError::NonSquareMatrix);
        }

        let order = self.order();

        return match order.0 {
            1 => return Ok(self.mat[0][0]),
            2 => return Ok(internals::det2(&self.mat)),
            _ => {
                let mat = &self.mat;

                let reduced: Vec<Vec<Vec<f64>>> = internals::inner_reduce(mat);
                let falttened: Vec<f64> = internals::inner_flatten(mat);
                    
                let mut tmp_vec: Vec<f64> = Vec::new();
                    
                for i in 0..mat.len() {
                    tmp_vec.push(falttened[i] * internals::inner_determinant(&reduced[i]));
                }
                    
                Ok(tmp_vec.iter().sum())
            }
        }
    }

    pub fn transpose(&self) -> Result<Self, MatrixError> {
        if self.row == 0 {
            return Err(MatrixError::NullMatrix);
        }

        let mat = &self.mat;
        let (row, col) = (self.row, self.col);
        let mut out_mat: Vec<Vec<f64>> = Vec::new();

        for i in 0..col {
            let mut tmp_mat: Vec<f64> = Vec::new();
    
            for j in mat.iter() {
                tmp_mat.push(j[i]);
            }
    
            out_mat.push(tmp_mat);
        }
    
        Ok(Self { mat: out_mat, row: col, col: row, issquare: self.issquare })
    }

    pub fn minors(&self) -> Result<Self, MatrixError> {
        if !self.issquare {
            return Err(MatrixError::NonSquareMatrix);
        }

        let mat = &self.mat;
        let mut out_mat: Vec<Vec<f64>> = Vec::new();
    
        for i in 0..self.row {
            let mut tmp_mat: Vec<f64> = Vec::new();
    
            for j in 0..self.col {
                tmp_mat.push(internals::determinant(&internals::trim_around(&mat, i, j)));
            }
    
            out_mat.push(tmp_mat);
        }
    
        Ok(Self { mat: out_mat, row: self.row, col: self.col, issquare: true })
    }

    pub fn cofactors(&self) -> Result<Self, MatrixError> {
        if !self.issquare {
            return Err(MatrixError::NonSquareMatrix);
        }

        let mat = &self.mat;
        let mut out_mat: Vec<Vec<f64>> = Vec::new();
    
        for (i1, j1) in internals::minors(mat).iter().enumerate() {
            let mut tmp_mat: Vec<f64> = Vec::new();
    
            for (i2, j2) in j1.iter().enumerate() {
                tmp_mat.push(j2 * (-1.0_f64).powi((i1 + i2) as i32));
            }
    
            out_mat.push(tmp_mat);
        }
    
        Ok(Self { mat:  out_mat, row: self.row, col: self.col, issquare: true })
    }

    pub fn adjoint(&self) -> Result<Self, MatrixError> {
        if !self.issquare {
            return Err(MatrixError::NonSquareMatrix);
        }

        Ok(Self { mat: internals::transpose(&internals::cofactors(&self.mat)), row: self.col, col: self.row, issquare: true })
    }

    #[allow(illegal_floating_point_literal_pattern)]
    pub fn inverse(&self) -> Result<Self, MatrixError> {
        return match self.determinant() {
            Ok(det) => match det {
                    0.0 => Err(MatrixError::SingularMatrix),
                    det => Ok(Self { mat: internals::scalar_mul(&self.mat, det), row: self.row, col: self.col, issquare: true })
                },
            Err(error) => match error {
                    MatrixError::NonSquareMatrix => Err(MatrixError::NonSquareMatrix),
                    _ => Err(MatrixError::UnknownError)
                }
            }
        }
}

//singular boolean results
impl Matrix {
    // pub fn is_square_matrix(&self) -> bool {
    //     self.issquare == (self.row == self.col)
    // }

    // pub fn is_square_matrix_deep(&self) -> bool {
    //     if self.is_valid_matrix() {
    //         true
    //     }
    // }

    fn is_identity_matrix(&self) -> bool {
        if !self.issquare {
            return false;
        }

        let mat = &self.mat;

        for (i1, j1) in mat.iter().enumerate() {
            for (i2, j2) in j1.iter().enumerate() {
                if ((i1 != i2) && (*j2 == 0.0)) || (*j2 == 1.0) {
                } else {
                    return false;
                }
            }
        }
    
        true
    }

    fn is_invertible_matrix(&self) -> Result<f64, MatrixError> {
        return match self.determinant() {
            Ok(det) => if det != 0.0 {
                                Ok(det)
                            } else {
                                Err(MatrixError::NullDeterminant)
                            },
            Err(error) => Err(error)
        }
    }

    fn is_valid_matrix(&self) -> bool {
        let col_size = self.mat[0].len();

        for i in self.mat.iter() {
            if col_size == i.len() {
                continue
            }

            return false;
        }

        true
    }
}

//composite boolean results
impl Matrix {

}

//sub-trait implementations


//trait implementations
impl Clone for Matrix {
    fn clone(&self) -> Self {
        Self::new_from_vec_unsized(self.mat.clone())
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", internals::matrix_print(&self.mat))
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", internals::matrix_print_debug(&self.mat))
    }
}

pub mod external {

}

mod internals {
    pub fn craft_new(vec: Vec<f64>, col: usize) -> Vec<Vec<f64>> {
        vec.chunks_exact(col).map(|x| Vec::from(x)).collect()
    }

    pub fn determinant(matrix: &[Vec<f64>]) -> f64 {
        let order: (usize, usize) = get_order(matrix);

        if order.0 == 2 {
            return det2(matrix);
        }
    
        let reduced: Vec<Vec<Vec<f64>>> = inner_reduce(matrix);
        let falttened: Vec<f64> = flatten(matrix);
    
        let mut tmp_vec: Vec<f64> = Vec::new();
    
        for i in 0..order.0 {
            tmp_vec.push(falttened[i] * inner_determinant(&reduced[i]));
        }
    
        return tmp_vec.iter().sum();
    }

    pub fn det2(matrix: &[Vec<f64>]) -> f64 {
        matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
    }

    pub fn minors(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let order: (usize, usize) = get_order(matrix);
        let mut out_mat: Vec<Vec<f64>> = Vec::new();
    
        for i in 0..order.0 {
            let mut tmp_mat: Vec<f64> = Vec::new();
    
            for j in 0..order.1 {
                tmp_mat.push(determinant(&trim_around(matrix, i, j)));
            }
    
            out_mat.push(tmp_mat);
        }
    
        out_mat
    }

    pub fn matrix_print(matrix: &[Vec<f64>]) -> String {
        let mut printable = String::new();
    
        for i in matrix.iter(){
            printable.push_str("( ");
    
            for j in i.iter(){
                printable += &format!("{} ", j);
            }
    
            printable.push_str(")\n");
        }
    
        printable
    }
    
    pub fn matrix_print_debug(matrix: &[Vec<f64>]) -> String {
        let mut printable = String::new();
    
        for i in matrix.iter(){
            printable.push_str("( ");
    
            for j in i.iter(){
                printable += &format!("{} ", j.round());
            }
    
            printable.push_str(")\n");
        }
    
        printable
    }

    pub fn trim_around(matrix: &[Vec<f64>], row: usize, col: usize) -> Vec<Vec<f64>> {
        let mut out_mat: Vec<Vec<f64>> = Vec::new();
    
        for (i1, j1) in matrix.iter().enumerate(){
            if i1 == row {
                continue;
            }
    
            let mut tmp_mat: Vec<f64> = Vec::new();
                for (i2, j2) in j1.iter().enumerate(){
                    if i2 == col {
                        continue;
                    }
    
                    tmp_mat.push(*j2);
                }
    
                out_mat.push(tmp_mat);
        }
    
        out_mat
    }
    
    fn is_valid_matrix_array(matrix: &[Vec<f64>]) -> bool {
        if matrix.len() == 0 {
            return false;
        }

        let col_size = matrix[0].len();

        if col_size > 0 {
            for i in matrix.iter() {
                if col_size == i.len() {
                    continue
                }

                return false;
            }

            return true;
        }

        false
    }

    pub fn cofactors(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let mut out_mat: Vec<Vec<f64>> = Vec::new();
    
        for (i1, j1) in minors(matrix).iter().enumerate() {
            let mut tmp_mat: Vec<f64> = Vec::new();
    
            for (i2, j2) in j1.iter().enumerate() {
                tmp_mat.push(j2 * (-1.0_f64).powi((i1 + i2) as i32));
            }
    
            out_mat.push(tmp_mat);
        }
    
        out_mat
    }

    pub fn transpose(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let mut out_mat: Vec<Vec<f64>> = Vec::new();
    
        for i in 0..matrix[0].len(){
            let mut tmp_mat: Vec<f64> = Vec::new();
    
            for j in matrix.iter(){
                tmp_mat.push(j[i]);
            }
    
            out_mat.push(tmp_mat);
        }
    
        out_mat
    }
    
    pub fn scalar_mul(matrix: &[Vec<f64>], scalar: f64) -> Vec<Vec<f64>> {
        let mut out_mat: Vec<Vec<f64>> = Vec::new();
    
        for i in 0..matrix[0].len(){
            let mut tmp_mat: Vec<f64> = Vec::new();
    
            for j in 0..matrix.len(){
                tmp_mat.push(matrix[i][j] * scalar);
            }
    
            out_mat.push(tmp_mat);
        }
    
        out_mat
    }

    fn get_order(matrix: &[Vec<f64>]) -> (usize, usize) {
        (matrix.len(), matrix[0].len())
    }

    pub fn inner_reduce(matrix: &[Vec<f64>]) -> Vec<Vec<Vec<f64>>>{
        let mut out_mat: Vec<Vec<Vec<f64>>> = Vec::new();
    
        for i in 0..matrix[0].len() {
            out_mat.push(trim_around(matrix, 0, i))
        }
    
        out_mat
    }

    pub fn inner_flatten(matrix: &[Vec<f64>]) -> Vec<f64> {
        let mut out_vec: Vec<f64> = Vec::new();
        let mut times: f64 = 1.0;
    
        for i in matrix[0].iter(){
            out_vec.push(*i* times);
    
            times *= -1.0;
        }
    
        out_vec
    }

    pub fn inner_determinant(matrix: &[Vec<f64>]) -> f64 {
        let order: (usize, usize) = get_order(matrix);
    
        if order.0 == 2 {
            return det2(matrix);
        }
    
        let reduced: Vec<Vec<Vec<f64>>> = inner_reduce(matrix);
        let falttened: Vec<f64> = flatten(matrix);
    
        let mut tmp_vec: Vec<f64> = Vec::new();
    
        for i in 0..order.0 {
            tmp_vec.push(falttened[i] * inner_determinant(&reduced[i]));
        }
    
        return tmp_vec.iter().sum();
    }

    fn flatten(matrix: &[Vec<f64>]) -> Vec<f64> {
        let mut out_vec: Vec<f64> = Vec::new();
        let mut times: f64 = 1.0;
    
        for i in matrix.iter(){
            for j in i.iter(){
                out_vec.push(*j * times);
    
                times *= -1.0;
            }
        }
    
        out_vec
    }
}