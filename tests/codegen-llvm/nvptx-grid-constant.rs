// --- LLM-generated --- //
// Checks that `#[nvptx(grid_constant)]` parameters are lowered to `byval` pointers tagged with
// `nvvm.grid_constant`.

//@ add-minicore
//@ compile-flags: --crate-type=rlib --target=nvptx64-nvidia-cuda -Copt-level=0
//@ needs-llvm-components: nvptx
#![feature(no_core, lang_items, abi_ptx, abi_gpu_kernel, nvptx_ext)]
#![no_core]

extern crate minicore;
use minicore::*;

#[repr(C)]
pub struct Params {
    a: u32,
    b: [f32; 4],
    c: u64,
}
impl Copy for Params {}

// CHECK: define ptx_kernel void @kernel(ptr byval([32 x i8]) align 8 "nvvm.grid_constant" %p, ptr %out, ptr align 4 %q)
#[no_mangle]
pub extern "ptx-kernel" fn kernel(#[nvptx(grid_constant)] p: &Params, out: *mut f32, q: &u32) {}

// CHECK: define ptx_kernel void @scalar(i32 %x, ptr byval([2 x i8]) align 2 "nvvm.grid_constant" %y)
#[no_mangle]
pub extern "gpu-kernel" fn scalar(x: u32, #[nvptx(grid_constant)] y: &u16) {}

// CHECK: define {{.*}}ptx_kernel void @{{.*}}generic{{.*}}(ptr byval([16 x i8]) align 8 "nvvm.grid_constant" %p)
pub extern "ptx-kernel" fn generic<T: Copy + Freeze>(#[nvptx(grid_constant)] p: &T) {}

pub fn instantiate() -> extern "ptx-kernel" fn(&(u64, u64)) {
    generic::<(u64, u64)>
}

// Zero-sized pointees cannot be `byval`, so the attribute has no effect.
// CHECK: define ptx_kernel void @zst(ptr %p)
#[no_mangle]
pub extern "ptx-kernel" fn zst(#[nvptx(grid_constant)] p: &()) {}
