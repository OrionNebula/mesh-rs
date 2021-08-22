#![no_std]

extern crate mesh_sys as ffi;
use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
};

// This constant equals _Alignof(max_align_t) and is platform-specific. It
// contains the _maximum_ alignment that the memory allocations returned by the
// C standard library memory allocation APIs (e.g. `malloc`) are guaranteed to
// have.
//
// The memory allocation APIs are required to return memory that can fit any
// object whose fundamental aligment is <= _Alignof(max_align_t).
//
// In C, there are no ZSTs, and the size of all types is a multiple of their
// alignment (size >= align). So for allocations with size <=
// _Alignof(max_align_t), the malloc-APIs return memory whose alignment is
// either the requested size if its a power-of-two, or the next smaller
// power-of-two.
#[cfg(all(any(
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "mipsel",
    target_arch = "powerpc"
)))]
#[allow(non_upper_case_globals)]
const alignof_max_align_t: usize = 8;
#[cfg(all(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "powerpc64",
    target_arch = "powerpc64le",
    target_arch = "mips64",
    target_arch = "s390x",
    target_arch = "sparc64"
)))]
#[allow(non_upper_case_globals)]
const alignof_max_align_t: usize = 16;

// Assumes a condition that always must hold.
macro_rules! assume {
    ($e:expr) => {
        debug_assert!($e);
        if !($e) {
            core::hint::unreachable_unchecked();
        }
    };
}

pub struct Mesh;

unsafe impl GlobalAlloc for Mesh {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        assume!(layout.size() != 0);

        let ptr = if layout.align() <= alignof_max_align_t && layout.align() < layout.size() {
            ffi::mesh_malloc(layout.size())
        } else {
            ffi::mesh_memalign(layout.align(), layout.size())
        };

        ptr as *mut u8
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        assume!(layout.size() != 0);

        if layout.align() <= alignof_max_align_t && layout.align() < layout.size() {
            ffi::mesh_calloc(1, layout.size()) as *mut u8
        } else {
            // calloc has no gaurenteed alignment (outside of alignof_max_align_t), so we have to zero manually
            let ptr = ffi::mesh_memalign(layout.align(), layout.size()) as *mut u8;
            core::ptr::write_bytes(ptr, 0, layout.size());

            ptr
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        assume!(!ptr.is_null());
        assume!(layout.size() != 0);

        ffi::mesh_sized_free(ptr as *mut c_void, layout.size())
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        assume!(layout.size() != 0);
        assume!(new_size != 0);

        if layout.align() <= alignof_max_align_t && layout.align() < layout.size() {
            ffi::mesh_realloc(ptr as *mut c_void, layout.size()) as *mut u8
        } else {
            let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());
            let new_ptr = self.alloc(new_layout);
            if !new_ptr.is_null() {
                core::ptr::copy_nonoverlapping(
                    ptr,
                    new_ptr,
                    core::cmp::min(layout.size(), new_size),
                );
                self.dealloc(ptr, layout);
            }

            new_ptr
        }
    }
}

pub unsafe fn usable_size<T>(ptr: *mut T) -> usize {
    ffi::mesh_malloc_usable_size(ptr as *mut c_void)
}
