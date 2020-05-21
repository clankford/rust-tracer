use crate::ray_tracer::common::f_equal;

pub struct Matrix {
    // [row][col]
    pub value: Vec<Vec<f32>>
}

impl Matrix {
    pub fn new(value: Vec<Vec<f32>>) -> Matrix {
        // TODO: Add validation for all inner vectors being the same length.
        Matrix {
            value
        }
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
            "The matrices are not equal"
        );
    }
}