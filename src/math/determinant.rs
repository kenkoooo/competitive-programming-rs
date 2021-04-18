pub fn calc_determinant<T>(mut matrix: Vec<Vec<T>>) -> T
where
    T: Copy + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T>,
{
    let n = matrix.len();
    assert!(
        matrix.iter().all(|row| row.len() == n),
        "The matrix is not square!"
    );

    for i in 0..n {
        for j in (i + 1)..n {
            let b = matrix[j][i] / matrix[i][i];
            for k in 0..n {
                matrix[j][k] = matrix[j][k] - matrix[i][k] * b;
            }
        }
    }

    let mut det = matrix[0][0];
    for i in 1..n {
        det = det * matrix[i][i];
    }
    det
}

#[cfg(test)]
mod tests {
    use crate::math::determinant::calc_determinant;

    #[test]
    fn test_calc_determinant() {
        let a = vec![vec![1, 2, 3], vec![2, 2, 4], vec![2, 4, 5]];
        let det = calc_determinant(a);
        assert_eq!(det, 2);
    }
}
