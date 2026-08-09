use glam::{Mat4};

pub fn debug_matrix_print(matrix: [[f32; 4]; 4], acc: usize){
    println!("");
    for y in matrix{
        print!("[");
        for x in y{
            print!("{:08.*},", acc, x);
        }
        println!("]");
    }
}