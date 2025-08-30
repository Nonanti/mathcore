use crate::types::MathError;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum PrecisionNumber {
    Exact(BigRational),
    Float(f64),
    Integer(BigInt),
}

impl PrecisionNumber {
    pub fn from_f64(value: f64) -> Self {
        if value.fract() == 0.0 && value.is_finite() {
            PrecisionNumber::Integer(BigInt::from_f64(value).unwrap())
        } else {
            PrecisionNumber::Float(value)
        }
    }

    pub fn from_str_with_precision(s: &str) -> Result<Self, MathError> {
        if let Ok(int) = BigInt::from_str(s) {
            return Ok(PrecisionNumber::Integer(int));
        }

        if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() == 2 {
                let num = BigInt::from_str(parts[0])
                    .map_err(|_| MathError::ParseError("Invalid numerator".to_string()))?;
                let den = BigInt::from_str(parts[1])
                    .map_err(|_| MathError::ParseError("Invalid denominator".to_string()))?;
                return Ok(PrecisionNumber::Exact(BigRational::new(num, den)));
            }
        }

        if s.contains('.') {
            let decimal_places = s.split('.').nth(1).map_or(0, |d| d.len());
            let without_dot = s.replace('.', "");

            if let Ok(num) = BigInt::from_str(&without_dot) {
                let den = BigInt::from(10_i32).pow(decimal_places as u32);
                return Ok(PrecisionNumber::Exact(BigRational::new(num, den)));
            }
        }

        s.parse::<f64>()
            .map(PrecisionNumber::Float)
            .map_err(|_| MathError::ParseError(format!("Cannot parse number: {}", s)))
    }

    pub fn to_rational(&self) -> BigRational {
        match self {
            PrecisionNumber::Exact(r) => r.clone(),
            PrecisionNumber::Integer(i) => BigRational::from(i.clone()),
            PrecisionNumber::Float(f) => {
                BigRational::from_float(*f).unwrap_or_else(BigRational::zero)
            }
        }
    }

    pub fn add(&self, other: &PrecisionNumber) -> PrecisionNumber {
        match (self, other) {
            (PrecisionNumber::Integer(a), PrecisionNumber::Integer(b)) => {
                PrecisionNumber::Integer(a + b)
            }
            (PrecisionNumber::Exact(a), PrecisionNumber::Exact(b)) => PrecisionNumber::Exact(a + b),
            (PrecisionNumber::Float(a), PrecisionNumber::Float(b)) => PrecisionNumber::Float(a + b),
            _ => {
                let a = self.to_rational();
                let b = other.to_rational();
                PrecisionNumber::Exact(a + b)
            }
        }
    }

    pub fn subtract(&self, other: &PrecisionNumber) -> PrecisionNumber {
        match (self, other) {
            (PrecisionNumber::Integer(a), PrecisionNumber::Integer(b)) => {
                PrecisionNumber::Integer(a - b)
            }
            (PrecisionNumber::Exact(a), PrecisionNumber::Exact(b)) => PrecisionNumber::Exact(a - b),
            (PrecisionNumber::Float(a), PrecisionNumber::Float(b)) => PrecisionNumber::Float(a - b),
            _ => {
                let a = self.to_rational();
                let b = other.to_rational();
                PrecisionNumber::Exact(a - b)
            }
        }
    }

    pub fn multiply(&self, other: &PrecisionNumber) -> PrecisionNumber {
        match (self, other) {
            (PrecisionNumber::Integer(a), PrecisionNumber::Integer(b)) => {
                PrecisionNumber::Integer(a * b)
            }
            (PrecisionNumber::Exact(a), PrecisionNumber::Exact(b)) => PrecisionNumber::Exact(a * b),
            (PrecisionNumber::Float(a), PrecisionNumber::Float(b)) => PrecisionNumber::Float(a * b),
            _ => {
                let a = self.to_rational();
                let b = other.to_rational();
                PrecisionNumber::Exact(a * b)
            }
        }
    }

    pub fn divide(&self, other: &PrecisionNumber) -> Result<PrecisionNumber, MathError> {
        if other.is_zero() {
            return Err(MathError::DivisionByZero);
        }

        match (self, other) {
            (PrecisionNumber::Exact(a), PrecisionNumber::Exact(b)) => {
                Ok(PrecisionNumber::Exact(a / b))
            }
            (PrecisionNumber::Float(a), PrecisionNumber::Float(b)) => {
                Ok(PrecisionNumber::Float(a / b))
            }
            _ => {
                let a = self.to_rational();
                let b = other.to_rational();
                Ok(PrecisionNumber::Exact(a / b))
            }
        }
    }

    pub fn power(&self, exponent: &PrecisionNumber) -> Result<PrecisionNumber, MathError> {
        match (self, exponent) {
            (PrecisionNumber::Integer(base), PrecisionNumber::Integer(exp)) => {
                if let Some(exp_u32) = exp.to_u32() {
                    Ok(PrecisionNumber::Integer(base.pow(exp_u32)))
                } else if exp.is_negative() {
                    let positive_exp = -exp;
                    if let Some(exp_u32) = positive_exp.to_u32() {
                        let result = BigRational::new(BigInt::one(), base.pow(exp_u32));
                        Ok(PrecisionNumber::Exact(result))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Exponent too large".to_string(),
                        ))
                    }
                } else {
                    Err(MathError::InvalidOperation(
                        "Exponent too large".to_string(),
                    ))
                }
            }
            (PrecisionNumber::Float(base), PrecisionNumber::Float(exp)) => {
                Ok(PrecisionNumber::Float(base.powf(*exp)))
            }
            _ => {
                if let (Some(base_f64), Some(exp_f64)) = (self.to_f64(), exponent.to_f64()) {
                    Ok(PrecisionNumber::Float(base_f64.powf(exp_f64)))
                } else {
                    Err(MathError::InvalidOperation(
                        "Cannot compute power with these operands".to_string(),
                    ))
                }
            }
        }
    }

    pub fn factorial(&self) -> Result<PrecisionNumber, MathError> {
        match self {
            PrecisionNumber::Integer(n) if !n.is_negative() => {
                let mut result = BigInt::one();
                let mut i = BigInt::from(2);
                while i <= *n {
                    result *= &i;
                    i += 1;
                }
                Ok(PrecisionNumber::Integer(result))
            }
            _ => Err(MathError::InvalidOperation(
                "Factorial requires non-negative integer".to_string(),
            )),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            PrecisionNumber::Integer(i) => i.is_zero(),
            PrecisionNumber::Exact(r) => r.is_zero(),
            PrecisionNumber::Float(f) => f.abs() < f64::EPSILON,
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self {
            PrecisionNumber::Float(f) => Some(*f),
            PrecisionNumber::Integer(i) => i.to_f64(),
            PrecisionNumber::Exact(r) => r
                .numer()
                .to_f64()
                .and_then(|n| r.denom().to_f64().map(|d| n / d)),
        }
    }

    pub fn sqrt(&self) -> Result<PrecisionNumber, MathError> {
        match self {
            PrecisionNumber::Integer(n) if !n.is_negative() => {
                let n_f64 = n.to_f64().ok_or(MathError::Overflow)?;
                let sqrt_val = n_f64.sqrt();

                if sqrt_val.fract() == 0.0 {
                    Ok(PrecisionNumber::Integer(
                        BigInt::from_f64(sqrt_val).unwrap(),
                    ))
                } else {
                    Ok(PrecisionNumber::Float(sqrt_val))
                }
            }
            PrecisionNumber::Float(f) if *f >= 0.0 => Ok(PrecisionNumber::Float(f.sqrt())),
            PrecisionNumber::Exact(r) if !r.is_negative() => {
                let n_f64 = r.to_f64().ok_or(MathError::Overflow)?;
                Ok(PrecisionNumber::Float(n_f64.sqrt()))
            }
            _ => Err(MathError::InvalidOperation(
                "Cannot take square root of negative number".to_string(),
            )),
        }
    }
}

impl fmt::Display for PrecisionNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrecisionNumber::Integer(i) => write!(f, "{}", i),
            PrecisionNumber::Exact(r) => {
                if r.denom().is_one() {
                    write!(f, "{}", r.numer())
                } else {
                    write!(f, "{}/{}", r.numer(), r.denom())
                }
            }
            PrecisionNumber::Float(fl) => write!(f, "{}", fl),
        }
    }
}

pub struct ArbitraryPrecision;

impl ArbitraryPrecision {
    pub fn compute_pi(precision: usize) -> PrecisionNumber {
        let mut pi = BigRational::zero();
        let sixteen = BigInt::from(16);

        for k in 0..precision {
            let k_bigint = BigInt::from(k);
            let eight_k = &k_bigint * 8;

            let term1 = BigRational::new(BigInt::from(4), &eight_k + 1);
            let term2 = BigRational::new(BigInt::from(2), &eight_k + 4);
            let term3 = BigRational::new(BigInt::one(), &eight_k + 5);
            let term4 = BigRational::new(BigInt::one(), &eight_k + 6);

            let series_term = term1 - term2 - term3 - term4;
            let divisor = sixteen.pow(k as u32);

            pi += BigRational::new(BigInt::one(), divisor) * series_term;
        }

        PrecisionNumber::Exact(pi)
    }

    pub fn compute_e(precision: usize) -> PrecisionNumber {
        let mut e = BigRational::one();
        let mut factorial = BigInt::one();

        for n in 1..precision {
            factorial *= n;
            e += BigRational::new(BigInt::one(), factorial.clone());
        }

        PrecisionNumber::Exact(e)
    }

    pub fn compute_sqrt(n: &BigInt, precision: usize) -> PrecisionNumber {
        if n.is_negative() {
            return PrecisionNumber::Float(f64::NAN);
        }

        let mut x = n.clone();
        let two = BigInt::from(2);

        for _ in 0..precision {
            let x_squared = &x * &x;
            if x_squared == *n {
                return PrecisionNumber::Integer(x);
            }

            x = (&x + n / &x) / &two;
        }

        PrecisionNumber::Integer(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision_arithmetic() {
        let a = PrecisionNumber::from_str_with_precision("1/3").unwrap();
        let b = PrecisionNumber::from_str_with_precision("1/6").unwrap();
        let c = a.add(&b);
        println!("1/3 + 1/6 = {}", c);

        let d = PrecisionNumber::from_str_with_precision("2.5").unwrap();
        let e = PrecisionNumber::from_str_with_precision("3.7").unwrap();
        let f = d.multiply(&e);
        println!("2.5 * 3.7 = {}", f);
    }

    #[test]
    fn test_compute_constants() {
        let pi = ArbitraryPrecision::compute_pi(10);
        println!("π ≈ {}", pi);

        let e = ArbitraryPrecision::compute_e(20);
        println!("e ≈ {}", e);
    }
}
