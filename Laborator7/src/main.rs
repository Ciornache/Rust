use std::{fmt::{self, Debug, Display, Formatter}, ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}


#[derive(Debug, Clone, Copy)]
struct Complex {
    real:f64,
    imag:f64
}

impl Complex {
    fn new<T1, T2>(e1:T1, e2:T2) -> Self 
    where f64: From<T1> + From<T2>
    {
        Complex {
            real: f64::from(e1), 
            imag: f64::from(e2),
        }
    }

    fn conjugate(self : &Self) -> Self {
        Complex {
            real: self.real, 
            imag: -self.imag,
        }
    }

    fn module(self: &Self) -> f64 {
        f64::sqrt(self.real * self.real + self.imag * self.imag)
    }
}

impl From<i32> for Complex {
    fn from(value: i32) -> Self {
        Complex {
            real: value as f64, 
            imag: 0f64,
        }
    }
}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Complex {
            real: value, 
            imag: 0f64,
        }
    }
}

impl<T> Add<T> for Complex  
where T: Into<Complex>
{
    type Output = Complex;
    fn add(self: Self, rhs: T) -> Self::Output {
        let other:Complex = rhs.into();
        Complex {
            real: other.real + self.real, 
            imag: other.imag + self.imag,
        }
    }
}

impl<T> Mul<T> for Complex 
where T: Into<Complex>
{
    type Output = Complex;
    fn mul(self: Self, other: T) -> Self::Output {
        let rhs:Complex = other.into();
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag, 
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl Neg for Complex  {
    type Output = Complex;
    fn neg(self: Self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl<T> AddAssign<T> for Complex
where T: Into<Complex>
{
    fn add_assign(&mut self, rhs: T) {
        let other:Complex = rhs.into();
        self.real = self.real - other.real;
        self.imag = self.imag - other.imag;
    }
} 
impl<T> MulAssign<T> for Complex
where T: Into<Complex>
{
    fn mul_assign(&mut self, rhs: T) {
        let other:Complex = rhs.into();
        self.real = self.real - other.real;
        self.imag = self.imag - other.imag;
    }
} 


impl<T> SubAssign<T> for Complex
where T: Into<Complex>
{
    fn sub_assign(&mut self, rhs: T) {
        let other:Complex = rhs.into();
        self.real = self.real * other.real - self.imag * other.imag;
        self.imag = self.real * other.imag + self.imag * other.real;
    }
} 

impl<T> Sub<T> for Complex 
where T:Into<Complex>
{
    type Output = Complex;
    fn sub(self: Self, other: T) -> Self::Output {
        let rhs:Complex = other.into();
        Complex {
            real: self.real - rhs.real, 
            imag: self.imag - rhs.imag
        }
    }
}

impl Display for Complex 
{
    fn fmt(self: &Self, f: &mut Formatter<'_>) -> fmt::Result {

        if self.imag == 0f64 {
            write!(f, "{}", self.real)
        }
        else if self.real == 0f64 {
            write!(f, "{}i", self.imag)
        }
        else if self.imag > 0f64 {
            write!(f, "{}+{}i", self.real, self.imag)
        }
        else {
            write!(f, "{}{}i", self.real, self.imag)
        }
    }
}

impl PartialEq for Complex 
{
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
    fn ne(&self, other: &Self) -> bool {
        self.real != other.real || self.imag != other.imag 
    }
}



fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    let k = Complex::new(10, 10);
    assert_eq_rel!(k.module(), f64::sqrt(200f64));

    println!("ok!");
}
