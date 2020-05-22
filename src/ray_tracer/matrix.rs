use crate::ray_tracer::common::f_equal;
use crate::ray_tracer::tuple::Tuple;
use std::ops::Mul;

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

// Only works for 4x4 matrices
impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        
        let mut m = Matrix::new(vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0]
        ]);

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
}