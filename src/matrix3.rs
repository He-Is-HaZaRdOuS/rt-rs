#![allow(dead_code)]

use crate::DEBUG;
use crate::vector3::Vector3;

#[derive(Debug)]
pub struct Matrix3 {
    pub m_data: [[f32; 3]; 3],
}

pub trait Instantiator {
    fn into(self) -> Matrix3;
}

/* Associative methods implementation block */
impl Matrix3 {
    pub fn det(&self) -> f32 {
        return Matrix3::s_det3(&self);
    }
}

/* Associative functions implementation block */
impl Matrix3 {
    /* Constructor */
    pub fn new<A>(args: A) -> Matrix3
    where
        A: Instantiator,
    {
        return args.into();
    }

    /* Static Constructors */
    pub fn identity() -> Matrix3 {
        if DEBUG {
            println!("Matrix3::Instantiator: Invoked Identity overload!");
        }
        return Matrix3 {
            m_data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        };
    }

    pub fn zero() -> Matrix3 {
        if DEBUG {
            println!("Matrix3::Instantiator: Invoked Zero overload!");
        }
        return Matrix3 {
            m_data: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        };
    }

    fn s_det3(matrix: &Matrix3) -> f32 {
        return matrix.m_data[0][0] * matrix.m_data[1][1] * matrix.m_data[2][2]
            + matrix.m_data[0][1] * matrix.m_data[1][2] * matrix.m_data[2][0]
            + matrix.m_data[0][2] * matrix.m_data[2][1] * matrix.m_data[1][0]
            - matrix.m_data[2][0] * matrix.m_data[1][1] * matrix.m_data[0][2]
            - matrix.m_data[1][0] * matrix.m_data[0][1] * matrix.m_data[2][2]
            - matrix.m_data[0][0] * matrix.m_data[1][2] * matrix.m_data[2][1];
    }
}

impl Instantiator for () {
    fn into(self) -> Matrix3 {
        if DEBUG {
            println!("Matrix3::Instantiator: Invoked () Identity overload!");
        }
        return Matrix3 {
            m_data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        };
    }
}

impl Instantiator for i64 {
    fn into(self) -> Matrix3 {
        if DEBUG {
            println!("Matrix3::Instantiator: Invoked (i64) Zero overload!");
        }
        return Matrix3 {
            m_data: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        };
    }
}

impl Instantiator for &Matrix3 {
    fn into(self) -> Matrix3 {
        if DEBUG {
            println!("Matrix3::Instantiator: Invoked (&Matrix3) overload!");
        }
        return Matrix3 {
            m_data: self.m_data,
        };
    }
}

impl Instantiator for (f32, f32, f32, f32, f32, f32, f32, f32, f32) {
    fn into(self) -> Matrix3 {
        if DEBUG {
            println!("Matrix3::Instantiator: Invoked (f32, f32, f32, f32, f32, f32, f32, f32, f32) overload!");
        }
        return Matrix3 {
            m_data: [
                [self.0, self.1, self.2],
                [self.3, self.4, self.5],
                [self.6, self.7, self.8],
            ],
        };
    }
}

impl Instantiator for (&Vector3, &Vector3, &Vector3) {
    fn into(self) -> Matrix3 {
        if DEBUG {
            println!("Matrix3::Instantiator: Invoked (&Vector3, &Vector3, &Vector3) overload!");
        }
        return Matrix3 {
            m_data: [
                [self.0.m_data[0], self.1.m_data[0], self.2.m_data[0]],
                [self.0.m_data[1], self.1.m_data[1], self.2.m_data[1]],
                [self.0.m_data[2], self.1.m_data[2], self.2.m_data[2]],
            ],
        };
    }
}
