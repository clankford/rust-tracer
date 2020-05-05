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
    fn create_matrix() {
        let m = Matrix::new(vec![
                            vec![1.0, 2.0, 3.0, 4.0],
                            vec![5.5, 6.5, 7.5, 8.5],
                            vec![9.0, 10.0, 11.0, 12.0],
                            vec![13.5, 14.5, 15.5, 16.5]]);
        assert!(
            m.value[0][0] == 1.0 && m.value[0][3] == 4.0 && m.value[1][0] == 5.5 && m.value[1][2] == 7.5 && 
            m.value[2][2] == 11.0 && m.value[3][0] == 13.5 && m.value[3][2] == 15.5,
            "The matrix was not created correctly."
        );
    }
}