pub struct Matrix {
    // [row][col]
    pub value: Vec<Vec<f32>>
}

impl Matrix {
    pub fn new(value: Vec<Vec<f32>>) -> Matrix {
        Matrix {
            value
        }
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
}