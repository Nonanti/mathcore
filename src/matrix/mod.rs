use crate::types::{Expr, MathError};
use nalgebra::{DMatrix, DVector};
use std::fmt;

// Type alias for SVD result
type SVDResult = Result<(DMatrix<f64>, DVector<f64>, DMatrix<f64>), MathError>;

#[derive(Debug, Clone)]
pub struct SymbolicMatrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<Expr>>,
}

impl SymbolicMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![Expr::zero(); cols]; rows];
        SymbolicMatrix { rows, cols, data }
    }

    pub fn from_vec(data: Vec<Vec<f64>>) -> Result<Self, MathError> {
        if data.is_empty() {
            return Err(MathError::InvalidOperation("Empty matrix".to_string()));
        }

        let rows = data.len();
        let cols = data[0].len();

        if !data.iter().all(|row| row.len() == cols) {
            return Err(MathError::InvalidOperation(
                "Inconsistent row lengths".to_string(),
            ));
        }

        let expr_data = data
            .into_iter()
            .map(|row| row.into_iter().map(Expr::Number).collect())
            .collect();

        Ok(SymbolicMatrix {
            rows,
            cols,
            data: expr_data,
        })
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::new(size, size);
        for i in 0..size {
            matrix.data[i][i] = Expr::one();
        }
        matrix
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&Expr> {
        self.data.get(row).and_then(|r| r.get(col))
    }

    pub fn set(&mut self, row: usize, col: usize, value: Expr) -> Result<(), MathError> {
        if row >= self.rows || col >= self.cols {
            return Err(MathError::InvalidOperation(
                "Index out of bounds".to_string(),
            ));
        }
        self.data[row][col] = value;
        Ok(())
    }

    pub fn add(&self, other: &SymbolicMatrix) -> Result<SymbolicMatrix, MathError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MathError::InvalidOperation(
                "Matrix dimensions don't match".to_string(),
            ));
        }

        let mut result = SymbolicMatrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = Expr::Binary {
                    op: crate::types::BinaryOp::Add,
                    left: Box::new(self.data[i][j].clone()),
                    right: Box::new(other.data[i][j].clone()),
                };
            }
        }
        Ok(result)
    }

    pub fn multiply(&self, other: &SymbolicMatrix) -> Result<SymbolicMatrix, MathError> {
        if self.cols != other.rows {
            return Err(MathError::InvalidOperation(format!(
                "Cannot multiply {}x{} with {}x{}",
                self.rows, self.cols, other.rows, other.cols
            )));
        }

        let mut result = SymbolicMatrix::new(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = Expr::zero();
                for k in 0..self.cols {
                    let product = Expr::Binary {
                        op: crate::types::BinaryOp::Multiply,
                        left: Box::new(self.data[i][k].clone()),
                        right: Box::new(other.data[k][j].clone()),
                    };
                    sum = Expr::Binary {
                        op: crate::types::BinaryOp::Add,
                        left: Box::new(sum),
                        right: Box::new(product),
                    };
                }
                result.data[i][j] = sum;
            }
        }

        Ok(result)
    }

    pub fn transpose(&self) -> SymbolicMatrix {
        let mut result = SymbolicMatrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j].clone();
            }
        }
        result
    }

    pub fn determinant(&self) -> Result<Expr, MathError> {
        if self.rows != self.cols {
            return Err(MathError::InvalidOperation(
                "Determinant requires square matrix".to_string(),
            ));
        }

        match self.rows {
            0 => Err(MathError::InvalidOperation("Empty matrix".to_string())),
            1 => Ok(self.data[0][0].clone()),
            2 => {
                let a = &self.data[0][0];
                let b = &self.data[0][1];
                let c = &self.data[1][0];
                let d = &self.data[1][1];

                Ok(Expr::Binary {
                    op: crate::types::BinaryOp::Subtract,
                    left: Box::new(Expr::Binary {
                        op: crate::types::BinaryOp::Multiply,
                        left: Box::new(a.clone()),
                        right: Box::new(d.clone()),
                    }),
                    right: Box::new(Expr::Binary {
                        op: crate::types::BinaryOp::Multiply,
                        left: Box::new(b.clone()),
                        right: Box::new(c.clone()),
                    }),
                })
            }
            n => {
                let mut det = Expr::zero();
                for j in 0..n {
                    let minor = self.minor(0, j)?;
                    let cofactor = minor.determinant()?;

                    let term = Expr::Binary {
                        op: crate::types::BinaryOp::Multiply,
                        left: Box::new(self.data[0][j].clone()),
                        right: Box::new(cofactor),
                    };

                    if j % 2 == 0 {
                        det = Expr::Binary {
                            op: crate::types::BinaryOp::Add,
                            left: Box::new(det),
                            right: Box::new(term),
                        };
                    } else {
                        det = Expr::Binary {
                            op: crate::types::BinaryOp::Subtract,
                            left: Box::new(det),
                            right: Box::new(term),
                        };
                    }
                }
                Ok(det)
            }
        }
    }

    fn minor(&self, row: usize, col: usize) -> Result<SymbolicMatrix, MathError> {
        if row >= self.rows || col >= self.cols {
            return Err(MathError::InvalidOperation(
                "Index out of bounds".to_string(),
            ));
        }

        let mut minor = SymbolicMatrix::new(self.rows - 1, self.cols - 1);
        let mut mi = 0;

        for i in 0..self.rows {
            if i == row {
                continue;
            }
            let mut mj = 0;
            for j in 0..self.cols {
                if j == col {
                    continue;
                }
                minor.data[mi][mj] = self.data[i][j].clone();
                mj += 1;
            }
            mi += 1;
        }

        Ok(minor)
    }

    pub fn trace(&self) -> Result<Expr, MathError> {
        if self.rows != self.cols {
            return Err(MathError::InvalidOperation(
                "Trace requires square matrix".to_string(),
            ));
        }

        let mut trace = Expr::zero();
        for i in 0..self.rows {
            trace = Expr::Binary {
                op: crate::types::BinaryOp::Add,
                left: Box::new(trace),
                right: Box::new(self.data[i][i].clone()),
            };
        }
        Ok(trace)
    }
}

pub struct LinearAlgebra;

impl LinearAlgebra {
    pub fn solve_system(a: &DMatrix<f64>, b: &DVector<f64>) -> Result<DVector<f64>, MathError> {
        let decomp = a.clone().lu();
        decomp
            .solve(b)
            .ok_or_else(|| MathError::InvalidOperation("System has no unique solution".to_string()))
    }

    pub fn eigenvalues(matrix: &DMatrix<f64>) -> Result<Vec<num_complex::Complex64>, MathError> {
        if !matrix.is_square() {
            return Err(MathError::InvalidOperation(
                "Eigenvalues require square matrix".to_string(),
            ));
        }

        let schur = matrix.clone().schur();
        let eigenvalues = schur.complex_eigenvalues();
        Ok(eigenvalues.iter().cloned().collect())
    }

    pub fn qr_decomposition(matrix: &DMatrix<f64>) -> (DMatrix<f64>, DMatrix<f64>) {
        let qr = matrix.clone().qr();
        (qr.q(), qr.r())
    }

    pub fn svd(matrix: &DMatrix<f64>) -> SVDResult {
        let svd = matrix.clone().svd(true, true);

        match (svd.u, svd.v_t) {
            (Some(u), Some(vt)) => Ok((u, svd.singular_values, vt)),
            _ => Err(MathError::InvalidOperation(
                "SVD computation failed".to_string(),
            )),
        }
    }

    pub fn rank(matrix: &DMatrix<f64>, tolerance: f64) -> usize {
        let svd = matrix.clone().svd(false, false);
        svd.singular_values
            .iter()
            .filter(|&&s| s > tolerance)
            .count()
    }

    pub fn norm(matrix: &DMatrix<f64>, norm_type: NormType) -> f64 {
        match norm_type {
            NormType::Frobenius => matrix.iter().map(|x| x * x).sum::<f64>().sqrt(),
            NormType::L1 => (0..matrix.ncols())
                .map(|j| {
                    (0..matrix.nrows())
                        .map(|i| matrix[(i, j)].abs())
                        .sum::<f64>()
                })
                .fold(0.0_f64, f64::max),
            NormType::L2 => {
                let svd = matrix.clone().svd(false, false);
                svd.singular_values[0]
            }
            NormType::LInf => (0..matrix.nrows())
                .map(|i| {
                    (0..matrix.ncols())
                        .map(|j| matrix[(i, j)].abs())
                        .sum::<f64>()
                })
                .fold(0.0_f64, f64::max),
        }
    }

    pub fn condition_number(matrix: &DMatrix<f64>) -> Result<f64, MathError> {
        if !matrix.is_square() {
            return Err(MathError::InvalidOperation(
                "Condition number requires square matrix".to_string(),
            ));
        }

        let svd = matrix.clone().svd(false, false);
        let max_sv = svd.singular_values.max();
        let min_sv = svd.singular_values.min();

        if min_sv.abs() < 1e-15 {
            Ok(f64::INFINITY)
        } else {
            Ok(max_sv / min_sv)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NormType {
    Frobenius,
    L1,
    L2,
    LInf,
}

impl fmt::Display for SymbolicMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix {}x{}:", self.rows, self.cols)?;
        for row in &self.data {
            write!(f, "[")?;
            for (i, elem) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", elem)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiplication() {
        let a = SymbolicMatrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();

        let b = SymbolicMatrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]).unwrap();

        let result = a.multiply(&b).unwrap();
        assert_eq!(result.rows, 2);
        assert_eq!(result.cols, 2);
    }

    #[test]
    fn test_determinant() {
        let matrix = SymbolicMatrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();

        let det = matrix.determinant().unwrap();
        println!("Determinant: {}", det);
    }
}
