use rand::random;
use image::{ImageBuffer, RgbImage};
use crate::vector4::*;
use crate::matrix4::*;
use crate::vector3::*;
use crate::matrix3::*;

pub fn test_vector() {
    println!("Vector3 and Vector4 tests: \n");

    /* Test Constructors */
    println!("Building Vectors... \n");
    let vec3_empty = Vector3::new(()); // empty tuple creates a Vec3(0, 0, 0)
    let _vec3_zero = Vector3::new(0); // any integer also creates an empty Vec3(0, 0, 0)
    let _vec3_zero = Vector3::zero(); // zero() also creates an empty Vec3(0, 0, 0)
    let vec3_3f32 = Vector3::new((3.0, 3.0, 3.0)); // a 3-tuple creates a Vec3(f32, f32, f32)
    let _vec3_vec3 = Vector3::new(&vec3_empty); // a Vec3 can create a Vec3
    let _vec3_vec4 = Vector3::new(&Vector4::new(())); // a Vec4 can also create a Vec3 but 4th component is discarded

    let vec4_empty = Vector4::new(()); // empty tuple creates a Vec4(0, 0, 0, 1)
    let _vec4_zero = Vector4::new(0); // any integer creates an empty Vec4(0, 0, 0, 0)
    let _vec4_zero = Vector4::zero(); // zero() also creates an empty Vec4(0, 0, 0, 0)
    let mut vec4_3f32 = Vector4::new((3.0, 3.0, 3.0)); // a 3-tuple creates a Vec4(f32, f32, f32, 1)
    let vec4_4f32 = Vector4::new((4.0, 4.0, 4.0, 4.0)); // a 4-tuple creates a Vec4(f32, f32, f32, f32)
    let _vec4_vec3 = Vector4::new(&vec3_3f32); // a Vec3 can create a Vec4(Vec3, 1)
    let _vec4_vec4 = Vector4::new(&vec4_empty); // a Vec4 can create a Vec4

    println!();

    println!("vec4_3f32: {:?}", vec4_3f32);
    println!("vec4_4f32: {:?}", vec4_4f32);

    println!();

    /* Test Associated Methods */
    println!("dot product of vec4_3f32 and vec4_4f32: {}", vec4_3f32.dot(&vec4_4f32));

    println!();

    println!("cross product of vec4_4f32 with itself: {:?}", vec4_3f32.cross(&vec4_4f32));
    println!();

    println!("vec4_3f32 before normalization: {:?} and its magnitude: {}", vec4_3f32, vec4_3f32.magnitude());
    vec4_3f32.normalize();
    println!("vec4_3f32 after normalization: {:?} and its magnitude: {}", vec4_3f32, vec4_3f32.magnitude());

}

pub fn test_matrix() {
    println!("Matrix3 and Matrix4 tests: \n");

    /* helper vector3s */
    println!("Building Vectors to assist in Matrix Creation... \n");
    let vec3_empty = Vector3::new(());
    let vec3_zero = Vector3::new(0);
    let vec3_3f32 = Vector3::new((3.0, 3.0, 3.0));

    let vec4_empty = Vector4::new(());
    let vec4_zero = Vector4::zero();
    let vec4_3f32 = Vector4::new((3.0, 3.0, 3.0));
    let vec4_4f32 = Vector4::new((4.0, 4.0, 4.0, 4.0));
    println!();

    /* Text Constructors */
    println!("Building Matrices... \n");
    let _mat3_identity = Matrix3::new(()); // an empty tuple creates an identity Matrix3
    let mat3_identity = Matrix3::identity(); // identity() also creates an identity Matrix3
    let _mat3_zero = Matrix3::new(0); // any integer creates a zero Matrix3
    let _mat3_zero = Matrix3::zero(); // zero() also creates a zero Matrix3
    let _mat3_mat3 = Matrix3::new(&mat3_identity); // this creates a Matrix3 from the provided Matrix3
    let _mat3_9f32 = Matrix3::new((1.0, 2.0, 3.0, 4.0, 5.0 , 6.0, 7.0, 8.0, 9.0)); // 9 f32's create a Matrix3
    let _mat3_3vec3 = Matrix3::new((&vec3_empty, &vec3_zero, &vec3_3f32)); // 3 Vec3's create a Matrix3

    let _mat4_identity = Matrix4::new(()); // an empty tuple creates an identity Matrix4
    let mat4_identity = Matrix4::identity(); // identity() also creates an identity Matrix4
    let _mat4_zero = Matrix4::new(0); // any integer creates a zero Matrix4
    let mat4_zero = Matrix4::zero(); // zero() also creates a zero Matrix4
    let _mat4_mat4 = Matrix4::new(&mat4_identity); // this creates a Matrix4 from the provided Matrix4
    let mat4_16f32 = Matrix4::new((1.0, 2.0, 3.0, 4.0, 5.0 , 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0)); // 16 f32's create a Matrix4
    let mat4_4vec4 = Matrix4::new((&vec4_empty, &vec4_zero, &vec4_3f32, &vec4_4f32)); // 4 Vec4's create a Matrix4

    println!();

    println!("mat4_identity: {:?}", mat4_identity);
    println!("mat4_zero: {:?}", mat4_zero);
    println!("mat4_16f32: {:?}", mat4_16f32);
    println!("mat4_4vec4: {:?}", mat4_4vec4);

    println!("\nTesting Static Transformation Constructors... \n");

    /* Test Static Constructors */
    let mat4_translate = Matrix4::translate(1.0, 2.0, 3.0);
    let mat4_scale = Matrix4::scale(4.0, 4.0, 4.0);
    let mat4_rotatex = Matrix4::rotate_x(90.0);
    let mat4_rotatey = Matrix4::rotate_y(90.0);
    let mat4_rotatez = Matrix4::rotate_z(90.0);
    let mat4_rotate_3f32 = Matrix4::rotate((1.0, 1.0, 1.0, 90.0));
    let mat4_rotate_vec3 = Matrix4::rotate((&vec3_3f32, 90.0));
    let mat4_rotate_vec4 = Matrix4::rotate((&vec4_4f32, 90.0));

    println!();

    println!("mat4_translate: {:?}", mat4_translate);
    println!("mat4_scale: {:?}", mat4_scale);
    println!("mat4_rotatex: {:?}", mat4_rotatex);
    println!("mat4_rotatey: {:?}", mat4_rotatey);
    println!("mat4_rotatez: {:?}", mat4_rotatez);
    println!("mat4_rotate_3f32: {:?}", mat4_rotate_3f32);
    println!("mat4_rotate_vec3: {:?}", mat4_rotate_vec3);
    println!("mat4_rotate_vec4: {:?}", mat4_rotate_vec4);

    println!();

    /* Test Associated Methods */
    println!("Testing Matrix4-Vector4 and Matrix4-Matrix4 Multiplication... \n");

    let mat4_16f32_m_vec4_4f32 = mat4_16f32.mult(&vec4_4f32);
    let mat4_16f32_m_mat4_scale = mat4_16f32.mult(&mat4_scale);

    println!("vec4_4f32_mat4_identity: {:?}", mat4_16f32_m_vec4_4f32);
    println!("mat4_16f32_mat4_scale: {:?}", mat4_16f32_m_mat4_scale);

    println!("\nTesting Matrix4 determinant... \n");

    let mat4_16f32 = Matrix4::new((4.0, 2.0, 5.0, 2.0,
                                               4.0 , 2.0, 7.0, 2.0,
                                               3.0, 6.0, 1.0, 2.0,
                                               2.0, 14.0, 5.0, 6.0));
    println!("mat4_16f32: {:?}", mat4_16f32);
    println!("det of mat4_16f32: {}", mat4_16f32.det());

}

pub fn test_json_vector() {
    println!("Testing creating Vector4 from its respective json file:");
    let vec4_json = Vector4::json("./res/vector.json");
    println!("{:?}", vec4_json);
}

pub fn test_json_matrix() {
    println!("Testing creating Matrix4 from its respective json file:");
    let mat4_json = Matrix4::json("./res/matrix.json");
    println!("{:?}", mat4_json);
}

pub fn test_image() {
    println!("Generating a 512x512 red-blue gradient image with randomized green value in range of 0-64... (fractal.png) \n");
    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut img: RgbImage = ImageBuffer::new(512, 512);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        let g: u8 = random::<u8>() / 4;
        *pixel = image::Rgb([r, g, b]);
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    img.save("fractal.png").unwrap();


}
