#![allow(dead_code)]

use std::fmt;

pub struct Matrix {
    mat: Vec<Vec<f64>>,
    row: usize,
    col: usize
}

// initialisation
impl Matrix {
    pub fn new() -> Matrix {
        Matrix { mat: Vec::new(), row: 0, col: 0 }
    }

    pub fn new_from_vec_sized(vec: Vec<f64>, row: usize, col: usize) -> Option<Matrix> {
        if vec.len() != row * col {
            return None;
        }

        let mut out_vec: Vec<Vec<f64>> = Vec::new();
        let mut tmp_vec: Vec<f64> = Vec::new();

        for i in 1..(row * col + 1) {
            tmp_vec.push(vec[i - 1]);

            if i % col == 0 {
                out_vec.push(tmp_vec);
                tmp_vec = Vec::new();
            }
        }

        Some(Matrix { mat: out_vec, row, col })
    }

    pub fn new_from_vec_unsized(vec: Vec<Vec<f64>>) -> Matrix {
        let (row, col) = (vec.len(), vec[0].len());
        Matrix { mat: vec, row, col }
    }

    pub fn new_identity_matrix(num: usize) -> Matrix {
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

        Matrix { mat: out_vec, row: num, col: num }
    }
}

// getting values
impl Matrix {
    pub fn get(&self) -> &Vec<Vec<f64>> {
        &self.mat
    }

    pub fn order(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    pub fn determinant(&self) -> f64 {
        determinant(&self.mat)
    }

    pub fn transpose(&self) -> Matrix {
        Matrix::new_from_vec_unsized(transpose(&self.mat))
    }

    pub fn minors(&self) -> Matrix {
        Matrix::new_from_vec_unsized(get_minors(&self.mat))
    }

    pub fn cofactors(&self) -> Matrix {
        Matrix::new_from_vec_unsized(get_cofactors(&self.mat))
    }

    pub fn adjoint(&self) -> Matrix {
        Matrix::new_from_vec_unsized(get_adjoint(&self.mat))
    }

    pub fn inverse(&self) -> Option<Matrix> {
        let det = self.determinant();

        if det == 0.0 {
            return None
        }

        Some(Matrix::new_from_vec_unsized(get_inverse_from_result(&self.mat, &det)))
    }

    // fn raise(&self, num: i32) -> Option<Matrix> {
    //     if num 
    //     let mut out_mat = matrix.clone();
    // 
    //     for _ in 0..(n - 1) {
    //         out_mat = out_mat.cross(&matrix).unwrap()
    //     }
    // 
    //     out_mat
    // }
}

// getting compound values
impl Matrix {
    
}

// operations
impl Matrix {
    pub fn cross(&self, other: &Self) -> Option<Matrix> {
        if self.row == other.col {
            return Some(Matrix::new_from_vec_unsized(matrix_mul(&self.mat, &other.mat)))
        }

        None
    }

    pub fn times(&self, other: f64) -> Matrix {
        Matrix::new_from_vec_unsized(scalar_mul(&self.mat, other))
    }

    pub fn add(&self, other: &Self) -> Option<Matrix> {
        if self.order() == other.order() {
            return Some(Matrix::new_from_vec_unsized(add_mat(&self.mat, &other.mat)))
        }

        None
    }

    pub fn sub(&self, other: &Self) -> Option<Matrix> {
        if self.order() == other.order() {
            return Some(Matrix::new_from_vec_unsized(sub_mat(&self.mat, &other.mat)))
        }

        None
    }
}

// mutating operations
impl Matrix {
    pub fn update(&mut self, val: f64, row: usize, col: usize) -> bool {
        if self.row >= row && self.col >= col {
            self.mat[row][col] = val;

            return true
        }

        false
    }

    pub fn push_row(&mut self, val: Vec<f64>) -> bool {
        if self.col == val.len() {
            self.mat.push(val);

            return true
        }

        false
    }

    pub fn push_col(&mut self, val: Vec<f64>) -> bool {
        if self.row == val.len() {
            for (i, j) in val.iter().enumerate() {
                self.mat[i].push(*j);
            }

            return true
        }

        false
    }

    pub fn round(&mut self) {
        for (i1, j1) in self.clone().mat.iter().enumerate() {
            for (i2, j2) in j1.iter().enumerate() {
                self.mat[i1][i2] = j2.round()
            }
        }
    }
}

// checks
impl Matrix {
    pub fn is_valid_matrix(&self) -> bool {
        is_valid_matrix(&self.mat)
    }

    pub fn is_square_matrix(&self) -> bool {
        self.row == self.col
    }

    pub fn is_identity_matrix(&self) -> bool {
        is_identity(&self.mat)
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        Self::new_from_vec_unsized(self.mat.clone())
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", matrix_print(&self.mat))
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", matrix_print_debug(&self.mat))
    }
}

//////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////

fn get_order(matrix: &[Vec<f64>]) -> (usize, usize) {
    (matrix.len(), matrix[0].len())
}

fn determinant(matrix: &[Vec<f64>]) -> f64 {
    let order: (usize, usize) = get_order(matrix);

    if order.0 != order.1 {
        return 0.0;
    }

    if order.0 == 1 {
        return matrix[0][0];
    }

    if order.0 == 2 {
        return det2(matrix);
    }

    let reduced: Vec<Vec<Vec<f64>>> = inner_reduce(matrix);
    let falttened: Vec<f64> = inner_flatten(matrix);

    let mut tmp_vec: Vec<f64> = Vec::new();

    for i in 0..order.0 {
        tmp_vec.push(falttened[i] * inner_determinant(&reduced[i]));
    }

    return tmp_vec.iter().sum();
}

fn matrix_mul(matrix_a: &[Vec<f64>], matrix_b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let order_b: (usize, usize) = get_order(matrix_b);

    let mut out_mat: Vec<Vec<f64>> = Vec::new();

    for i in matrix_a.iter() {
        let mut tmp_mat: Vec<f64> = Vec::new();

        for j in 0..order_b.1 {
            let mut sum_mat: Vec<f64> = Vec::new();

            for k in 0..order_b.0 {
                sum_mat.push(i[k] * matrix_b[k][j]);
            }

            tmp_mat.push(sum_mat.iter().sum());
        }

        out_mat.push(tmp_mat);
    }

    out_mat
}

fn scalar_mul(matrix: &[Vec<f64>], scalar: f64) -> Vec<Vec<f64>> {
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

fn scalar_div(matrix: &[Vec<f64>], scalar: &f64) -> Vec<Vec<f64>> {
    let scalar: f64 = 1.0 / scalar;

    scalar_mul(matrix, scalar)
}

fn add_mat(matrix_a: &[Vec<f64>], matrix_b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut out_mat: Vec<Vec<f64>> = Vec::new();

    for (i1, i2) in matrix_a.iter().enumerate(){
        let mut tmp_mat: Vec<f64> = Vec::new();

        for (j1, j2) in i2.iter().enumerate() {
            tmp_mat.push(j2 + matrix_b[i1][j1])
        }

        out_mat.push(tmp_mat)
    }

    out_mat
}

fn sub_mat(matrix_a: &[Vec<f64>], matrix_b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut out_mat: Vec<Vec<f64>> = Vec::new();

    for (i1, i2) in matrix_a.iter().enumerate(){
        let mut tmp_mat: Vec<f64> = Vec::new();

        for (j1, j2) in i2.iter().enumerate() {
            tmp_mat.push(j2 - matrix_b[i1][j1])
        }

        out_mat.push(tmp_mat)
    }

    out_mat
}

fn transpose(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
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

fn get_minors(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
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

fn get_cofactors(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut out_mat: Vec<Vec<f64>> = Vec::new();

    for (i1, j1) in get_minors(matrix).iter().enumerate() {
        let mut tmp_mat: Vec<f64> = Vec::new();

        for (i2, j2) in j1.iter().enumerate() {
            tmp_mat.push(j2 * (-1.0_f64).powi((i1 + i2) as i32));
        }

        out_mat.push(tmp_mat);
    }

    out_mat
}

fn get_adjoint(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    transpose(&get_cofactors(matrix))
}

fn get_inverse(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    scalar_div(&get_cofactors(matrix), &determinant(matrix))
}

fn get_inverse_from_result(matrix: &[Vec<f64>], determinant: &f64) -> Vec<Vec<f64>> {
    scalar_div(&get_cofactors(matrix), determinant)
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

fn matrix_print(matrix: &[Vec<f64>]) -> String {
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

fn matrix_print_debug(matrix: &[Vec<f64>]) -> String {
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

fn det2(matrix: &[Vec<f64>]) -> f64 {
    matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
}

fn trim_around(matrix: &[Vec<f64>], row: usize, col: usize) -> Vec<Vec<f64>> {
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

fn reduce(matrix: &[Vec<f64>]) -> Vec<Vec<Vec<f64>>> {
    let mut out_mat: Vec<Vec<Vec<f64>>> = Vec::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            out_mat.push(trim_around(matrix, i, j))
        }
    }

    out_mat
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

fn get_semi_minors(matrix: &[Vec<f64>]) -> Vec<Vec<Vec<Vec<f64>>>> {
    let order: (usize, usize) = get_order(matrix);
    let mut out_mat: Vec<Vec<Vec<Vec<f64>>>> = Vec::new();

    for i in 0..order.0 {
        let mut tmp_mat: Vec<Vec<Vec<f64>>> = Vec::new();

        for j in 0..order.1 {
            tmp_mat.push(trim_around(matrix, i, j));
        }

        out_mat.push(tmp_mat);
    }

    out_mat
}

fn get_cofactors_from_minors(minors: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut out_mat: Vec<Vec<f64>> = Vec::new();

    for (i1, j1) in minors.iter().enumerate() {
        let mut tmp_mat: Vec<f64> = Vec::new();

        for (i2, j2) in j1.iter().enumerate() {
            tmp_mat.push(j2 * (-1.0_f64).powi((i1 + i2) as i32));
        }

        out_mat.push(tmp_mat);
    }

    out_mat
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn are_multiplyable(matrix_a: &[Vec<f64>], matrix_b: &[Vec<f64>]) -> bool {
    get_order(matrix_a).1 == get_order(matrix_b).0
}

fn are_add_sub_able(matrix_a: &[Vec<f64>], matrix_b: &[Vec<f64>]) -> bool {
    get_order(matrix_a) == get_order(matrix_b)
}

fn is_valid_matrix(matrix: &[Vec<f64>]) -> bool {
    let order: (usize, usize) = get_order(matrix);

    for i in matrix.iter() {
        if order.1 != i.len() {
            return false;
        }
    }

    true
}

fn is_square_matrix(matrix: &[Vec<f64>]) -> bool {
    matrix.len() == matrix[0].len()
}

fn is_invertible(matrix: &[Vec<f64>]) -> bool {
    if is_square_matrix(matrix) {
        return !(determinant(matrix) == 0.0);
    }

    false
}

fn is_identity(matrix: &[Vec<f64>]) -> bool {
    for (i1, j1) in matrix.iter().enumerate() {
        for (i2, j2) in j1.iter().enumerate() {
            if ((i1 != i2) && (*j2 == 0.0)) || (*j2 == 1.0) {
            } else {
                return false;
            }
        }
    }

    true
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn is_invertible_result(matrix: &[Vec<f64>]) -> (bool, f64) {
    let det: f64 = determinant(matrix);

    if is_square_matrix(matrix) {
        return (!(det == 0.0), det);
    }

    (false, 0.0)
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn determinant_indexed(matrix: &[Vec<f64>], row: usize) -> f64 {
    let order: (usize, usize) = get_order(matrix);

    if row >= order.0 {
        return 0.0;
    }

    if order.0 != order.1 {
        return 0.0;
    }

    if order.0 == 1 {
        return matrix[0][0];
    }

    if order.0 == 2 {
        return det2(matrix);
    }

    let reduced: Vec<Vec<Vec<f64>>> = inner_reduce_indexed(matrix, row);
    let falttened: Vec<f64> = inner_flatten_indexed(matrix, row);

    let mut tmp_vec: Vec<f64> = Vec::new();

    for i in 0..order.0 {
        tmp_vec.push(falttened[i] * inner_determinant_indexed(&reduced[i], row));
    }

    return tmp_vec.iter().sum::<f64>() * (-1.0_f64).powi(row as i32);
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn inner_reduce(matrix: &[Vec<f64>]) -> Vec<Vec<Vec<f64>>>{
    let mut out_mat: Vec<Vec<Vec<f64>>> = Vec::new();

    for i in 0..matrix[0].len() {
        out_mat.push(trim_around(matrix, 0, i))
    }

    out_mat
}

fn inner_flatten(matrix: &[Vec<f64>]) -> Vec<f64> {
    let mut out_vec: Vec<f64> = Vec::new();
    let mut times: f64 = 1.0;

    for i in matrix[0].iter(){
        out_vec.push(*i* times);

        times *= -1.0;
    }

    out_vec
}

fn inner_determinant(matrix: &[Vec<f64>]) -> f64 {
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

fn inner_flatten_indexed(matrix: &[Vec<f64>], row: usize) -> Vec<f64> {
    let mut out_vec: Vec<f64> = Vec::new();
    let mut times: f64 = 1.0;

    for i in matrix[row].iter(){
        out_vec.push(*i* times);

        times *= -1.0;
    }

    out_vec
}

fn inner_reduce_indexed(matrix: &[Vec<f64>], row: usize) -> Vec<Vec<Vec<f64>>>{
    let mut out_mat: Vec<Vec<Vec<f64>>> = Vec::new();

    for i in 0..matrix[row].len() {
        out_mat.push(trim_around(matrix, row, i))
    }

    out_mat
}

fn inner_determinant_indexed(matrix: &[Vec<f64>], row: usize) -> f64 {
    let order: (usize, usize) = get_order(matrix);

    if order.0 == 2 {
        return det2(matrix);
    }

    let reduced: Vec<Vec<Vec<f64>>> = inner_reduce_indexed(matrix, row);
    let falttened: Vec<f64> = inner_flatten_indexed(matrix, row);

    let mut tmp_vec: Vec<f64> = Vec::new();

    for i in 0..order.0 {
        tmp_vec.push(falttened[i] * inner_determinant_indexed(&reduced[i], row));
    }

    return tmp_vec.iter().sum();
}
