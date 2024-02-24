#![allow(dead_code)]
use crate::DEBUG;
use crate::vector3::Vector3;

#[derive(Debug)]
pub struct Vector4 {
    pub m_data: [f32; 4],
}

pub trait Instantiator {
    fn into(self) -> Vector4;
}

/* Associative methods implementation block */
impl Vector4 {
    /* Accessors */
    pub fn get_x(&self) -> f32 {
        return self.m_data[0];
    }

    pub fn get_y(&self) -> f32 {
        return self.m_data[1];
    }

    pub fn get_z(&self) -> f32 {
        return self.m_data[2];
    }

    pub fn get_w(&self) -> f32 {
        return self.m_data[3];
    }

    pub fn get_r(&self) -> f32 {
        return self.m_data[0];
    }

    pub fn get_g(&self) -> f32 {
        return self.m_data[1];
    }

    pub fn get_b(&self) -> f32 {
        return self.m_data[2];
    }

    pub fn get_a(&self) -> f32 {
        return self.m_data[3];
    }

    /* Mutators */
    pub fn set_x(&mut self, x: f32) {
        self.m_data[0] = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.m_data[1] = y;
    }

    pub fn set_z(&mut self, z: f32) {
        self.m_data[2] = z;
    }

    pub fn set_w(&mut self, w: f32) {
        self.m_data[3] = w;
    }

    pub fn set_r(&mut self, r: f32) {
        self.m_data[0] = r;
    }

    pub fn set_g(&mut self, g: f32) {
        self.m_data[1] = g;
    }

    pub fn set_b(&mut self, b: f32) {
        self.m_data[2] = b;
    }

    pub fn set_a(&mut self, a: f32) {
        self.m_data[3] = a;
    }

    pub fn magnitude(&self) -> f32 {
        return (self.get_x().powf(2.0) + self.get_y().powf(2.0) + self.get_z().powf(2.0)).sqrt()
    }

    pub fn dot(&self, v: &Vector4) -> f32 {
        return (self.get_x() * v.get_x()) + (self.get_y() * v.get_y()) + (self.get_z() * v.get_z()) + (self.get_w() * v.get_w());
    }

    pub fn cross(&self, v: &Vector4) -> Vector4 {
        return Vector4::new(( (self.get_y() * v.get_z()) - (self.get_z() * v.get_y()), //x
                                (self.get_z() * v.get_x()) - (self.get_x() * v.get_z()), //y
                              (self.get_x() * v.get_y()) - (self.get_y() * v.get_x()), //z
                            self.get_w())); //w
    }

    pub fn normalize(&mut self) {
        let mag: f32 = 1.0 / self.magnitude();
        self.set_x(self.get_x() * mag);
        self.set_y(self.get_y() * mag);
        self.set_z(self.get_z() * mag);
        self.set_w(self.get_w() * mag);
    }
}

/* Associative functions implementation block */
impl Vector4 {
    /* Constructor */
    pub fn new<A> (args: A) -> Vector4
        where A: Instantiator
    {
        return args.into();
    }

    /* Static Constructors */
    pub fn zero() -> Vector4 {
        return Vector4::new(0i64);
    }
}

impl Instantiator for () {
    fn into(self) -> Vector4 {
        if DEBUG {
            println!("Vector4::Instantiator: Invoked () overload!");
        }
        return Vector4{m_data: [0.0, 0.0, 0.0, 1.0]};
    }
}

impl Instantiator for i64 {
    fn into(self) -> Vector4 {
        if DEBUG {
            println!("Vector4::Instantiator: Invoked (i64) Zero overload!");
        }
        return Vector4{m_data: [0.0, 0.0, 0.0, 0.0]};
    }
}

impl Instantiator for (f32, f32, f32) {
    fn into(self) -> Vector4 {
        if DEBUG  {
            println!("Vector4::Instantiator: Invoked (f32, f32, f32) overload!");
        }
        return Vector4{m_data: [self.0, self.1, self.2, 1.0]};
    }
}

impl Instantiator for (f32, f32, f32, f32) {
    fn into(self) -> Vector4 {
        if DEBUG {
            println!("Vector4::Instantiator: Invoked (f32, f32, f32, f32) overload!");
        }
        return Vector4{m_data: [self.0, self.1, self.2, self.3]};
    }
}

impl Instantiator for &Vector3 {
    fn into(self) -> Vector4 {
        if DEBUG {
            println!("Vector4::Instantiator: Invoked (&Vector3) overload!");
        }
        return Vector4{m_data: {[self.m_data[0], self.m_data[1], self.m_data[2], 1.0]}};
    }
}

impl Instantiator for &Vector4 {
    fn into(self) -> Vector4 {
        if DEBUG {
            println!("Vector4::Instantiator: Invoked (&Vector4) overload!");
        }
        return Vector4{m_data: {self.m_data}};
    }
}
