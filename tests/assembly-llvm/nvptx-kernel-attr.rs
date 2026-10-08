// --- LLM-generated --- //
// Checks that `#[nvptx(...)]` produces the corresponding PTX performance-tuning directives.

//@ add-minicore
//@ assembly-output: emit-asm
//@ compile-flags: --crate-type=rlib --target=nvptx64-nvidia-cuda -Ctarget-cpu=sm_90
//@ needs-llvm-components: nvptx
#![feature(no_core, lang_items, abi_ptx, abi_gpu_kernel, nvptx_ext)]
#![no_core]

extern crate minicore;
use minicore::*;

// CHECK-LABEL: .entry launch_bounds(
// CHECK-NEXT: .maxntid 256, 2, 1
// CHECK-NEXT: .minnctapersm 2
// CHECK-NEXT: .maxnreg 64
// CHECK-NEXT: .maxclusterrank 8
#[no_mangle]
#[nvptx(
    max_ctas_per_cluster(8),
    min_ctas_per_sm(2),
    max_registers(64),
    max_threads_per_cta(256, 2, 1)
)]
pub extern "ptx-kernel" fn launch_bounds() {}

// CHECK-LABEL: .entry exact(
// CHECK-NEXT: .reqntid 32, 4, 2
// CHECK-NEXT: .explicitcluster
// CHECK-NEXT: .reqnctapercluster 2, 1, 1
#[no_mangle]
#[nvptx(exact_threads_per_cta(32, 4, 2), exact_cluster_dim(2, 1, 1))]
pub extern "gpu-kernel" fn exact() {}
