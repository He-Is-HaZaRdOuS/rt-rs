use crate::test::*;

mod matrix3;
mod matrix4;
mod test;
mod vector3;
mod vector4;

/* toggles constructor messages */
const DEBUG: bool = true;

fn main() {
    trace_rays();
}

fn trace_rays() {
    println!("DEBUG: {}", DEBUG);
    test_vector();
    println!();
    test_matrix();
    println!();
    test_json_vector();
    println!();
    test_json_matrix();
    println!();
    test_image();
}
