#![no_std]

use libc::{ size_t, c_void };

#[link(name = "mesh", kind = "dylib")]
extern "C" {
    pub fn mesh_malloc(sz: size_t) -> *mut c_void;
    pub fn mesh_free(ptr: *mut c_void);
    pub fn mesh_sized_free(ptr: *mut c_void, sz: size_t);
    pub fn mesh_realloc(oldPtr: *mut c_void, newSize: size_t) -> *mut c_void;
    pub fn mesh_malloc_usable_size(ptr: *mut c_void) -> size_t;
    pub fn mesh_memalign(alignment: size_t, size: size_t) -> *mut c_void;
    pub fn mesh_calloc(count: size_t, size: size_t) -> *mut c_void;
}
