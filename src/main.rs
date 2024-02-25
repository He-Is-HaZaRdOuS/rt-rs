use std::fs::File;
use std::io::Read;
use crate::matrix4::Matrix4;
use crate::test::*;

mod vector3;
mod vector4 ;
mod matrix3;
mod matrix4;
mod test;

/* toggles constructor messages */
const DEBUG: bool = false;

fn main() {

    println!("DEBUG: {}", DEBUG);
    test_vector();
    println!();
    test_matrix();
    println!();
    test_json_vector();
    println!();
    test_json_matrix();

}
