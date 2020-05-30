use crate::ray_tracer::common::f_equal;
use crate::ray_tracer::tuple::Tuple;
use std::ops::Mul;

#[derive(Clone)]
pub struct Matrix {
    // [row][col]
    pub value: Vec<Vec<f32>>
}

impl Matrix {
    pub fn new(value: Vec<Vec<f32>>) -> Matrix {
        // TODO: Add validation for all inner vectors being the same length. (Panic)
        Matrix {
            value
        }
    }

    pub fn identity() -> Matrix {
        Matrix::new(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0]
        ])
    }

    pub fn transpose(mut self) -> Matrix {
        for i in 0..self.value.len() {
            for j in i..self.value.len() {
                let temp = self.value[j][i];
                self.value[j][i] = self.value[i][j];
                self.value[i][j] = temp;
            }
        }
        self
    }

    pub fn determinant(&self) -> f32 {
        determinant(&self)
    }

    pub fn inverse(&self) -> Matrix {
        if !self.is_invertible() {
            panic!("The matrix is not invertible!");
        }

        let mut m = self.clone();
        let d = self.determinant();

        for i in 0..self.value.len() {
            for j in 0..self.value[i].len() {
                let c = cofactor(self, i, j);
                m.value[j][i] = c / d;
            }
        }

        m
    }

    // This might not need to be on the matrix's public API
    fn is_invertible(&self) -> bool {
        !f_equal(self.determinant(), 0.0) 
    }
}

// Must overload PartialEq instead of leveraging Derive PartialEq on the Matrix struct. This is
// because we have a custom implementation for comparing floating point numbers f_equal.
impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        // Assumes that all nested vectors are the same length.
        if (&self.value.len() == &other.value.len()) & (&self.value[0].len() == &other.value[0].len()) {
            for i in 0..self.value.len() {
                for j in 0..self.value[i].len() {
                    if !f_equal(self.value[i][j], other.value[i][j]) {
                        return false
                    }
                }
            }
            true
        } else {
            false
        } 
    }
}
impl Eq for Matrix {}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        
        let mut m = self.clone();

        for i in 0..m.value.len() {
            for j in 0..m.value[i].len() {
                m.value[i][j] = self.value[i][0] * other.value[0][j] +
                            self.value[i][1] * other.value[1][j] +
                            self.value[i][2] * other.value[2][j] +
                            self.value[i][3] * other.value[3][j];
            }
        }
        m
    }
}

// This is super janky. There must be a better way to do this. Only works for 4x4 matrices/
impl Mul<&Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, other: &Tuple) -> Tuple {

        let w = match other.w {
            Some(0) => 0.0,
            Some(1) => 1.0,
            _ => panic!("This operation can't be performed on a color."),
        };

        let mut t = Tuple::point(0.0, 0.0, 0.0);
        for i in 0..self.value.len() {
            let value = self.value[i][0] * other.x +
                    self.value[i][1] * other.y +
                    self.value[i][2] * other.z +
                    self.value[i][3] * w;
            
            match i {
                0 => t.x = value,
                1 => t.y = value,
                2 => t.z = value,
                3 => t.w = Some(value as u8),
                _ => panic!("Matrix and Tuple have mismatched sizes.")
            }
        }
        t
    }
}

fn determinant(matrix: &Matrix) -> f32 {
    // Assumes the matrix will always be square
    let dim = matrix.value.len();
    match dim {
        2 => matrix.value[0][0] * matrix.value[1][1] - matrix.value[0][1] * matrix.value[1][0],
        3 | 4 => {
            let mut det = 0.0;
            for i in 0..matrix.value.len() {
                det = det + matrix.value[0][i] * cofactor(&matrix, 0, i);
            }
            det
        },
        _ => panic!("Can only take the determinant of 2x2, 3x3, or 4x4 matrices."),
    }
}

// Takes in a reference to a matrix and clones it internally to return a new submatrix.
fn submatrix(matrix: &Matrix, row: usize, col: usize) -> Matrix {
    let mut sub = matrix.clone();
    sub.value.remove(row);
    for i in 0..sub.value.len() {
        sub.value[i].remove(col);
    }
    sub
}

fn minor(matrix: &Matrix, row: usize, col: usize) -> f32 {    
    determinant(&submatrix(&matrix, row, col))
}

fn cofactor(matrix: &Matrix, row: usize, col: usize) -> f32 {
    if (row + col) % 2 == 0 {
        minor(&matrix, row, col)
    } else {
        -minor(&matrix, row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_4x4_matrix() {
        let m = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5]]
        );
        assert!(
            m.value[0][0] == 1.0 && m.value[0][3] == 4.0 && m.value[1][0] == 5.5 && m.value[1][2] == 7.5 && 
            m.value[2][2] == 11.0 && m.value[3][0] == 13.5 && m.value[3][2] == 15.5,
            "The 4x4 matrix was not created correctly."
        );
    }

    #[test]
    fn create_2x2_matrix() {
        let m = Matrix::new(vec![
            vec![-3.0, 5.0],
            vec![1.0, -2.0]
        ]);
        assert!(
            m.value[0][0] == -3.0 && m.value[0][1] == 5.0 && m.value[1][0] == 1.0 && m.value[1][1] == -2.0,
            "The 2x2 matrix was not created correctly."
        );
    }

    #[test]
    fn create_3x3_matrix() {
        let m = Matrix::new(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0]
        ]);
        assert!(
            m.value[0][0] == -3.0 && m.value[1][1] == -2.0 && m.value[2][2] == 1.0,
            "The 3x3 matrix was not created correctly."
        );
    }

    #[test]
    fn matricies_are_equal() {
        let m1 = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0]
        ]);
        let m2 = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0]
        ]);
        assert!(
            m1 == m2,
            "The matrices are not equal"
        );
    }
    #[test]
    fn matricies_are_not_equal() {
        let m1 = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0]
        ]);
        let m2 = Matrix::new(vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0]
        ]);
        assert_eq!(
            m1 == m2, false,
            "The matrices are not equal!"
        );
    }

    #[test]
    fn multiply_two_matrices() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0]
        ]);
        let b = Matrix::new(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0]
        ]);
        let result = Matrix::new(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0]
        ]);
        assert!(
            &a * &b == result,
            "The result of multiplying two matricies together is incorrect!"
        );
    }

    #[test]
    fn multiply_tuple_and_matrix() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0]
        ]);
        let b = Tuple::point(1.0, 2.0, 3.0);
        let result = Tuple::point(18.0, 24.0, 33.0);
        assert!(
            &a * &b == result,
            "The result of multiplying a matrix and a tuple together is incorrect!"
        );
    }

    #[test]
    fn multiply_matrix_by_identity_matrix() {
        let a = Matrix::new(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0]
        ]);
        assert!(
            &a * &Matrix::identity() == a,
            "The result of multiplying by the identify matrix should be the original matrix!"
        );
    }

    #[test]
    fn multiple_tuple_by_identity() {
        let a = Tuple::point(1.0, 2.0, 3.0);
        assert!(
            &Matrix::identity() * &a == a,
            "The result of multiplying a tuple by the identify matrix should be the original tuple!"
        );
    }

    #[test]
    fn transpose_matrix() {
        let a = Matrix::new(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0]
        ]);
        let result = Matrix::new(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0]
        ]);
        assert!(
            a.transpose() == result,
            "The result of the matrix transpose is not correct!"
        );
    }

    #[test]
    fn transpose_identity_matrix() {
        assert!(
            &Matrix::identity().transpose() == &Matrix::identity(),
            "The result of the matrix transpose is not correct!"
        );
    }

    #[test]
    fn determinant_of_2x2() {
        let a = Matrix::new(vec![
            vec![1.0, 5.0],
            vec![-3.0, 2.0]
        ]);
        let result = determinant(&a);
        assert!(
            f_equal(result, 17.0),
            "The determinant should be 17, instead the result was {}", result
        );
    }

    #[test]
    fn determinant_of_3x3() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0]
        ]);
        let result = determinant(&a);
        assert!(
            f_equal(result, -196.0),
            "The determinant should be -196, instead the result was {}", result
        );
    }

    #[test]
    fn determinant_of_4x4() {
        let a = Matrix::new(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0]
        ]);
        let result = determinant(&a);
        assert!(
            f_equal(result, -4071.0),
            "The determinant should be -4071, instead the result was {}", result
        );
    }

    #[test]
    fn submatrix_of_3x3() {
        let a = Matrix::new(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0]
        ]);
        let b = Matrix::new(vec![
            vec![-3.0, 2.0],
            vec![0.0, 6.0]
        ]);
        assert!(
            submatrix(&a, 0, 2) == b,
            "The submatrix of the 3x3 matrix is not correct!"
        );
    }

    #[test]
    fn submatrix_of_4x4() {
        let a = Matrix::new(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0]
        ]);
        let b = Matrix::new(vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0]
        ]);
        assert!(
            submatrix(&a, 2, 1) == b,
            "The submatrix of the 4x4 matrix is not correct!"
        );
    }

    #[test]
    fn minor_of_3x3() {
        let a = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0]
        ]);
        let result = minor(&a, 1, 0);
        assert!(
            f_equal(result, 25.0),
            "The minor should be 25, instead the result was {}", result
        );
    }

    #[test]
    fn cofactor_of_3x3() {
        let a = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0]
        ]);
        let result = cofactor(&a, 1, 0);
        assert!(
            f_equal(result, -25.0),
            "The cofactor should be -25, instead the result was {}", result
        );
    }

    #[test]
    fn matrix_is_invertible() {
        let a = Matrix::new(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0]
        ]);
        assert!(
            a.is_invertible(),
            "The matrix should be invertible, but isn't."
        );
    }

    #[test]
    fn matrix_is_not_invertible() {
        let a = Matrix::new(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0]
        ]);
        assert!(
            !a.is_invertible(),
            "The matrix should not be invertible, but is."
        );
    }

    #[test]
    fn inverse_4x4_matrix_1() {
        let a = Matrix::new(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0]
        ]);
        let b = Matrix::new(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639]
        ]);
        assert!(
            a.inverse() == b,
            "The inverse of the 4x4 matrix is not correct!"
        );
    }

    #[test]
    fn inverse_4x4_matrix_2() {
        let a = Matrix::new(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0]
        ]);
        let b = Matrix::new(vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308]
        ]);
        assert!(
            a.inverse() == b,
            "The inverse of the 4x4 matrix is not correct!"
        );
    }

    #[test]
    fn inverse_4x4_matrix_3() {
        let a = Matrix::new(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0]
        ]);
        let b = Matrix::new(vec![
            vec![-0.04074, -0.07778, 0.14444, -0.22222],
            vec![-0.07778, 0.03333, 0.36667, -0.33333],
            vec![-0.02901, -0.14630, -0.10926, 0.12963],
            vec![0.17778, 0.06667, -0.26667, 0.33333]
        ]);
        assert!(
            a.inverse() == b,
            "The inverse of the 4x4 matrix is not correct!"
        );
    }
    
    #[test]
    fn multiply_by_inverse() {
        let a = Matrix::new(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0]
        ]);
        let b = Matrix::new(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0]
        ]);
        let c = &a * &b;
        assert!(
            &c * &b.inverse() == a,
            "Multiplying by the inverse failed to produce the right result!"
        );
    }
}