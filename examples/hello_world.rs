use mesh;

// Normally you can just include mesh-global as a dependency
#[global_allocator]
pub static MESH: mesh::Mesh = mesh::Mesh;

fn main() {
    let mut vec = Vec::new();

    for i in 0..=100 {
        vec.push(i);
    }

    println!("{:?}", vec);
}
