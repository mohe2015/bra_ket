use nalgebra::{Complex, DMatrix, SMatrix};
use std::f64::consts::{PI as PI_f64, SQRT_2};

/// A qubit index
pub type Qubit = usize;

/// A real number
pub type R = f64;

/// A complex number
pub type C = Complex<R>;

/// A density matrix
pub type DensityMatrix = DMatrix<C>;

/// A 2x2 complex matrix
pub type Matrix2x2 = SMatrix<C, 2, 2>;

/// An angle
pub type Angle = R;

/// pi
pub static PI: Angle = PI_f64 as Angle;

/// The acceptable numerical precision for the test
pub static COMPARISON_PRECISION: R = 1e-6;

/// The pauli idenitiy matrix
pub static IDENTITY: Matrix2x2 = Matrix2x2::new(
    C::new(1., 0.),
    C::new(0., 0.),
    C::new(0., 0.),
    C::new(1., 0.),
);

/// The x pauli matrix
pub static SIGMA_X: Matrix2x2 = Matrix2x2::new(
    C::new(0., 0.),
    C::new(1., 0.),
    C::new(1., 0.),
    C::new(0., 0.),
);

/// the y pauli matrix
pub static SIGMA_Y: Matrix2x2 = Matrix2x2::new(
    C::new(0., 0.),
    C::new(0., -1.),
    C::new(0., 1.),
    C::new(0., 0.),
);

/// The z pauli matrix
pub static SIGMA_Z: Matrix2x2 = Matrix2x2::new(
    C::new(1., 0.),
    C::new(0., 0.),
    C::new(0., 0.),
    C::new(-1., 0.),
);

pub static H: Matrix2x2 = Matrix2x2::new(
    C::new(1. / SQRT_2, 0.),
    C::new(1. / SQRT_2, 0.),
    C::new(1. / SQRT_2, 0.),
    C::new(-1. / SQRT_2, 0.),
);
