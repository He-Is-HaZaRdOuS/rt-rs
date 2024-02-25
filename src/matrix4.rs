#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::f32::consts::PI;
use std::fs::File;
use std::io::Read;
use crate::DEBUG;
use crate::vector3::Vector3;
use crate::vector4::Vector4;
use crate::matrix3::Matrix3;

#[derive(Debug, Serialize, Deserialize)]
pub struct Matrix4 {
    #[serde(rename = "values")]
    m_data: [[f32; 4]; 4],
}

pub trait Instantiator {
    fn into(self) -> Matrix4;
}

pub trait Rotator {
    fn into(self) -> Matrix4;
}

pub trait Multiplier<T> {
    fn mult(&self, obj : &T) -> T;
}

/* Associative methods implementation block */
impl Matrix4 {
    pub fn det(&self) -> f32 {
        return Matrix4::s_det4(&self);
    }
}
impl Multiplier<Vector4> for Matrix4 {
    fn mult(&self, obj : &Vector4) -> Vector4 {
        if DEBUG {
            println!("Matrix4::Multiplier: Invoked (&Vector4) overload!");
        }
        return Vector4::new((self.m_data[0][0] * obj.m_data[0] + self.m_data[0][1] * obj.m_data[1] + self.m_data[0][2] * obj.m_data[2] + self.m_data[0][3] * obj.m_data[3],
                                 self.m_data[1][0] * obj.m_data[0] + self.m_data[1][1] * obj.m_data[1] + self.m_data[1][2] * obj.m_data[2] + self.m_data[1][3] * obj.m_data[3],
                                 self.m_data[2][0] * obj.m_data[0] + self.m_data[2][1] * obj.m_data[1] + self.m_data[2][2] * obj.m_data[2] + self.m_data[2][3] * obj.m_data[3],
                                 self.m_data[3][0] * obj.m_data[0] + self.m_data[3][1] * obj.m_data[1] + self.m_data[3][2] * obj.m_data[2] + self.m_data[3][3] * obj.m_data[3]));
    }
}

impl Multiplier<Matrix4> for Matrix4 {
    fn mult(&self, obj : &Matrix4) -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Multiplier: Invoked (&Matrix4) overload!");
        }
        let mut m = Self::new(0);

        for i in 0..4 {
            for k in 0..4 {
                for j in 0..4 {
                    m.m_data[i][j] += self.m_data[i][k] * obj.m_data[k][j];
                }
            }
        }

        return m;
    }

}

/* Associative functions implementation block */
impl Matrix4 {
    /* Constructor */
    pub fn new<A> (args: A) -> Matrix4
        where A: Instantiator
    {
        return args.into();
    }

    pub fn json(path: &str) -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Instantiator: Invoked json overload!");
        }
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&data).expect("JSON was not well-formatted");

        let vals = json.get("values");

        assert_eq!(16, vals.unwrap().as_array().unwrap().len());

        let vals: Option<&Vec<Value>> = vals.unwrap().as_array();
        let mut arr = [0.0f32; 16];
        for i in 0..16 {
            arr[i] = vals.unwrap().get(i).unwrap().as_f64().unwrap() as f32;
        }

        return Matrix4::new((arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6], arr[7], arr[8], arr[9], arr[10], arr[11], arr[12], arr[13], arr[14], arr[15]));
    }

    /* Static Constructors */
    pub fn identity() -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Instantiator: Invoked Identity overload!");
        }
        return Matrix4{m_data: [[1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]]};

    }

    pub fn zero() -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Instantiator: Invoked Zero overload!");
        }
        return Matrix4{m_data: [[0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]]};

    }
    pub fn translate(tx: f32, ty: f32, tz: f32) -> Matrix4 {
        let mut m = Self::identity();
        m.m_data[0][3] = tx;
        m.m_data[1][3] = ty;
        m.m_data[2][3] = tz;
        return m;
    }

    pub fn scale(sx: f32, sy: f32, sz: f32) -> Matrix4 {
        let mut m = Self::identity();
        m.m_data[0][0] = sx;
        m.m_data[1][1] = sy;
        m.m_data[2][2] = sz;
        return m;
    }

    pub fn rotate_x(angle: f32) -> Matrix4{
        let mut m = Self::identity();
        let angle = angle * PI / 180.0;
        m.m_data[1][1] = angle.cos();
        m.m_data[1][2] = -angle.sin();
        m.m_data[2][1] = angle.sin();
        m.m_data[2][2] = angle.cos();
        return m;
    }

    pub fn rotate_y(angle: f32) -> Matrix4{
        let mut m = Self::identity();
        let angle = angle * PI / 180.0;
        m.m_data[0][0] = angle.cos();
        m.m_data[0][2] = angle.sin();
        m.m_data[2][0] = -angle.sin();
        m.m_data[2][2] = angle.cos();
        return m;
    }

    pub fn rotate_z(angle: f32) -> Matrix4{
        let mut m = Self::identity();
        let angle = angle * PI / 180.0;
        m.m_data[0][0] = angle.cos();
        m.m_data[0][1] = -angle.sin();
        m.m_data[1][0] = angle.sin();
        m.m_data[1][1] = angle.cos();
        return m;
    }

    pub fn rotate<A> (args: A) -> Matrix4
        where A: Rotator
    {
        return args.into();
    }

    fn s_rotate (x: f32, y: f32, z: f32, angle: f32) -> Matrix4 {
        let angle = angle * PI / 180.0;
        let c : f32 = angle.cos();
        let s : f32 = angle.sin();
        let t : f32 = 1.0 - c;

        let magnitude : f32 = (x.powf(2.0) + y.powf(2.0) + z.powf(2.0)).sqrt();
        let x = x / magnitude;
        let y = y / magnitude;
        let z = z / magnitude;

        let tx : f32 = t*x;
        let ty : f32 = t*y;
        let tz : f32 = t*z;

        let txx : f32 = tx*x;
        let txy : f32 = tx*y;
        let txz : f32 = tx*z;

        let tyy : f32 = ty*y;
        let tyz : f32 = ty*z;

        let tzz : f32 = tz*z;

        let sx : f32 = s*x;
        let sy : f32 = s*y;
        let sz : f32 = s*z;

        let mut m = Self::identity();

        m.m_data[0][0] = txx + c;
        m.m_data[0][1] = txy - sz;
        m.m_data[0][2] = txz + sy;
        //m.m_data[0][3] = 0.0;

        m.m_data[1][0] = txy + sz;
        m.m_data[1][1] = tyy + c;
        m.m_data[1][2] = tyz - sx;
        //m.m_data[1][3] = 0.0;

        m.m_data[2][0] = txz - sy;
        m.m_data[2][1] = tyz + sx;
        m.m_data[2][2] = tzz + c;
        //m.m_data[2][3] = 0.0;

        m.m_data[3][0] = 0.0;
        m.m_data[3][1] = 0.0;
        m.m_data[3][2] = 0.0;
        //m.m_data[3][3] = 1.0;

        return m;
    }

    fn s_det4(matrix: &Matrix4) -> f32 {
        let m0 = Matrix3::new((&Vector3::new((matrix.m_data[1][1], matrix.m_data[1][2], matrix.m_data[1][3])),
                                    &Vector3::new((matrix.m_data[2][1], matrix.m_data[2][2], matrix.m_data[2][3])),
                                    &Vector3::new((matrix.m_data[3][1], matrix.m_data[3][2], matrix.m_data[3][3])) ));

        let m1 = Matrix3::new((&Vector3::new((matrix.m_data[1][0], matrix.m_data[1][2], matrix.m_data[1][3])),
                                    &Vector3::new((matrix.m_data[2][0], matrix.m_data[2][2], matrix.m_data[2][3])),
                                    &Vector3::new((matrix.m_data[3][0], matrix.m_data[3][2], matrix.m_data[3][3])) ));

        let m2 = Matrix3::new((&Vector3::new((matrix.m_data[1][0], matrix.m_data[1][1], matrix.m_data[1][3])),
                                    &Vector3::new((matrix.m_data[2][0], matrix.m_data[2][1], matrix.m_data[2][3])),
                                    &Vector3::new((matrix.m_data[3][0], matrix.m_data[3][1], matrix.m_data[3][3])) ));

        let m3 = Matrix3::new((&Vector3::new((matrix.m_data[1][0], matrix.m_data[1][1], matrix.m_data[1][2])),
                                    &Vector3::new((matrix.m_data[2][0], matrix.m_data[2][1], matrix.m_data[2][2])),
                                    &Vector3::new((matrix.m_data[3][0], matrix.m_data[3][1], matrix.m_data[3][2])) ));

        return matrix.m_data[0][0] * Matrix3::det(&m0)
            - matrix.m_data[0][1] * Matrix3::det(&m1)
            + matrix.m_data[0][2] * Matrix3::det(&m2)
            - matrix.m_data[0][3] * Matrix3::det(&m3);
    }

}

impl Instantiator for () {
    fn into(self) -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Instantiator: Invoked () Identity overload!");
        }
        return Matrix4{m_data: [[1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [0.0, 0.0, 0.0, 1.0]]};
    }
}

impl Instantiator for i64 {
    fn into(self) -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Instantiator: Invoked (i64) Zero overload!");
        }
        return Matrix4{m_data: [[0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]]};
    }
}

impl Instantiator for &Matrix4 {
    fn into(self) -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Instantiator: Invoked (&Matrix4) overload!");
        }
        return Matrix4{m_data: self.m_data };
    }
}

impl Instantiator for (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
    fn into(self) -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Instantiator: Invoked (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) overload!");
        }
        return Matrix4{m_data: [[self.0, self.1, self.2, self.3],
                                [self.4, self.5, self.6, self.7],
                                [self.8, self.9, self.10, self.11],
                                [self.12, self.13, self.14, self.15]]};
    }
}

impl Instantiator for (&Vector4, &Vector4, &Vector4, &Vector4) {
    fn into(self) -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Instantiator: Invoked (&Vector4, &Vector4, &Vector4, &Vector4) overload!");
        }
        return Matrix4{m_data: [self.0.m_data,
                            self.1.m_data,
                            self.2.m_data,
                            self.3.m_data]};
    }
}

impl Rotator for (f32, f32, f32, f32) {
    fn into(self) -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Rotator: Invoked (f32, f32, f32, f32) overload!");
        }
        return Matrix4::s_rotate(self.0, self.1, self.2, self.3);
    }
}

impl Rotator for (&Vector3, f32) {
    fn into(self) -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Rotator: Invoked (&Vector3, f32) overload!");
        }
        return Matrix4::s_rotate(self.0.get_x(), self.0.get_y(), self.0.get_z(), self.1);
    }
}

impl Rotator for (&Vector4, f32) {
    fn into(self) -> Matrix4 {
        if DEBUG {
            println!("Matrix4::Rotator: Invoked (&Vector4, f32) overload!");
        }
        return Matrix4::s_rotate(self.0.get_x(), self.0.get_y(), self.0.get_z(), self.1);
    }
}
