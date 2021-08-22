#[cfg(any(feature = "force_global_mesh", target_os = "linux", target_os = "macos"))]
#[global_allocator]
pub static MESH: mesh::Mesh = mesh::Mesh;
