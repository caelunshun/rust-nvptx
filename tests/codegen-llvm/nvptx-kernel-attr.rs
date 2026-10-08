// --- LLM-generated --- //
// Checks that `#[nvptx(...)]` is lowered to the corresponding `nvvm.*` function attributes.

//@ add-minicore
//@ compile-flags: --crate-type=rlib --target=nvptx64-nvidia-cuda -Copt-level=0
//@ needs-llvm-components: nvptx
#![feature(no_core, lang_items, abi_ptx, abi_gpu_kernel, nvptx_ext)]
#![no_core]

extern crate minicore;
use minicore::*;

// CHECK: define ptx_kernel void @all_scalars() unnamed_addr #[[SCALARS:[0-9]+]]
#[no_mangle]
#[nvptx(max_ctas_per_cluster(8), min_ctas_per_sm(2), max_registers(64))]
pub extern "ptx-kernel" fn all_scalars() {}

// CHECK: define ptx_kernel void @max_dims() unnamed_addr #[[MAX_DIMS:[0-9]+]]
#[no_mangle]
#[nvptx(max_threads_per_cta(256, 2, 1))]
pub extern "gpu-kernel" fn max_dims() {}

// CHECK: define ptx_kernel void @exact_dims() unnamed_addr #[[EXACT_DIMS:[0-9]+]]
#[no_mangle]
#[nvptx(exact_threads_per_cta(32, 4, 2), exact_cluster_dim(2, 1, 1))]
pub extern "ptx-kernel" fn exact_dims() {}

// CHECK: define ptx_kernel void @no_attr() unnamed_addr #[[NO_ATTR:[0-9]+]]
#[no_mangle]
pub extern "ptx-kernel" fn no_attr() {}

// CHECK: attributes #[[SCALARS]] = {{.*}}"nvvm.maxclusterrank"="8" "nvvm.maxnreg"="64" "nvvm.minctasm"="2"
// CHECK: attributes #[[MAX_DIMS]] = {{.*}}"nvvm.maxntid"="256,2,1"
// CHECK-NOT: nvvm.reqntid
// CHECK-NOT: nvvm.cluster_dim
// CHECK: attributes #[[EXACT_DIMS]] = {{.*}}"nvvm.cluster_dim"="2,1,1" "nvvm.reqntid"="32,4,2"
// CHECK-NOT: nvvm.
// CHECK: attributes #[[NO_ATTR]]
// CHECK-NOT: nvvm.
